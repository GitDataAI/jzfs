package versionmgr

import (
	"context"
	"errors"
	"io"

	"github.com/go-git/go-git/v5/plumbing/storer"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils/hash"
)

var (
	ErrStop = errors.New("stop iter")
)

type CommitNode struct {
	ctx        context.Context
	commit     *models.Commit
	commitRepo models.ICommitRepo
}

func NewCommitNode(ctx context.Context, commit *models.Commit, commitRepo models.ICommitRepo) *CommitNode {
	return &CommitNode{ctx: ctx, commit: commit, commitRepo: commitRepo}
}

func (c *CommitNode) Ctx() context.Context {
	return c.ctx
}

func (c *CommitNode) Commit() *models.Commit {
	return c.commit
}

func (c *CommitNode) RepoID() uuid.UUID {
	return c.commit.RepositoryID
}

// TreeHash returns the TreeHash in the commit.
func (c *CommitNode) TreeHash() hash.Hash {
	return c.commit.TreeHash
}

// Parents return a CommitIter to the parent Commits.
func (c *CommitNode) Parents() ([]*CommitNode, error) {
	parentNodes := make([]*CommitNode, len(c.commit.ParentHashes))
	for _, hash := range c.commit.ParentHashes {
		commit, err := c.commitRepo.Commit(c.ctx, hash)
		if err != nil {
			return nil, err
		}
		parentNodes = append(parentNodes, &CommitNode{
			ctx:        c.ctx,
			commit:     commit,
			commitRepo: c.commitRepo,
		})
	}
	return parentNodes, nil
}

func (c *CommitNode) GetCommit(hash hash.Hash) (*CommitNode, error) {
	commit, err := c.commitRepo.Commit(c.ctx, hash)
	if err != nil {
		return nil, err
	}
	return &CommitNode{
		ctx:        c.ctx,
		commit:     commit,
		commitRepo: c.commitRepo,
	}, nil
}

func (c *CommitNode) GetCommits(hashes []hash.Hash) ([]*CommitNode, error) {
	commits := make([]*CommitNode, len(hashes))
	for i, hash := range hashes {
		commit, err := c.commitRepo.Commit(c.ctx, hash)
		if err != nil {
			return nil, err
		}
		commits[i] = &CommitNode{
			ctx:        c.ctx,
			commit:     commit,
			commitRepo: c.commitRepo,
		}
	}
	return commits, nil
}

// CommitIter is a generic closable interface for iterating over commits.
type CommitIter interface {
	Next() (*CommitNode, error)
	ForEach(func(*CommitNode) error) error
}

var _ CommitIter = (*arraryCommitIter)(nil)

type arraryCommitIter struct {
	commits []*CommitNode
	idx     int
}

func newArrayCommitIter(commits []*CommitNode) *arraryCommitIter {
	return &arraryCommitIter{
		commits: commits,
		idx:     -1,
	}
}

func (a *arraryCommitIter) Next() (*CommitNode, error) {
	if a.idx < len(a.commits)-1 {
		a.idx++
		return a.commits[a.idx], nil
	}
	return nil, io.EOF
}

func (a *arraryCommitIter) ForEach(f func(*CommitNode) error) error {
	for _, commit := range a.commits {
		err := f(commit)
		if errors.Is(err, storer.ErrStop) {
			break
		}
		if err != nil {
			return err
		}
	}
	return nil
}

func (a *arraryCommitIter) Has() bool {
	return a.idx == len(a.commits)-1
}
