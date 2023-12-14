package versionmgr

import (
	"io"

	"github.com/emirpasic/gods/trees/binaryheap"
)

type commitIteratorByCTime struct {
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
	c *CommitNode,
	seenExternal map[string]bool,
	ignore []string,
) CommitIter {
	seen := make(map[string]bool)
	for _, h := range ignore {
		seen[h] = true
	}

	heap := binaryheap.NewWith(func(a, b interface{}) int {
		if a.(*CommitNode).Commit().Committer.When.Before(b.(*CommitNode).Commit().Committer.When) {
			return 1
		}
		return -1
	})
	heap.Push(c)

	return &commitIteratorByCTime{
		seenExternal: seenExternal,
		seen:         seen,
		heap:         heap,
	}
}

func (w *commitIteratorByCTime) Next() (*CommitNode, error) {
	var c *CommitNode
	for {
		cIn, ok := w.heap.Pop()
		if !ok {
			return nil, io.EOF
		}
		c = cIn.(*CommitNode)

		if w.seen[c.Commit().Hash.Hex()] || w.seenExternal[c.Commit().Hash.Hex()] {
			continue
		}

		w.seen[c.Commit().Hash.Hex()] = true

		for _, h := range c.Commit().ParentHashes {
			if w.seen[h.Hex()] || w.seenExternal[h.Hex()] {
				continue
			}
			pc, err := c.GetCommit(h)
			if err != nil {
				return nil, err
			}
			w.heap.Push(pc)
		}

		return c, nil
	}
}

func (w *commitIteratorByCTime) ForEach(cb func(*CommitNode) error) error {
	for {
		c, err := w.Next()
		if err == io.EOF {
			break
		}
		if err != nil {
			return err
		}

		err = cb(c)
		if err == ErrStop {
			break
		}
		if err != nil {
			return err
		}
	}

	return nil
}

func (w *commitIteratorByCTime) Close() {}
