package versionmgr

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"path"
	"time"

	"github.com/GitDataAI/jiaozifs/versionmgr/merkletrie"

	"github.com/GitDataAI/jiaozifs/utils"

	"github.com/GitDataAI/jiaozifs/block"
	"github.com/GitDataAI/jiaozifs/block/factory"
	"github.com/GitDataAI/jiaozifs/block/params"
	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/utils/hash"
	"github.com/GitDataAI/jiaozifs/utils/httputil"
	"github.com/GitDataAI/jiaozifs/utils/pathutil"
	logging "github.com/ipfs/go-log/v2"
)

var workRepoLog = logging.Logger("work_repo")

type WorkRepoState string

const (
	InWip    WorkRepoState = "wip"
	InBranch WorkRepoState = "branch"
	InCommit WorkRepoState = "commit"
	InTag    WorkRepoState = "tag"
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
	tag      *models.Tag
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
	return repository.rootTree(ctx, repository.repo)
}

func (repository *WorkRepository) rootTree(ctx context.Context, repo models.IRepo) (*WorkTree, error) {
	if repository.headTree == nil {
		//use repo default headTree
		ref, err := repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.repoModel.ID).SetName(repository.repoModel.HEAD))
		if err != nil {
			return nil, err
		}

		repository.branch = ref
		treeHash := hash.Empty
		var commit *models.Commit
		if !ref.CommitHash.IsEmpty() {
			commit, err = repo.CommitRepo(repository.repoModel.ID).Commit(ctx, ref.CommitHash)
			if err != nil {
				return nil, err
			}
			treeHash = commit.TreeHash
		}
		repository.setCurState(InBranch, nil, ref, nil, commit)
		repository.headTree = &treeHash
	}
	return NewWorkTree(ctx, repo.FileTreeRepo(repository.repoModel.ID), models.NewRootTreeEntry(*repository.headTree))
}

func (repository *WorkRepository) CheckOut(ctx context.Context, refType WorkRepoState, refName string) error {
	treeHash := hash.Empty
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
		repository.setCurState(InWip, wip, ref, nil, nil)
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
		repository.setCurState(InBranch, nil, branch, nil, commit)
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
			repository.setCurState(InCommit, nil, nil, nil, commit)
		}
	} else if refType == InTag {
		tag, err := repository.repo.TagRepo().Get(ctx, models.NewGetTagParams().SetRepositoryID(repository.repoModel.ID).SetName(refName))
		if err != nil {
			return err
		}
		commit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, tag.Target)
		if err != nil {
			return err
		}
		treeHash = commit.TreeHash
		repository.setCurState(InTag, nil, nil, tag, commit)
	} else {
		return fmt.Errorf("not support type")
	}
	repository.headTree = &treeHash
	return nil
}

