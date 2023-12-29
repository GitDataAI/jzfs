package versionmgr

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"time"

	"github.com/jiaozifs/jiaozifs/utils"

	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/block"
	"github.com/jiaozifs/jiaozifs/block/factory"
	"github.com/jiaozifs/jiaozifs/block/params"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/jiaozifs/jiaozifs/utils/httputil"
	"github.com/jiaozifs/jiaozifs/utils/pathutil"
)

var workRepoLog = logging.Logger("work_repo")

type WorkRepoState string

const (
	InWip    WorkRepoState = "wip"
	InBranch WorkRepoState = "branch"
	InCommit WorkRepoState = "commit"
)

type WorkRepository struct {
	operator  *models.User
	repoModel *models.Repository
	adapter   block.Adapter
	repo      models.IRepo
	state     WorkRepoState
	//cache
	headTree *hash.Hash
	wip      *models.WorkingInProcess
	branch   *models.Branch
	commit   *models.Commit
}

func NewWorkRepositoryFromConfig(ctx context.Context, operator *models.User, repoModel *models.Repository, repo models.IRepo, publicAdapterConfig params.AdapterConfig) (*WorkRepository, error) {
	var adapter block.Adapter
	var err error
	if repoModel.UsePublicStorage {
		adapter, err = factory.BuildBlockAdapter(ctx, publicAdapterConfig)
	} else {
		adapter, err = AdapterFromConfig(ctx, *repoModel.StorageAdapterParams)
	}
	if err != nil {
		return nil, err
	}
	return NewWorkRepositoryFromAdapter(ctx, operator, repoModel, repo, adapter), nil
}

func NewWorkRepositoryFromAdapter(_ context.Context, operator *models.User, repoModel *models.Repository, repo models.IRepo, adapter block.Adapter) *WorkRepository {
	return &WorkRepository{
		operator:  operator,
		repoModel: repoModel,
		repo:      repo, adapter: adapter,
	}
}

// WriteBlob write blob content to storage
func (repository *WorkRepository) WriteBlob(ctx context.Context, body io.Reader, contentLength int64, properties models.Property) (*models.Blob, error) {
	// handle the upload itself
	hashReader := hash.NewHashingReader(body, hash.Md5)
	tempf, err := os.CreateTemp("", "*")
	if err != nil {
		return nil, err
	}
	_, err = io.Copy(tempf, hashReader)
	if err != nil {
		return nil, err
	}

	checkSum := hash.Hash(hashReader.Md5.Sum(nil))
	_, err = tempf.Seek(0, io.SeekStart)
	if err != nil {
		return nil, err
	}

	defer func() {
		name := tempf.Name()
		_ = tempf.Close()
		_ = os.RemoveAll(name)
	}()

	address := pathutil.PathOfHash(checkSum)
	err = repository.adapter.Put(ctx, block.ObjectPointer{
		StorageNamespace: utils.StringValue(repository.repoModel.StorageNamespace),
		IdentifierType:   block.IdentifierTypeRelative,
		Identifier:       address,
	}, contentLength, tempf, block.PutOpts{})
	if err != nil {
		return nil, err
	}

	return models.NewBlob(properties, repository.repoModel.ID, checkSum, hashReader.CopiedSize)
}

// ReadBlob read blob content with range
func (repository *WorkRepository) ReadBlob(ctx context.Context, blob *models.Blob, rangeSpec *string) (io.ReadCloser, error) {
	address := pathutil.PathOfHash(blob.CheckSum)
	pointer := block.ObjectPointer{
		StorageNamespace: utils.StringValue(repository.repoModel.StorageNamespace),
		IdentifierType:   block.IdentifierTypeRelative,
		Identifier:       address,
	}

	// setup response
	var reader io.ReadCloser
	// handle partial response if byte range supplied
	if rangeSpec != nil {
		rng, err := httputil.ParseRange(*rangeSpec, blob.Size)
		if err != nil {
			return nil, err
		}
		reader, err = repository.adapter.GetRange(ctx, pointer, rng.StartOffset, rng.EndOffset)
		if err != nil {
			return nil, err
		}

	} else {
		var err error
		reader, err = repository.adapter.Get(ctx, pointer, blob.Size)
		if err != nil {
			return nil, err
		}
	}
	return reader, nil
}

