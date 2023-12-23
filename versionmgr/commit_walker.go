package versionmgr

import (
	"context"
	"errors"
	"io"

	"github.com/jiaozifs/jiaozifs/utils/hash"
)

type commitPreIterator struct {
	ctx          context.Context
	seenExternal map[string]bool
	seen         map[string]bool
	stack        []CommitIter
	start        *WrapCommitNode
}

// NewCommitPreorderIter returns a CommitIter that walks the commit history,
// starting at the given commit and visiting its parents in pre-order.
// The given callback will be called for each visited commit. Each commit will
// be visited only once. If the callback returns an error, walking will stop
// and will return the error. Other errors might be returned if the history
// cannot be traversed (e.g. missing objects). Ignore allows to skip some
// commits from being iterated.
func NewCommitPreorderIter(
	ctx context.Context,
	c *WrapCommitNode,
	seenExternal map[string]bool,
	ignore []hash.Hash,
) CommitIter {
	seen := make(map[string]bool)
	for _, h := range ignore {
		seen[h.Hex()] = true
	}

	return &commitPreIterator{
		ctx:          ctx,
		seenExternal: seenExternal,
		seen:         seen,
		stack:        make([]CommitIter, 0),
		start:        c,
	}
}

func (w *commitPreIterator) Next() (*WrapCommitNode, error) {
	var c *WrapCommitNode
	for {
		if w.start != nil {
			c = w.start
			w.start = nil
		} else {
			current := len(w.stack) - 1
			if current < 0 {
				return nil, io.EOF
			}

			var err error
			c, err = w.stack[current].Next()
			if err == io.EOF {
				w.stack = w.stack[:current]
				continue
			}

			if err != nil {
				return nil, err
			}
		}

		if w.seen[c.Commit().Hash.Hex()] || w.seenExternal[c.Commit().Hash.Hex()] {
			continue
		}

		w.seen[c.Commit().Hash.Hex()] = true

		if c.Commit().NumParents() > 0 {
			commitIter, err := filteredParentIter(w.ctx, c, w.seen)
			if err != nil {
				return nil, err
			}
			w.stack = append(w.stack, commitIter)
		}

		return c, nil
	}
}

func filteredParentIter(ctx context.Context, c *WrapCommitNode, seen map[string]bool) (CommitIter, error) {
	var hashes []hash.Hash
	for _, h := range c.Commit().ParentHashes {
		if !seen[h.Hex()] {
			hashes = append(hashes, h)
		}
	}
	commits, err := c.GetCommits(ctx, hashes)
	if err != nil {
		return nil, err
	}

	return newArrayCommitIter(commits), nil
}

func (w *commitPreIterator) ForEach(cb func(*WrapCommitNode) error) error {
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

type commitPostIterator struct {
	ctx   context.Context
	stack []*WrapCommitNode
	seen  map[string]bool
}

// NewCommitPostorderIter returns a CommitIter that walks the commit
// history like WalkCommitHistory but in post-order. This means that after
// walking a merge commit, the merged commit will be walked before the base
// it was merged on. This can be useful if you wish to see the history in
// chronological order. Ignore allows to skip some commits from being iterated.
func NewCommitPostorderIter(ctx context.Context, c *WrapCommitNode, ignore []hash.Hash) CommitIter {
	seen := make(map[string]bool)
	for _, h := range ignore {
		seen[h.Hex()] = true
	}

	return &commitPostIterator{
		ctx:   ctx,
		stack: []*WrapCommitNode{c},
		seen:  seen,
	}
}

func (w *commitPostIterator) Next() (*WrapCommitNode, error) {
	for {
		if len(w.stack) == 0 {
			return nil, io.EOF
		}

		c := w.stack[len(w.stack)-1]
		w.stack = w.stack[:len(w.stack)-1]

		if w.seen[c.Commit().Hash.Hex()] {
			continue
		}

		w.seen[c.Commit().Hash.Hex()] = true

		parentCommits, err := c.Parents(w.ctx)
		if err != nil {
			return nil, err
		}
		return c, newArrayCommitIter(parentCommits).ForEach(func(p *WrapCommitNode) error {
			w.stack = append(w.stack, p)
			return nil
		})
	}
}

func (w *commitPostIterator) ForEach(cb func(*WrapCommitNode) error) error {
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