// Revert changes in wip, not a good algo, but maybe enough
func (repository *WorkRepository) Revert(ctx context.Context, prefixPath string) error {
	if repository.state != InWip {
		return fmt.Errorf("working repo not in wip state")
	}

	baseTreeHash := hash.Empty
	if !repository.wip.BaseCommit.IsEmpty() {
		commit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, repository.wip.BaseCommit)
		if err != nil {
			return err
		}
		baseTreeHash = commit.TreeHash
	}

	prefixPath = CleanPath(prefixPath)
	if len(prefixPath) == 0 {
		//just revert all, in fact this strategy can apply to all path , but not a easy work
		err := repository.repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(repository.wip.ID).SetCurrentTree(baseTreeHash))
		if err != nil {
			return err
		}
		repository.wip.CurrentTree = baseTreeHash
		return nil
	}

	err := repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		baseTree, err := NewWorkTree(ctx, repo.FileTreeRepo(repository.repoModel.ID), models.NewRootTreeEntry(baseTreeHash))
		if err != nil {
			return err
		}
		curTree, err := NewWorkTree(ctx, repo.FileTreeRepo(repository.repoModel.ID), models.NewRootTreeEntry(repository.wip.CurrentTree))
		if err != nil {
			return err
		}

		changes, err := baseTree.Diff(ctx, repository.wip.CurrentTree, prefixPath)
		if err != nil {
			return err
		}
		if changes.Num() == 0 {
			return models.ErrNotFound
		}

		err = changes.ForEach(func(change IChange) error {
			action, err := change.Action()
			if err != nil {
				return err
			}
			changePath := change.Path()
			switch action {
			case merkletrie.Insert:
				err = curTree.RemoveEntry(ctx, changePath)
				if err != nil {
					return err
				}
			case merkletrie.Modify:
				blob, _, err := baseTree.FindBlob(ctx, changePath)
				if err != nil {
					return err
				}

				err = curTree.ReplaceLeaf(ctx, changePath, blob)
				if err != nil {
					return err
				}
			case merkletrie.Delete:
				blob, _, err := baseTree.FindBlob(ctx, changePath)
				if err != nil {
					return err
				}

				err = curTree.AddLeaf(ctx, changePath, blob)
				if err != nil {
					return err
				}
			}
			return nil
		})
		if err != nil {
			return err
		}

		err = repository.repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(repository.wip.ID).SetCurrentTree(curTree.Root().Hash()))
		if err != nil {
			return err
		}
		repository.wip.CurrentTree = curTree.Root().Hash()
		return nil
	})
	return err
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
	if !(repository.state == InWip) {
		return nil, errors.New("must commit changes on branch")
	}

	if !bytes.Equal(repository.branch.CommitHash, repository.wip.BaseCommit) {
		return nil, fmt.Errorf("base commit not equal with branch, please update wip")
	}

	creator, err := repository.repo.UserRepo().Get(ctx, models.NewGetUserParams().SetID(repository.wip.CreatorID))
	if err != nil {
		return nil, err
	}

	author := models.Signature{
		Name:  creator.Name,
		Email: creator.Email,
		When:  repository.wip.UpdatedAt,
	}

	var commit *models.Commit
	err = repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		var err error
		commit, err = repository.commitChangeRoot(ctx, repo, author, repository.wip.CurrentTree, msg)
		if err != nil {
			return err
		}

		return repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(repository.wip.ID).SetBaseCommit(commit.Hash))
	})
	if err != nil {
		return nil, err
	}

	repository.branch.CommitHash = commit.Hash
	repository.wip.BaseCommit = commit.Hash
	repository.headTree = &repository.wip.CurrentTree
	return commit, err
}

// ChangeInWip apply change to wip
func (repository *WorkRepository) ChangeInWip(ctx context.Context, changFn func(root *WorkTree) error) error {
	return repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		workTree, err := repository.changeInWip(ctx, repo, changFn)
		if err != nil {
			return err
		}

		repository.wip.CurrentTree = workTree.Root().Hash()
		repository.headTree = &repository.wip.CurrentTree
		return repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(repository.wip.ID).SetCurrentTree(workTree.Root().Hash()))
	})
}

// ChangeAndCommit apply changes to tree, and create a new commit
func (repository *WorkRepository) ChangeAndCommit(ctx context.Context, msg string, changFn func(root *WorkTree) error) (*models.Commit, error) {
	if !bytes.Equal(repository.branch.CommitHash, repository.wip.BaseCommit) {
		return nil, fmt.Errorf("base commit not equal with branch, please update wip")
	}

	creator, err := repository.repo.UserRepo().Get(ctx, models.NewGetUserParams().SetID(repository.wip.CreatorID))
	if err != nil {
		return nil, err
	}

	author := models.Signature{
		Name:  creator.Name,
		Email: creator.Email,
		When:  repository.wip.UpdatedAt,
	}

	var commit *models.Commit
	err = repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		workTree, err := repository.changeInWip(ctx, repo, changFn)
		if err != nil {
			return err
		}
		commit, err = repository.commitChangeRoot(ctx, repo, author, workTree.Root().Hash(), msg)
		if err != nil {
			return err
		}

		err = repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(repository.wip.ID).SetCurrentTree(workTree.Root().Hash()).SetBaseCommit(commit.Hash))
		if err != nil {
			return err
		}
		repository.wip.CurrentTree = workTree.Root().Hash()
		repository.headTree = &repository.wip.CurrentTree
		return err
	})
	if err != nil {
		return nil, err
	}
	repository.branch.CommitHash = commit.Hash
	repository.wip.BaseCommit = commit.Hash
	//dont set commit here, wip possibly wip changes
	return commit, err
}