// RootTree return worktree at root
func (repository *WorkRepository) RootTree(ctx context.Context) (*WorkTree, error) {
	if repository.headTree == nil {
		//use repo default headTree
		ref, err := repository.repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.repoModel.ID).SetName(repository.repoModel.HEAD))
		if err != nil {
			return nil, err
		}

		repository.branch = ref
		treeHash := hash.EmptyHash
		var commit *models.Commit
		if !ref.CommitHash.IsEmpty() {
			commit, err = repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, ref.CommitHash)
			if err != nil {
				return nil, err
			}
			treeHash = commit.TreeHash
		}
		repository.setCurState(InBranch, nil, ref, commit)
		repository.headTree = &treeHash
	}
	return NewWorkTree(ctx, repository.repo.FileTreeRepo(repository.repoModel.ID), models.NewRootTreeEntry(*repository.headTree))
}

func (repository *WorkRepository) CheckOut(ctx context.Context, refType WorkRepoState, refName string) error {
	treeHash := hash.EmptyHash
	if refType == InWip {
		ref, err := repository.repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.repoModel.ID).SetName(refName))
		if err != nil {
			return fmt.Errorf("unable to get branch %s of repository %s: %w", refName, repository.repoModel.Name, err)
		}
		wip, err := repository.repo.WipRepo().Get(ctx, models.NewGetWipParams().SetCreatorID(repository.operator.ID).SetRepositoryID(repository.repoModel.ID).SetRefID(ref.ID))
		if err != nil {
			return fmt.Errorf("unable to get wip of repository %s branch %s: %w", repository.repoModel.Name, refName, err)
		}
		treeHash = wip.CurrentTree
		repository.setCurState(InWip, wip, ref, nil)
	} else if refType == InBranch {
		branch, err := repository.repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.repoModel.ID).SetName(refName))
		if err != nil {
			return err
		}
		var commit *models.Commit
		if !branch.CommitHash.IsEmpty() {
			commit, err = repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, branch.CommitHash)
			if err != nil {
				return err
			}
			treeHash = commit.TreeHash
		}
		repository.setCurState(InBranch, nil, branch, commit)
	} else if refType == InCommit {
		commitHash, err := hash.FromHex(refName)
		if err != nil {
			return err
		}

		if !commitHash.IsEmpty() {
			commit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, commitHash)
			if err != nil {
				return err
			}
			treeHash = commit.TreeHash
			repository.setCurState(InCommit, nil, nil, commit)
		}
	} else {
		return fmt.Errorf("not support type")
	}
	repository.headTree = &treeHash
	return nil
}

func (repository *WorkRepository) RevertWip(ctx context.Context) error {
	if repository.state != InWip {
		return fmt.Errorf("working repo not in wip state")
	}
	treeHash := hash.EmptyHash
	if !repository.wip.BaseCommit.IsEmpty() {
		commit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, repository.wip.BaseCommit)
		if err != nil {
			return err
		}
		treeHash = commit.TreeHash
	}
	err := repository.repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(repository.wip.ID).SetCurrentTree(treeHash))
	if err != nil {
		return err
	}
	repository.wip.CurrentTree = treeHash
	return nil
}

// DeleteWip remove wip  todo remove files
func (repository *WorkRepository) DeleteWip(ctx context.Context) error {
	if repository.state != InBranch {
		return fmt.Errorf("working repo not in branch state")
	}

	deleteParams := models.NewDeleteWipParams().SetRefID(repository.branch.ID).SetRepositoryID(repository.repoModel.ID).SetCreatorID(repository.operator.ID)
	affectRow, err := repository.repo.WipRepo().Delete(ctx, deleteParams)
	if err != nil {
		return err
	}
	if affectRow == 0 {
		return models.ErrNotFound
	}
	return nil
}

