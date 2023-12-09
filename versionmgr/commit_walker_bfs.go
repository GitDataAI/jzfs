package versionmgr

import (
	"errors"
	"io"

	"github.com/go-git/go-git/v5/plumbing/storer"
	"github.com/jiaozifs/jiaozifs/utils/hash"
)

type bfsCommitIterator struct {
	seenExternal map[string]bool
	seen         map[string]bool
	queue        []*CommitNode
}

// NewCommitIterBSF returns a CommitIter that walks the commit history,
// starting at the given commit and visiting its parents in pre-order.
// The given callback will be called for each visited commit. Each commit will
// be visited only once. If the callback returns an error, walking will stop
// and will return the error. Other errors might be returned if the history
// cannot be traversed (e.g. missing objects). Ignore allows to skip some
// commits from being iterated.
func NewCommitIterBSF(
	c *CommitNode,
	seenExternal map[string]bool,
	ignore []hash.Hash,
) CommitIter {
	seen := make(map[string]bool)
	for _, h := range ignore {
		seen[h.Hex()] = true
	}

	return &bfsCommitIterator{
		seenExternal: seenExternal,
		seen:         seen,
		queue:        []*CommitNode{c},
	}
}

func (w *bfsCommitIterator) appendHash(store *CommitNode, h hash.Hash) error {
	if w.seen[h.Hex()] || w.seenExternal[h.Hex()] {
		return nil
	}
	c, err := store.GetCommit(h)
	if err != nil {
		return err
	}
	w.queue = append(w.queue, c)
	return nil
}

func (w *bfsCommitIterator) Next() (*CommitNode, error) {
	var c *CommitNode
	for {
		if len(w.queue) == 0 {
			return nil, io.EOF
		}
		c = w.queue[0]
		w.queue = w.queue[1:]

		if w.seen[c.Hash.Hex()] || w.seenExternal[c.Hash.Hex()] {
			continue
		}

		w.seen[c.Hash.Hex()] = true

		for _, h := range c.ParentHashes {
			err := w.appendHash(c, h)
			if err != nil {
				return nil, err
			}
		}

		return c, nil
	}
}

func (w *bfsCommitIterator) ForEach(cb func(node *CommitNode) error) error {
	for {
		c, err := w.Next()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}

		err = cb(c)
		if errors.Is(err, storer.ErrStop) {
			break
		}
		if err != nil {
			return err
		}
	}

	return nil
}
