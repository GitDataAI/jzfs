package versionmgr

import (
	"context"
	"errors"
	"io"

	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils/hash"
)

var (
	ErrStop = errors.New("stop iter")
)

type WrapCommitNode struct {
	commit     *models.Commit
	commitRepo models.ICommitRepo
}

func NewWrapCommitNode(commitRepo models.ICommitRepo, commit *models.Commit) *WrapCommitNode {
	return &WrapCommitNode{commit: commit, commitRepo: commitRepo}
}

func (c *WrapCommitNode) Commit() *models.Commit {
	return c.commit
}

func (c *WrapCommitNode) RepoID() uuid.UUID {
	return c.commit.RepositoryID
}

// TreeHash returns the TreeHash in the commit.
func (c *WrapCommitNode) TreeHash() hash.Hash {
	return c.commit.TreeHash
}

// Hash returns the Hash in the commit.
func (c *WrapCommitNode) Hash() hash.Hash {
	return c.commit.Hash
}

// Parents return a CommitIter to the parent Commits.
func (c *WrapCommitNode) Parents(ctx context.Context) ([]*WrapCommitNode, error) {
	parentNodes := make([]*WrapCommitNode, len(c.commit.ParentHashes))
	for index, hash := range c.commit.ParentHashes {
		commit, err := c.commitRepo.Commit(ctx, hash)
		if err != nil {
			return nil, err
		}
		parentNodes[index] = &WrapCommitNode{
			commit:     commit,
			commitRepo: c.commitRepo,
		}
	}
	return parentNodes, nil
}

func (c *WrapCommitNode) GetCommit(ctx context.Context, hash hash.Hash) (*WrapCommitNode, error) {
	commit, err := c.commitRepo.Commit(ctx, hash)
	if err != nil {
		return nil, err
	}
	return &WrapCommitNode{
		commit:     commit,
		commitRepo: c.commitRepo,
	}, nil
}

func (c *WrapCommitNode) GetCommits(ctx context.Context, hashes []hash.Hash) ([]*WrapCommitNode, error) {
	commits := make([]*WrapCommitNode, len(hashes))
	for i, hash := range hashes {
		commit, err := c.commitRepo.Commit(ctx, hash)
		if err != nil {
			return nil, err
		}
		commits[i] = &WrapCommitNode{
			commit:     commit,
			commitRepo: c.commitRepo,
		}
	}
	return commits, nil
}

// CommitIter is a generic closable interface for iterating over commits.
type CommitIter interface {
	Next() (*WrapCommitNode, error)
	ForEach(func(*WrapCommitNode) error) error
}

var _ CommitIter = (*arraryCommitIter)(nil)

type arraryCommitIter struct {
	commits []*WrapCommitNode
	idx     int
}

func newArrayCommitIter(commits []*WrapCommitNode) *arraryCommitIter {
	return &arraryCommitIter{
		commits: commits,
		idx:     -1,
	}
}

func (a *arraryCommitIter) Next() (*WrapCommitNode, error) {
	if a.idx < len(a.commits)-1 {
		a.idx++
		return a.commits[a.idx], nil
	}
	return nil, io.EOF
}

func (a *arraryCommitIter) ForEach(f func(*WrapCommitNode) error) error {
	for _, commit := range a.commits {
		err := f(commit)
		if errors.Is(err, ErrStop) {
			break
		}
		if err != nil {
			return err
		}
	}
	return nil
}

func (a *arraryCommitIter) Has() bool {
	return a.idx < len(a.commits)-1
}
