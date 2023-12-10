package versionmgr

import (
	"errors"
	"io"

	"github.com/go-git/go-git/v5/plumbing/storer"
	"github.com/jiaozifs/jiaozifs/utils/hash"
)

// NewFilterCommitIter returns a CommitIter that walks the commit history,
// starting at the passed commit and visiting its parents in Breadth-first order.
// The commits returned by the CommitIter will validate the passed CommitFilter.
// The history won't be transversed beyond a commit if isLimit is true for it.
// Each commit will be visited only once.
// If the commit history can not be traversed, or the Close() method is called,
// the CommitIter won't return more commits.
// If no isValid is passed, all ancestors of from commit will be valid.
// If no isLimit is limit, all ancestors of all commits will be visited.
func NewFilterCommitIter(
	from *CommitNode,
	isValid *CommitFilter,
	isLimit *CommitFilter,
) CommitIter {
	var validFilter CommitFilter
	if isValid == nil {
		validFilter = func(_ *CommitNode) bool {
			return true
		}
	} else {
		validFilter = *isValid
	}

	var limitFilter CommitFilter
	if isLimit == nil {
		limitFilter = func(_ *CommitNode) bool {
			return false
		}
	} else {
		limitFilter = *isLimit
	}

	return &filterCommitIter{
		isValid: validFilter,
		isLimit: limitFilter,
		visited: map[string]struct{}{},
		queue:   []*CommitNode{from},
	}
}

// CommitFilter returns a boolean for the passed Commit
type CommitFilter func(*CommitNode) bool

// filterCommitIter implements CommitIter
type filterCommitIter struct {
	isValid CommitFilter
	isLimit CommitFilter
	visited map[string]struct{}
	queue   []*CommitNode
	lastErr error
}

// Next returns the next commit of the CommitIter.
// It will return io.EOF if there are no more commits to visit,
// or an error if the history could not be traversed.
func (w *filterCommitIter) Next() (*CommitNode, error) {
	var commit *CommitNode
	var err error
	for {
		commit, err = w.popNewFromQueue()
		if err != nil {
			return nil, w.close(err)
		}

		w.visited[commit.Commit().Hash.Hex()] = struct{}{}

		if !w.isLimit(commit) {
			err = w.addToQueue(commit, commit.Commit().ParentHashes...)
			if err != nil {
				return nil, w.close(err)
			}
		}

		if w.isValid(commit) {
			return commit, nil
		}
	}
}

// ForEach runs the passed callback over each Commit returned by the CommitIter
// until the callback returns an error or there is no more commits to traverse.
func (w *filterCommitIter) ForEach(cb func(*CommitNode) error) error {
	for {
		commit, err := w.Next()
		if err == io.EOF {
			break
		}

		if err != nil {
			return err
		}

		err = cb(commit)
		if errors.Is(err, storer.ErrStop) {
			break
		}

		if err != nil {
			return err
		}
	}

	return nil
}

// Error returns the error that caused that the CommitIter is no longer returning commits
func (w *filterCommitIter) Error() error {
	return w.lastErr
}

// Close closes the CommitIter
func (w *filterCommitIter) Close() {
	w.visited = map[string]struct{}{}
	w.queue = []*CommitNode{}
	w.isLimit = nil
	w.isValid = nil
}

// close closes the CommitIter with an error
func (w *filterCommitIter) close(err error) error {
	w.Close()
	w.lastErr = err
	return err
}

// popNewFromQueue returns the first new commit from the internal fifo queue,
// or an io.EOF error if the queue is empty
func (w *filterCommitIter) popNewFromQueue() (*CommitNode, error) {
	var first *CommitNode
	for {
		if len(w.queue) == 0 {
			if w.lastErr != nil {
				return nil, w.lastErr
			}

			return nil, io.EOF
		}

		first = w.queue[0]
		w.queue = w.queue[1:]
		if _, ok := w.visited[first.Commit().Hash.Hex()]; ok {
			continue
		}

		return first, nil
	}
}

// addToQueue adds the passed commits to the internal fifo queue if they weren't seen
// or returns an error if the passed hashes could not be used to get valid commits
func (w *filterCommitIter) addToQueue(
	c *CommitNode,
	hashes ...hash.Hash,
) error {
	for _, hash := range hashes {
		if _, ok := w.visited[hash.Hex()]; ok {
			continue
		}

		commit, err := c.GetCommit(hash)
		if err != nil {
			return err
		}

		w.queue = append(w.queue, commit)
	}

	return nil
}