// CommitChanges append a new commit to current headTree, read changes from wip, than create a new commit with parent point to current headTree,
// and replace tree hash with wip's currentTreeHash.
func (repository *WorkRepository) CommitChanges(ctx context.Context, msg string) (*models.Commit, error) {
	if !(repository.state == InWip || repository.state == InBranch) {
		return nil, errors.New("must commit changes on branch")
	}

	head := repository.headTree
	creator, err := repository.repo.UserRepo().Get(ctx, models.NewGetUserParams().SetID(repository.wip.CreatorID))
	if err != nil {
		return nil, err
	}

	if !bytes.Equal(repository.branch.CommitHash, repository.wip.BaseCommit) {
		return nil, fmt.Errorf("base commit not equal with branch, please update wip")
	}

	parentHash := []hash.Hash{}
	if !repository.branch.CommitHash.IsEmpty() {
		parentHash = []hash.Hash{repository.branch.CommitHash}
	}

	commit := &models.Commit{
		Hash:         nil,
		RepositoryID: repository.repoModel.ID,
		Author: models.Signature{
			Name:  creator.Name,
			Email: creator.Email,
			When:  repository.wip.UpdatedAt,
		},
		Committer: models.Signature{
			Name:  repository.operator.Name,
			Email: repository.operator.Email,
			When:  time.Now(),
		},
		MergeTag:     "",
		Message:      msg,
		TreeHash:     repository.wip.CurrentTree,
		ParentHashes: parentHash,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	commitHash, err := commit.GetHash()
	if err != nil {
		return nil, err
	}
	commit.Hash = commitHash
	err = repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		_, err = repo.CommitRepo(repository.repoModel.ID).Insert(ctx, commit)
		if err != nil {
			return err
		}

		err = repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(repository.wip.ID).SetBaseCommit(commitHash))
		if err != nil {
			return err
		}

		head = &repository.wip.CurrentTree
		return repo.BranchRepo().UpdateByID(ctx, models.NewUpdateBranchParams(repository.branch.ID).SetCommitHash(commitHash))
	})
	if err != nil {
		return nil, err
	}

	repository.branch.CommitHash = commitHash
	repository.wip.BaseCommit = commitHash
	repository.headTree = head
	return commit, err
}

// CreateBranch create branch base on current head
func (repository *WorkRepository) CreateBranch(ctx context.Context, branchName string) (*models.Branch, error) {
	//check exit
	_, err := repository.repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetName(branchName).SetRepositoryID(repository.repoModel.ID))
	if err == nil {
		return nil, fmt.Errorf("%s already exit", branchName)
	}
	if err != nil && !errors.Is(err, models.ErrNotFound) {
		return nil, err
	}

	commitHash := hash.EmptyHash
	if repository.commit != nil {
		commitHash = repository.commit.Hash
	}
	// Create branch
	newBranch := &models.Branch{
		RepositoryID: repository.repoModel.ID,
		CommitHash:   commitHash,
		Name:         branchName,
		CreatorID:    repository.operator.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	return repository.repo.BranchRepo().Insert(ctx, newBranch)
}

// DeleteBranch delete branch also delete wip belong this branch
func (repository *WorkRepository) DeleteBranch(ctx context.Context) error {
	return repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		deleteBranchParams := models.NewDeleteBranchParams().
			SetRepositoryID(repository.repoModel.ID).
			SetName(repository.branch.Name)
		_, err := repo.BranchRepo().Delete(ctx, deleteBranchParams)
		if err != nil {
			return err
		}

		deleteWipParams := models.NewDeleteWipParams().SetRepositoryID(repository.repoModel.ID).SetRefID(repository.branch.ID)
		_, err = repo.WipRepo().Delete(ctx, deleteWipParams)
		return err
	})
}

