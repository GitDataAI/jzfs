package versionmgr

import (
	"context"
	"errors"
	"io"

	"github.com/GitDataAI/jiaozifs/utils/hash"
)

type bfsCommitIterator struct {
	ctx          context.Context
	seenExternal map[string]bool
	seen         map[string]bool
	queue        []*WrapCommitNode
}

// NewCommitIterBSF returns a CommitIter that walks the commit history,
// starting at the given commit and visiting its parents in pre-order.
// The given callback will be called for each visited commit. Each commit will
// be visited only once. If the callback returns an error, walking will stop
// and will return the error. Other errors might be returned if the history
// cannot be traversed (e.g. missing objects). Ignore allows to skip some
// commits from being iterated.
func NewCommitIterBSF(
	ctx context.Context,
	c *WrapCommitNode,
	seenExternal map[string]bool,
	ignore []hash.Hash,
) CommitIter {
	seen := make(map[string]bool)
	for _, h := range ignore {
		seen[h.Hex()] = true
	}

	return &bfsCommitIterator{
		ctx:          ctx,
		seenExternal: seenExternal,
		seen:         seen,
		queue:        []*WrapCommitNode{c},
	}
}

func (w *bfsCommitIterator) appendHash(ctx context.Context, store *WrapCommitNode, h hash.Hash) error {
	if w.seen[h.Hex()] || w.seenExternal[h.Hex()] {
		return nil
	}
	c, err := store.GetCommit(ctx, h)
	if err != nil {
		return err
	}
	w.queue = append(w.queue, c)
	return nil
}

func (w *bfsCommitIterator) Next() (*WrapCommitNode, error) {
	var c *WrapCommitNode
	for {
		if len(w.queue) == 0 {
			return nil, io.EOF
		}
		c = w.queue[0]
		w.queue = w.queue[1:]

		if w.seen[c.Commit().Hash.Hex()] || w.seenExternal[c.Commit().Hash.Hex()] {
			continue
		}

		w.seen[c.Commit().Hash.Hex()] = true

		for _, h := range c.Commit().ParentHashes {
			err := w.appendHash(w.ctx, c, h)
			if err != nil {
				return nil, err
			}
		}

		return c, nil
	}
}

func (w *bfsCommitIterator) ForEach(cb func(node *WrapCommitNode) error) error {
	for {
		c, err := w.Next()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}

		err = cb(c)
		if errors.Is(err, ErrStop) {
			break
		}
		if err != nil {
			return err
		}
	}

	return nil
}
