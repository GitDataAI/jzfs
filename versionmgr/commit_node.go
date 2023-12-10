package versionmgr

import (
	"context"
	"errors"
	"io"

	"github.com/go-git/go-git/v5/plumbing/storer"
	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/models/filemode"

	"github.com/jiaozifs/jiaozifs/models"
)

var (
	ErrStop = errors.New("stop iter")
)

type CommitNode struct {
	Ctx context.Context
	models.Commit
	Object models.IObjectRepo
}

// Tree returns the Tree from the commit.
func (c *CommitNode) Tree() (*TreeNode, error) {
	treeNode, err := c.Object.TreeNode(c.Ctx, c.TreeHash)
	if err != nil {
		return nil, err
	}
	return NewTreeNode(c.Ctx, models.TreeEntry{
		Name: "",
		Mode: filemode.Dir,
		Hash: treeNode.Hash,
	}, c.Object)
}

// Parents return a CommitIter to the parent Commits.
func (c *CommitNode) Parents() ([]*CommitNode, error) {
	parentNodes := make([]*CommitNode, len(c.ParentHashes))
	for _, hash := range c.ParentHashes {
		commit, err := c.Object.Commit(c.Ctx, hash)
		if err != nil {
			return nil, err
		}
		parentNodes = append(parentNodes, &CommitNode{
			Ctx:    c.Ctx,
			Commit: *commit,
			Object: c.Object,
		})
	}
	return parentNodes, nil
}

func (c *CommitNode) GetCommit(hash hash.Hash) (*CommitNode, error) {
	commit, err := c.Object.Commit(c.Ctx, hash)
	if err != nil {
		return nil, err
	}
	return &CommitNode{
		Ctx:    c.Ctx,
		Commit: *commit,
		Object: c.Object,
	}, nil
}

func (c *CommitNode) GetCommits(hashes []hash.Hash) ([]*CommitNode, error) {
	commits := make([]*CommitNode, len(hashes))
	for i, hash := range hashes {
		commit, err := c.Object.Commit(c.Ctx, hash)
		if err != nil {
			return nil, err
		}
		commits[i] = &CommitNode{
			Ctx:    c.Ctx,
			Commit: *commit,
			Object: c.Object,
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

func (a arraryCommitIter) Next() (*CommitNode, error) {
	if a.idx == len(a.commits)-1 {
		a.idx++
		return a.commits[a.idx], nil
	}
	return nil, io.EOF
}

func (a arraryCommitIter) ForEach(f func(*CommitNode) error) error {
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