func (repository *WorkRepository) changeInWip(ctx context.Context, repo models.IRepo, changFn func(root *WorkTree) error) (*WorkTree, error) {
	if !(repository.state == InWip) {
		return nil, errors.New("must commit changes on branch")
	}

	workTree, err := repository.rootTree(ctx, repo)
	if err != nil {
		return nil, err
	}

	return workTree, changFn(workTree)
}

func (repository *WorkRepository) commitChangeRoot(ctx context.Context, repo models.IRepo, author models.Signature, root hash.Hash, msg string) (*models.Commit, error) {
	parentHash := make([]hash.Hash, 0) //avoid nil parent
	if !repository.branch.CommitHash.IsEmpty() {
		parentHash = []hash.Hash{repository.branch.CommitHash}
	}

	commit := &models.Commit{
		Hash:         nil,
		RepositoryID: repository.repoModel.ID,
		Author:       author,
		Committer: models.Signature{
			Name:  repository.operator.Name,
			Email: repository.operator.Email,
			When:  time.Now(),
		},
		MergeTag:     "",
		Message:      msg,
		TreeHash:     root,
		ParentHashes: parentHash,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	commitHash, err := commit.GetHash()
	if err != nil {
		return nil, err
	}
	commit.Hash = commitHash

	_, err = repo.CommitRepo(repository.repoModel.ID).Insert(ctx, commit)
	if err != nil {
		return nil, err
	}

	// Update branch
	err = repo.BranchRepo().UpdateByID(ctx, models.NewUpdateBranchParams(repository.branch.ID).SetCommitHash(commitHash))
	if err != nil {
		return nil, err
	}
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

	commitHash := hash.Empty
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
		if err != nil {
			return err
		}

		return err
	})
}

// CreateTag create tag base on current head
func (repository *WorkRepository) CreateTag(ctx context.Context, tagName string, msg *string) (*models.Tag, error) {
	//check exit
	_, err := repository.repo.TagRepo().Get(ctx, models.NewGetTagParams().SetName(tagName).SetRepositoryID(repository.repoModel.ID))
	if err == nil {
		return nil, fmt.Errorf("%s already exit", tagName)
	}

	if err != nil && !errors.Is(err, models.ErrNotFound) {
		return nil, err
	}

	if repository.commit == nil {
		return nil, fmt.Errorf("no commit to create tag")
	}

	commitHash := repository.commit.Hash
	if commitHash.IsEmpty() {
		return nil, fmt.Errorf("empty commit to create tag")
	}

	// Create branch
	newTag := &models.Tag{
		CreatorID:    repository.operator.ID,
		Target:       commitHash,
		Message:      msg,
		RepositoryID: repository.repoModel.ID,
		Name:         tagName,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	return repository.repo.TagRepo().Insert(ctx, newTag)
}

// DeleteTag delete tag
func (repository *WorkRepository) DeleteTag(ctx context.Context) error {
	return repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		delTagParams := models.NewDeleteTagParams().
			SetRepositoryID(repository.repoModel.ID).
			SetID(repository.tag.ID)
		_, err := repo.TagRepo().Delete(ctx, delTagParams)
		return err
	})
}

// GetOrCreateWip get wip if exited, otherwise create one
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
	currentTreeHash := hash.Empty
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
	repository.setCurState(InWip, wip, repository.branch, nil, nil)
	return wip, true, nil
}

// DiffCommit find file changes in two commit
func (repository *WorkRepository) DiffCommit(ctx context.Context, toCommitID hash.Hash, pathPrefix string) (*Changes, error) {
	workTree, err := repository.RootTree(ctx)
	if err != nil {
		return nil, err
	}
	toCommit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, toCommitID)
	if err != nil {
		return nil, err
	}

	return workTree.Diff(ctx, toCommit.TreeHash, pathPrefix)
}

