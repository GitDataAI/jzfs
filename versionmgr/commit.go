package versionmgr

import (
	"bytes"
	"context"
	"time"

	"github.com/go-git/go-git/v5/utils/merkletrie"

	"github.com/jiaozifs/jiaozifs/models/filemode"

	"github.com/go-git/go-git/v5/utils/merkletrie/noder"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/google/uuid"

	"github.com/jiaozifs/jiaozifs/models"
)

type CommitOp struct {
	User   models.IUserRepo
	Object models.IObjectRepo
	Wip    models.IWipRepo
	Ref    models.IRefRepo
}

func NewCommitOp(repo models.IRepo) *CommitOp {
	return &CommitOp{
		User:   repo.UserRepo(),
		Object: repo.ObjectRepo(),
		Wip:    repo.WipRepo(),
		Ref:    repo.RefRepo(),
	}
}

func (commitOp *CommitOp) AddCommit(ctx context.Context, refID, committerID, wipID uuid.UUID, msg string) (*models.Commit, error) {
	wip, err := commitOp.Wip.Get(ctx, &models.GetWipParam{
		ID: wipID,
	})
	if err != nil {
		return nil, err
	}

	committer, err := commitOp.User.Get(ctx, &models.GetUserParam{
		ID: committerID,
	})
	if err != nil {
		return nil, err
	}

	creator, err := commitOp.User.Get(ctx, &models.GetUserParam{
		ID: wip.CreateID,
	})
	if err != nil {
		return nil, err
	}

	ref, err := commitOp.Ref.Get(ctx, &models.GetRefParams{
		ID: refID,
	})
	if err != nil {
		return nil, err
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
		ParentHashes: []hash.Hash{ref.CommitHash},
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	commitHash, err := commit.GetHash()
	if err != nil {
		return nil, err
	}
	commit.Hash = commitHash
	_, err = commitOp.Object.Insert(ctx, commit.Object())
	if err != nil {
		return nil, err
	}

	ref.CommitHash = commitHash
	err = commitOp.Ref.UpdateCommitHash(ctx, ref.ID, commitHash)
	if err != nil {
		return nil, err
	}
	err = commitOp.Wip.UpdateState(ctx, wipID, models.Completed)
	if err != nil {
		return nil, err
	}
	return commit, nil
}

func (commitOp *CommitOp) DiffCommit(ctx context.Context, baseCommitID, toCommitID hash.Hash) (merkletrie.Changes, error) {
	baseCommit, err := commitOp.Object.Commit(ctx, baseCommitID)
	if err != nil {
		return nil, err
	}

	toCommit, err := commitOp.Object.Commit(ctx, toCommitID)
	if err != nil {
		return nil, err
	}

	fromNode, err := NewTreeNode(ctx, models.TreeEntry{
		Name: "",
		Mode: filemode.Dir,
		Hash: baseCommit.TreeHash,
	}, commitOp.Object)
	if err != nil {
		return nil, err
	}

	toNode, err := NewTreeNode(ctx, models.TreeEntry{
		Name: "",
		Mode: filemode.Dir,
		Hash: toCommit.TreeHash,
	}, commitOp.Object)
	if err != nil {
		return nil, err
	}

	return merkletrie.DiffTreeContext(ctx, fromNode, toNode, func(a, b noder.Hasher) bool {
		return bytes.Equal(a.Hash(), b.Hash())
	})
}

func (commitOp *CommitOp) Merge(ctx context.Context, mergerID, baseRefID, mergeRefID uuid.UUID) (*models.Commit, error) {
	_, err := commitOp.User.Get(ctx, &models.GetUserParam{
		ID: mergerID,
	})
	if err != nil {
		return nil, err
	}

	baseRef, err := commitOp.Ref.Get(ctx, &models.GetRefParams{
		ID: baseRefID,
	})
	if err != nil {
		return nil, err
	}

	mergeRef, err := commitOp.Ref.Get(ctx, &models.GetRefParams{
		ID: mergeRefID,
	})
	if err != nil {
		return nil, err
	}

	_, err = commitOp.DiffCommit(ctx, baseRef.CommitHash, mergeRef.CommitHash)
	if err != nil {
		return nil, err
	}

	return nil, nil
}
