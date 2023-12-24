package versionmgr

import (
	"context"
	"errors"
	"io"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/emirpasic/gods/trees/binaryheap"
)

type commitIteratorByCTime struct {
	ctx          context.Context
	seenExternal map[string]bool
	seen         map[string]bool
	heap         *binaryheap.Heap
}

// NewCommitIterCTime returns a CommitIter that walks the commit history,
// starting at the given commit and visiting its parents while preserving Committer Time order.
// this appears to be the closest order to `git log`
// The given callback will be called for each visited commit. Each commit will
// be visited only once. If the callback returns an error, walking will stop
// and will return the error. Other errors might be returned if the history
// cannot be traversed (e.g. missing objects). Ignore allows to skip some
// commits from being iterated.
func NewCommitIterCTime(
	ctx context.Context,
	c *WrapCommitNode,
	seenExternal map[string]bool,
	ignore []hash.Hash,
) CommitIter {
	seen := make(map[string]bool)
	for _, h := range ignore {
		seen[h.Hex()] = true
	}

	heap := binaryheap.NewWith(func(a, b interface{}) int {
		if a.(*WrapCommitNode).Commit().Committer.When.Before(b.(*WrapCommitNode).Commit().Committer.When) {
			return 1
		}
		return -1
	})
	heap.Push(c)

	return &commitIteratorByCTime{
		ctx:          ctx,
		seenExternal: seenExternal,
		seen:         seen,
		heap:         heap,
	}
}

func (w *commitIteratorByCTime) Next() (*WrapCommitNode, error) {
	var c *WrapCommitNode
	for {
		cIn, ok := w.heap.Pop()
		if !ok {
			return nil, io.EOF
		}
		c = cIn.(*WrapCommitNode)

		if w.seen[c.Commit().Hash.Hex()] || w.seenExternal[c.Commit().Hash.Hex()] {
			continue
		}

		w.seen[c.Commit().Hash.Hex()] = true

		for _, h := range c.Commit().ParentHashes {
			if w.seen[h.Hex()] || w.seenExternal[h.Hex()] {
				continue
			}
			pc, err := c.GetCommit(w.ctx, h)
			if err != nil {
				return nil, err
			}
			w.heap.Push(pc)
		}

		return c, nil
	}
}

func (w *commitIteratorByCTime) ForEach(cb func(*WrapCommitNode) error) error {
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

func (w *commitIteratorByCTime) Close() {}