func (repository *WorkRepository) GetCommitChanges(ctx context.Context, pathPrefix string) (*Changes, error) {
	commitHash := hash.Empty
	if len(repository.commit.ParentHashes) == 1 {
		commitHash = repository.commit.ParentHashes[0]
	} else if len(repository.commit.ParentHashes) == 2 {
		commitHash = repository.commit.ParentHashes[1]
	}

	treeHash := hash.Empty
	if !commitHash.IsEmpty() {
		commit, err := repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, commitHash)
		if err != nil {
			return nil, err
		}
		treeHash = commit.TreeHash
	}

	workTree, err := NewWorkTree(ctx, repository.repo.FileTreeRepo(repository.repoModel.ID), models.NewRootTreeEntry(treeHash))
	if err != nil {
		return nil, err
	}
	return workTree.Diff(ctx, repository.commit.TreeHash, pathPrefix)
}

func (repository *WorkRepository) GetMergeState(ctx context.Context, toMergeCommitHash hash.Hash) ([]*ChangePair, error) {
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

	var toMergeCommit *models.Commit
	if !toMergeCommitHash.IsEmpty() {
		toMergeCommit, err = repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, toMergeCommitHash)
		if err != nil {
			return nil, err
		}
	}

	var bestAncestor *models.Commit
	err = repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		commitRepo := repo.CommitRepo(repository.repoModel.ID)
		fileTreeRepo := repo.FileTreeRepo(repository.repoModel.ID)
		var err error
		bestAncestor, err = findBestAncestor(ctx, commitRepo, fileTreeRepo, repository.operator, repository.repoModel, commit, toMergeCommit)
		if err != nil {
			return err
		}

		return ErrStop
	})
	if err != nil && !errors.Is(err, ErrStop) {
		return nil, err
	}

	ancestorWorkTree, err := NewWorkTree(ctx, repository.repo.FileTreeRepo(repository.repoModel.ID), models.NewRootTreeEntry(bestAncestor.TreeHash))
	if err != nil {
		return nil, err
	}

	baseDiff, err := ancestorWorkTree.Diff(ctx, treeHashFromCommit(commit), "")
	if err != nil {
		return nil, err
	}

	mergeDiff, err := ancestorWorkTree.Diff(ctx, treeHashFromCommit(toMergeCommit), "")
	if err != nil {
		return nil, err
	}

	changePairs := make([]*ChangePair, 0)
	iter := NewChangesPairIter(baseDiff, mergeDiff)
	for iter.Has() {
		changePair, err := iter.Next()
		if err != nil {
			return nil, err
		}
		changePairs = append(changePairs, changePair)
	}
	return changePairs, nil
}

// Merge implement merge like git, docs https://en.wikipedia.org/wiki/Merge_(version_control)
func (repository *WorkRepository) Merge(ctx context.Context, toMergeCommitHash hash.Hash, msg string, resolver ConflictResolver) (*models.Commit, error) {
	if repository.state != InBranch {
		return nil, errors.New("must merge on branch")
	}
	var targetCommit *models.Commit
	var err error
	if !repository.branch.CommitHash.IsEmpty() {
		//get branch commit
		targetCommit, err = repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, repository.branch.CommitHash)
		if err != nil {
			return nil, err
		}
	}

	var sourceCommit *models.Commit
	if !toMergeCommitHash.IsEmpty() {
		//get toMergeCommit
		sourceCommit, err = repository.repo.CommitRepo(repository.repoModel.ID).Commit(ctx, toMergeCommitHash)
		if err != nil {
			return nil, err
		}
	}

	var newCommit *models.Commit
	err = repository.repo.Transaction(ctx, func(repo models.IRepo) error {
		commitRepo := repo.CommitRepo(repository.repoModel.ID)
		fileTreeRepo := repo.FileTreeRepo(repository.repoModel.ID)
		bestAncestor, err := findBestAncestor(ctx, commitRepo, fileTreeRepo, repository.operator, repository.repoModel, sourceCommit, targetCommit)
		if err != nil {
			return err
		}
		newCommit, err = merge(ctx, commitRepo, fileTreeRepo, repository.repoModel, repository.operator, bestAncestor, sourceCommit, targetCommit, msg, resolver)
		if err != nil {
			return err
		}

		updateParams := models.NewUpdateBranchParams(repository.branch.ID).SetCommitHash(newCommit.Hash)
		return repo.BranchRepo().UpdateByID(ctx, updateParams)
	})
	if err != nil {
		return nil, err
	}
	return newCommit, nil
}