func (repository *WorkRepository) GetOrCreateWip(ctx context.Context) (*models.WorkingInProcess, bool, error) {
	if repository.state != InBranch {
		return nil, false, fmt.Errorf("only create wip from branch")
	}

	wip, err := repository.repo.WipRepo().Get(ctx, models.NewGetWipParams().SetRefID(repository.branch.ID).SetCreatorID(repository.operator.ID).SetRepositoryID(repository.repoModel.ID))
	if err == nil {
		repository.headTree = &wip.CurrentTree
		return wip, false, nil
	}

	if err != nil && !errors.Is(err, models.ErrNotFound) {
		return nil, false, err
	}

	// if not found create a wip
	currentTreeHash := hash.EmptyHash
	if !repository.branch.CommitHash.IsEmpty() {
		baseCommit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, repository.branch.CommitHash)
		if err != nil {
			return nil, false, err
		}
		currentTreeHash = baseCommit.TreeHash
	}

	wip = &models.WorkingInProcess{
		CurrentTree:  currentTreeHash,
		BaseCommit:   repository.branch.CommitHash,
		RepositoryID: repository.repoModel.ID,
		RefID:        repository.branch.ID,
		State:        0,
		CreatorID:    repository.operator.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	wip, err = repository.repo.WipRepo().Insert(ctx, wip)
	if err != nil {
		return nil, false, err
	}
	repository.headTree = &wip.CurrentTree
	repository.setCurState(InWip, wip, repository.branch, nil)
	return wip, true, nil
}

// DiffCommit find file changes in two commit
func (repository *WorkRepository) DiffCommit(ctx context.Context, toCommitID hash.Hash) (*Changes, error) {
	workTree, err := repository.RootTree(ctx)
	if err != nil {
		return nil, err
	}
	toCommit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, toCommitID)
	if err != nil {
		return nil, err
	}

	return workTree.Diff(ctx, toCommit.TreeHash)
}

func (repository *WorkRepository) GetCommitChanges(ctx context.Context) (*Changes, error) {
	if len(repository.commit.ParentHashes) == 0 {
		workTree, err := repository.RootTree(ctx)
		if err != nil {
			return nil, err
		}
		return workTree.Diff(ctx, hash.EmptyHash)
	} else if len(repository.commit.ParentHashes) == 1 {
		return repository.DiffCommit(ctx, repository.commit.ParentHashes[0])
	}
	return repository.DiffCommit(ctx, repository.commit.ParentHashes[1])
}

// Merge implement merge like git, docs https://en.wikipedia.org/wiki/Merge_(version_control)
func (repository *WorkRepository) Merge(ctx context.Context, merger *models.User, toMergeCommitHash hash.Hash, msg string, resolver ConflictResolver) (*models.Commit, error) {
	if repository.state != InBranch {
		return nil, errors.New("must merge on branch")
	}
	var commit *models.Commit
	var err error
	if !repository.branch.CommitHash.IsEmpty() {
		commit, err = repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, repository.branch.CommitHash)
		if err != nil {
			return nil, err
		}
	}

	newCommit, err := merge(ctx,
		repository.repo.CommitRepo(repository.repoModel.ID),
		repository.repo.FileTreeRepo(repository.repoModel.ID),
		merger,
		commit,
		repository.repoModel,
		toMergeCommitHash,
		msg,
		resolver)
	if err != nil {
		return nil, err
	}

	updateParams := models.NewUpdateBranchParams(repository.branch.ID).SetCommitHash(newCommit.Hash)
	err = repository.repo.BranchRepo().UpdateByID(ctx, updateParams)
	if err != nil {
		return nil, err
	}
	return newCommit, nil
}

func (repository *WorkRepository) setCurState(state WorkRepoState, wip *models.WorkingInProcess, branch *models.Branch, commit *models.Commit) {
	repository.state = state
	repository.wip = wip
	repository.branch = branch
	repository.commit = commit
}

func (repository *WorkRepository) CurWip() *models.WorkingInProcess {
	return repository.wip
}

func (repository *WorkRepository) CurBranch() *models.Branch {
	return repository.branch
}

func (repository *WorkRepository) Reset() {
	repository.headTree = nil
	repository.setCurState("", nil, nil, nil)
}

