package versionmgr

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/go-git/go-git/v5/utils/merkletrie"

	"github.com/jiaozifs/jiaozifs/models/filemode"

	"github.com/go-git/go-git/v5/utils/merkletrie/noder"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/google/uuid"

	"github.com/jiaozifs/jiaozifs/models"
)

var (
	ErrConflict = errors.New("conflict dected but not found resolver")
)

type CommitOp struct {
	commit *models.Commit

	user   models.IUserRepo
	object models.IObjectRepo
	wip    models.IWipRepo
}

func NewCommitOp(repo models.IRepo, commit *models.Commit) *CommitOp {
	return &CommitOp{
		commit: commit,
		user:   repo.UserRepo(),
		object: repo.ObjectRepo(),
		wip:    repo.WipRepo(),
	}
}

func (commitOp *CommitOp) Commit() *models.Commit {
	return commitOp.commit
}

func (commitOp *CommitOp) AddCommit(ctx context.Context, committerID, wipID uuid.UUID, msg string) (*CommitOp, error) {
	wip, err := commitOp.wip.Get(ctx, &models.GetWipParam{
		ID: wipID,
	})
	if err != nil {
		return nil, err
	}

	committer, err := commitOp.user.Get(ctx, &models.GetUserParam{
		ID: committerID,
	})
	if err != nil {
		return nil, err
	}

	creator, err := commitOp.user.Get(ctx, &models.GetUserParam{
		ID: wip.CreateID,
	})
	if err != nil {
		return nil, err
	}

	parentHash := []hash.Hash{}
	if commitOp.commit != nil {
		parentHash = []hash.Hash{commitOp.commit.Hash}
	}
	commit := &models.Commit{
		Hash: nil,
		Type: models.CommitObject,
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
	_, err = commitOp.object.Insert(ctx, commit.Object())
	if err != nil {
		return nil, err
	}

	return &CommitOp{
		commit: commit,
		user:   commitOp.user,
		object: commitOp.object,
		wip:    commitOp.wip,
	}, nil
}

func (commitOp *CommitOp) DiffCommit(ctx context.Context, toCommitID hash.Hash) (*Changes, error) {
	fromNode, err := NewTreeNode(ctx, models.TreeEntry{
		Name: "",
		Mode: filemode.Dir,
		Hash: commitOp.commit.TreeHash,
	}, commitOp.object)
	if err != nil {
		return nil, err
	}

	toCommit, err := commitOp.object.Commit(ctx, toCommitID)
	if err != nil {
		return nil, err
	}
	toNode, err := NewTreeNode(ctx, models.TreeEntry{
		Name: "",
		Mode: filemode.Dir,
		Hash: toCommit.TreeHash,
	}, commitOp.object)
	if err != nil {
		return nil, err
	}

	changes, err := merkletrie.DiffTreeContext(ctx, fromNode, toNode, func(a, b noder.Hasher) bool {
		return bytes.Equal(a.Hash(), b.Hash())
	})
	if err != nil {
		return nil, err
	}
	return newChanges(changes), nil
}

func (commitOp *CommitOp) Merge(ctx context.Context, mergerID uuid.UUID, toMergeCommitHash hash.Hash, msg string) (*models.Commit, error) {
	merger, err := commitOp.user.Get(ctx, &models.GetUserParam{
		ID: mergerID,
	})
	if err != nil {
		return nil, err
	}

	toMergeCommit, err := commitOp.object.Commit(ctx, toMergeCommitHash)
	if err != nil {
		return nil, err
	}

	//find accesstor
	baseCommitNode := NewCommitNode(ctx, commitOp.commit, commitOp.object)
	toMergeCommitNode := NewCommitNode(ctx, toMergeCommit, commitOp.object)
	bestAncestor, err := baseCommitNode.MergeBase(toMergeCommitNode)
	if err != nil {
		return nil, err
	}

	//Fast forward merge
	//3 way merge

	if len(bestAncestor) == 0 {
		return nil, fmt.Errorf("no common ancesstor find")
	}

	bestCommit := bestAncestor[0]
	if len(bestAncestor) > 1 {
		//merge cross merge create virtual commit
		firstCommit := &CommitOp{
			commit: bestAncestor[0].Commit(),
			user:   commitOp.user,
			object: commitOp.object,
			wip:    commitOp.wip,
		}
		virtualCommit, err := firstCommit.Merge(ctx, mergerID, bestAncestor[1].Commit().Hash, "")
		if err != nil {
			return nil, err
		}

		bestCommit = NewCommitNode(ctx, virtualCommit, commitOp.object)
	}

	baseDiff, err := commitOp.DiffCommit(ctx, bestCommit.Commit().Hash)
	if err != nil {
		return nil, err
	}

	mergeCommitOp := &CommitOp{
		commit: toMergeCommit,
		user:   commitOp.user,
		object: commitOp.object,
		wip:    commitOp.wip,
	}
	mergeDiff, err := mergeCommitOp.DiffCommit(ctx, bestCommit.Commit().Hash)
	if err != nil {
		return nil, err
	}

	//merge diff
	cmw := NewChangesMergeIter(baseDiff, mergeDiff, nil)
	workTree, err := NewWorkTree(ctx, commitOp.object, models.NewRootTreeEntry(bestCommit.Commit().TreeHash))
	if err != nil {
		return nil, err
	}

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
		Type:         models.CommitObject,
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

	mergeCommitObject, err := commitOp.object.Insert(ctx, mergeCommit.Object())
	if err != nil {
		return nil, err
	}
	return mergeCommitObject.Commit(), nil
}