type ArchiveType string

const (
	ZipArchiveType ArchiveType = "zip"
	CarArchiveType ArchiveType = "car"
)

func (repository *WorkRepository) Archive(ctx context.Context, archiveType ArchiveType) (io.ReadCloser, int64, error) {
	rootTree, err := repository.RootTree(ctx)
	if err != nil {
		return nil, 0, err
	}

	wk := NewFileWalk(rootTree.object, rootTree.root)
	reader := func(ctx context.Context, blob *models.Blob, s string) (io.ReadCloser, error) {
		return repository.ReadBlob(ctx, blob, nil)
	}

	archiver := NewRepoArchiver(repository.repoModel.Name, wk, reader)
	tmpDir, err := os.MkdirTemp(os.TempDir(), "*") //todo file cache for archive
	if err != nil {
		return nil, 0, err
	}
	var tmpFile string
	switch archiveType {
	case ZipArchiveType:
		tmpFile = path.Join(tmpDir, hash.Hash(rootTree.root.Hash()).Hex()+".zip")
		err = archiver.ArchiveZip(ctx, tmpFile)
	case CarArchiveType:
		tmpFile = path.Join(tmpDir, hash.Hash(rootTree.root.Hash()).Hex()+".car")
		err = archiver.ArchiveZip(ctx, tmpFile)
	default:
		return nil, 0, fmt.Errorf("unexpect archive type %s", archiveType)
	}
	if err != nil {
		return nil, 0, err
	}
	st, err := os.Stat(tmpFile)
	if err != nil {
		return nil, 0, err
	}
	fs, err := os.Open(tmpFile)
	if err != nil {
		return nil, 0, err
	}
	return fs, st.Size(), nil
}

func (repository *WorkRepository) setCurState(state WorkRepoState, wip *models.WorkingInProcess, branch *models.Branch, tag *models.Tag, commit *models.Commit) {
	repository.state = state
	repository.wip = wip
	repository.branch = branch
	repository.tag = tag
	repository.commit = commit
}

// CurWip return current wip if in wip, else return nil
func (repository *WorkRepository) CurWip() *models.WorkingInProcess {
	return repository.wip
}

// CurBranch return current branch if in branch, else return nil
func (repository *WorkRepository) CurBranch() *models.Branch {
	return repository.branch
}

// CurTag return current tag if in tag, else return nil
func (repository *WorkRepository) CurTag() *models.Branch {
	return repository.branch
}

func (repository *WorkRepository) Reset() {
	repository.headTree = nil
	repository.setCurState("", nil, nil, nil, nil)
}

func findBestAncestor(ctx context.Context,
	commitRepo models.ICommitRepo,
	fileTreeRepo models.IFileTreeRepo,
	merger *models.User,
	repoModel *models.Repository,
	baseCommit *models.Commit,
	toMergeCommit *models.Commit) (*models.Commit, error) {

	if baseCommit == nil && toMergeCommit == nil {
		return nil, errors.New("cannot find nil commit")
	}

	if baseCommit == nil && toMergeCommit != nil {
		return toMergeCommit, nil
	}

	if baseCommit != nil && toMergeCommit == nil {
		return baseCommit, nil
	}
	//find accessor
	baseCommitNode := NewWrapCommitNode(commitRepo, baseCommit)
	toMergeCommitNode := NewWrapCommitNode(commitRepo, toMergeCommit)

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
		subBestAncestor, err := findBestAncestor(ctx, commitRepo, fileTreeRepo, merger, repoModel, bestAncestor[0].Commit(), bestAncestor[1].Commit())
		if err != nil {
			return nil, err
		}

		virtualCommit, err := merge(ctx, commitRepo, fileTreeRepo, repoModel, merger, subBestAncestor, bestAncestor[0].Commit(), bestAncestor[1].Commit(), "virtual commit", ForbidResolver)
		if err != nil {
			return nil, err
		}

		bestCommit = NewWrapCommitNode(commitRepo, virtualCommit)
	}
	return bestCommit.Commit(), nil
}