func merge(ctx context.Context,
	commitRepo models.ICommitRepo,
	fileTreeRepo models.IFileTreeRepo,
	merger *models.User,
	baseCommit *models.Commit,
	repoModel *models.Repository,
	toMergeCommitHash hash.Hash, msg string, resolver ConflictResolver) (*models.Commit, error) {
	toMergeCommit, err := commitRepo.Commit(ctx, toMergeCommitHash)
	if err != nil {
		return nil, err
	}

	//find accessor
	baseCommitNode := NewWrapCommitNode(commitRepo, baseCommit)
	toMergeCommitNode := NewWrapCommitNode(commitRepo, toMergeCommit)

	{
		//do nothing while merge is ancestor of base
		mergeIsAncestorOfBase, err := toMergeCommitNode.IsAncestor(ctx, baseCommitNode)
		if err != nil {
			return nil, err
		}

		if mergeIsAncestorOfBase {
			workRepoLog.Warnf("merge commit %s is ancestor of base commit %s", toMergeCommitHash, baseCommit.Hash)
			return baseCommit, nil
		}
	}

	{
		//try fast-forward merge no need to create new commit node
		baseIsAncestorOfMerge, err := baseCommitNode.IsAncestor(ctx, toMergeCommitNode)
		if err != nil {
			return nil, err
		}

		if baseIsAncestorOfMerge {
			workRepoLog.Warnf("base commit %s is ancestor of merge commit %s", toMergeCommitHash, baseCommit.Hash)
			return toMergeCommit, nil
		}
	}

	// three-way merge
	bestAncestor, err := baseCommitNode.MergeBase(ctx, toMergeCommitNode)
	if err != nil {
		return nil, err
	}

	if len(bestAncestor) == 0 {
		return nil, fmt.Errorf("no common ancesstor find")
	}

	bestCommit := bestAncestor[0]
	if len(bestAncestor) > 1 {
		//merge cross merge create virtual commit
		virtualCommit, err := merge(ctx, commitRepo, fileTreeRepo, merger, bestAncestor[0].Commit(), repoModel, bestAncestor[1].Commit().Hash, "virtual commit", resolver)
		if err != nil {
			return nil, err
		}

		bestCommit = NewWrapCommitNode(commitRepo, virtualCommit)
	}

	ancestorWorkTree, err := NewWorkTree(ctx, fileTreeRepo, models.NewRootTreeEntry(bestAncestor[0].TreeHash()))
	if err != nil {
		return nil, err
	}

	baseDiff, err := ancestorWorkTree.Diff(ctx, baseCommit.TreeHash)
	if err != nil {
		return nil, err
	}

	mergeDiff, err := ancestorWorkTree.Diff(ctx, toMergeCommit.TreeHash)
	if err != nil {
		return nil, err
	}

	//merge diff
	baseWorkTree, err := NewWorkTree(ctx, fileTreeRepo, models.NewRootTreeEntry(bestCommit.Commit().TreeHash))
	if err != nil {
		return nil, err
	}

	cmw := NewChangesMergeIter(baseDiff, mergeDiff, resolver)
	for cmw.Has() {
		change, err := cmw.Next()
		if err != nil {
			return nil, err
		}
		//apply change
		err = baseWorkTree.ApplyOneChange(ctx, change)
		if err != nil {
			return nil, err
		}
	}

	author := models.Signature{
		Name:  merger.Name,
		Email: merger.Email,
		When:  time.Now(),
	}

	mergeCommit := &models.Commit{
		Author:       author,
		RepositoryID: repoModel.ID,
		Committer:    author,
		MergeTag:     "",
		Message:      msg,
		TreeHash:     baseWorkTree.Root().Hash(),
		ParentHashes: []hash.Hash{baseCommit.Hash, toMergeCommitHash},
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	hash, err := mergeCommit.GetHash()
	if err != nil {
		return nil, err
	}
	mergeCommit.Hash = hash

	mergeCommitResult, err := commitRepo.Insert(ctx, mergeCommit)
	if err != nil {
		return nil, err
	}
	return mergeCommitResult, nil
}
