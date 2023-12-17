package versionmgr

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils/hash"
)

var (
	commitLog = logging.Logger("commit")
)

// CommitOp used to wrap some function for commit, todo not easy to use, optimize it
type CommitOp struct {
	commit *models.Commit

	userRepo     models.IUserRepo
	fileTreeRepo models.IFileTreeRepo
	commitRepo   models.ICommitRepo
	wipRepo      models.IWipRepo
}

// NewCommitOp create commit operation with repo and exit commit, if operate with new repo, set commit arguments to nil
func NewCommitOp(repo models.IRepo, commit *models.Commit) *CommitOp {
	return &CommitOp{
		commit:       commit,
		userRepo:     repo.UserRepo(),
		fileTreeRepo: repo.FileTreeRepo(),
		commitRepo:   repo.CommitRepo(),
		wipRepo:      repo.WipRepo(),
	}
}

// Commit return commit
func (commitOp *CommitOp) Commit() *models.Commit {
	return commitOp.commit
}

// AddCommit append a new commit to current head, read changes from wip, than create a new commit with parent point to current head,
// and replace tree hash with wip's currentTreeHash.
func (commitOp *CommitOp) AddCommit(ctx context.Context, committer *models.User, wipID uuid.UUID, msg string) (*CommitOp, error) {
	wip, err := commitOp.wipRepo.Get(ctx, models.NewGetWipParams().SetID(wipID))
	if err != nil {
		return nil, err
	}

	creator, err := commitOp.userRepo.Get(ctx, models.NewGetUserParams().SetID(wip.CreatorID))
	if err != nil {
		return nil, err
	}

	parentHash := []hash.Hash{}
	if commitOp.commit != nil {
		parentHash = []hash.Hash{commitOp.commit.Hash}
	}
	commit := &models.Commit{
		Hash: nil,
		Author: models.Signature{
			Name:  creator.Name,
			Email: creator.Email,
			When:  wip.UpdatedAt,
		},
		Committer: models.Signature{
			Name:  committer.Name,
			Email: committer.Email,
			When:  time.Now(),
		},
		MergeTag:     "",
		Message:      msg,
		TreeHash:     wip.CurrentTree,
		ParentHashes: parentHash,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	commitHash, err := commit.GetHash()
	if err != nil {
		return nil, err
	}
	commit.Hash = commitHash
	_, err = commitOp.commitRepo.Insert(ctx, commit)
	if err != nil {
		return nil, err
	}

	return &CommitOp{
		commit:       commit,
		userRepo:     commitOp.userRepo,
		fileTreeRepo: commitOp.fileTreeRepo,
		commitRepo:   commitOp.commitRepo,
		wipRepo:      commitOp.wipRepo,
	}, nil
}

// DiffCommit find file changes in two commit
func (commitOp *CommitOp) DiffCommit(ctx context.Context, toCommitID hash.Hash) (*Changes, error) {
	workTree, err := NewWorkTree(ctx, commitOp.fileTreeRepo, models.NewRootTreeEntry(commitOp.Commit().TreeHash))
	if err != nil {
		return nil, err
	}
	toCommit, err := commitOp.commitRepo.Commit(ctx, toCommitID)
	if err != nil {
		return nil, err
	}

	return workTree.Diff(ctx, toCommit.TreeHash)
}

// Merge implement merge like git, docs https://en.wikipedia.org/wiki/Merge_(version_control)
func (commitOp *CommitOp) Merge(ctx context.Context, merger *models.User, toMergeCommitHash hash.Hash, msg string, resolver ConflictResolver) (*models.Commit, error) {

	toMergeCommit, err := commitOp.commitRepo.Commit(ctx, toMergeCommitHash)
	if err != nil {
		return nil, err
	}

	//find accesstor
	baseCommitNode := NewCommitNode(ctx, commitOp.Commit(), commitOp.commitRepo)
	toMergeCommitNode := NewCommitNode(ctx, toMergeCommit, commitOp.commitRepo)

	{
		//do nothing while merge is ancestor of base
		mergeIsAncestorOfBase, err := toMergeCommitNode.IsAncestor(baseCommitNode)
		if err != nil {
			return nil, err
		}

		if mergeIsAncestorOfBase {
			commitLog.Warnf("merge commit %s is ancestor of base commit %s", toMergeCommitHash, commitOp.Commit().Hash)
			return commitOp.Commit(), nil
		}
	}

	{
		//try fast-forward merge no need to create new commit node
		baseIsAncestorOfMerge, err := baseCommitNode.IsAncestor(toMergeCommitNode)
		if err != nil {
			return nil, err
		}

		if baseIsAncestorOfMerge {
			commitLog.Warnf("base commit %s is ancestor of merge commit %s", toMergeCommitHash, commitOp.Commit().Hash)
			return toMergeCommit, nil
		}
	}

	// three-way merge
	bestAncestor, err := baseCommitNode.MergeBase(toMergeCommitNode)
	if err != nil {
		return nil, err
	}

	if len(bestAncestor) == 0 {
		return nil, fmt.Errorf("no common ancesstor find")
	}

	bestCommit := bestAncestor[0]
	if len(bestAncestor) > 1 {
		//merge cross merge create virtual commit
		firstCommit := &CommitOp{
			commit:       bestAncestor[0].Commit(),
			userRepo:     commitOp.userRepo,
			fileTreeRepo: commitOp.fileTreeRepo,
			commitRepo:   commitOp.commitRepo,
			wipRepo:      commitOp.wipRepo,
		}
		virtualCommit, err := firstCommit.Merge(ctx, merger, bestAncestor[1].Commit().Hash, "", resolver)
		if err != nil {
			return nil, err
		}

		bestCommit = NewCommitNode(ctx, virtualCommit, commitOp.commitRepo)
	}

	bestCommitOp := &CommitOp{
		commit:       bestAncestor[0].Commit(),
		userRepo:     commitOp.userRepo,
		fileTreeRepo: commitOp.fileTreeRepo,
		commitRepo:   commitOp.commitRepo,
		wipRepo:      commitOp.wipRepo,
	}

	baseDiff, err := bestCommitOp.DiffCommit(ctx, commitOp.Commit().Hash)
	if err != nil {
		return nil, err
	}

	mergeDiff, err := bestCommitOp.DiffCommit(ctx, toMergeCommit.Hash)
	if err != nil {
		return nil, err
	}

	//merge diff
	workTree, err := NewWorkTree(ctx, commitOp.fileTreeRepo, models.NewRootTreeEntry(bestCommit.Commit().TreeHash))
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
		err = workTree.ApplyOneChange(ctx, change)
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
		Committer:    author,
		MergeTag:     "",
		Message:      msg,
		TreeHash:     workTree.Root().Hash(),
		ParentHashes: []hash.Hash{commitOp.commit.Hash, toMergeCommitHash},
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	hash, err := mergeCommit.GetHash()
	if err != nil {
		return nil, err
	}
	mergeCommit.Hash = hash

	mergeCommitResult, err := commitOp.commitRepo.Insert(ctx, mergeCommit)
	if err != nil {
		return nil, err
	}
	return mergeCommitResult, nil
}