// merge
// todo too much arguments, need a better solution
func merge(ctx context.Context,
	commitRepo models.ICommitRepo,
	fileTreeRepo models.IFileTreeRepo,
	repoModel *models.Repository,
	merger *models.User,
	bestAncestor *models.Commit,
	sourceCommit *models.Commit,
	targetCommit *models.Commit,
	msg string,
	resolver ConflictResolver) (*models.Commit, error) {
	if sourceCommit == nil && targetCommit == nil {
		return nil, errors.New("cannot find nil commit")
	}

	if sourceCommit != nil && targetCommit == nil {
		//do nothing
		return sourceCommit, nil
	}

	if sourceCommit == nil && targetCommit != nil {
		return targetCommit, nil
	}

	baseCommitNode := NewWrapCommitNode(commitRepo, sourceCommit)
	targetMergeCommitNode := NewWrapCommitNode(commitRepo, targetCommit)

	{
		//do nothing while merge is ancestor of base
		mergeIsAncestorOfBase, err := targetMergeCommitNode.IsAncestor(ctx, baseCommitNode)
		if err != nil {
			return nil, err
		}

		if mergeIsAncestorOfBase {
			workRepoLog.Warnf("merge commit %s is ancestor of base commit %s", targetCommit.Hash, sourceCommit.Hash)
			return sourceCommit, nil
		}
	}

	{
		//try fast-forward merge no need to create new commit node
		baseIsAncestorOfMerge, err := baseCommitNode.IsAncestor(ctx, targetMergeCommitNode)
		if err != nil {
			return nil, err
		}

		if baseIsAncestorOfMerge {
			workRepoLog.Warnf("base commit %s is ancestor of merge commit %s", targetCommit.Hash, sourceCommit.Hash)
			return targetCommit, nil
		}
	}

	ancestorWorkTree, err := NewWorkTree(ctx, fileTreeRepo, models.NewRootTreeEntry(bestAncestor.TreeHash))
	if err != nil {
		return nil, err
	}

	sourceDiff, err := ancestorWorkTree.Diff(ctx, sourceCommit.TreeHash, "")
	if err != nil {
		return nil, err
	}

	targetDiff, err := ancestorWorkTree.Diff(ctx, targetCommit.TreeHash, "")
	if err != nil {
		return nil, err
	}

	//merge diff
	baseWorkTree, err := NewWorkTree(ctx, fileTreeRepo, models.NewRootTreeEntry(bestAncestor.TreeHash))
	if err != nil {
		return nil, err
	}

	cmw := NewChangesMergeIter(sourceDiff, targetDiff, resolver)
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
		ParentHashes: []hash.Hash{sourceCommit.Hash, targetCommit.Hash},
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	hash, err := mergeCommit.GetHash()
	if err != nil {
		return nil, err
	}
	mergeCommit.Hash = hash

	mergeCommit, err = commitRepo.Insert(ctx, mergeCommit)
	if err != nil {
		return nil, err
	}
	return mergeCommit, nil
}

func treeHashFromCommit(commit *models.Commit) hash.Hash {
	if commit != nil {
		return commit.TreeHash
	}
	return hash.Empty
}

func commitHashFromCommit(commit *models.Commit) hash.Hash { //nolint
	if commit != nil {
		return commit.Hash
	}
	return hash.Empty
}
