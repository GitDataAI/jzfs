package versionmgr

import (
	"bytes"
	"errors"
	"fmt"
	"io"
	"sort"
	"strings"

	"github.com/go-git/go-git/v5/utils/merkletrie/noder"

	"github.com/go-git/go-git/v5/utils/merkletrie"
)

var (
	ErrActionNotMatch = errors.New("change action not match")
	ErrConflict       = errors.New("conflict dected but not found resolver")
)

type IChange interface {
	Action() (merkletrie.Action, error)
	From() noder.Path
	To() noder.Path
	Path() string
	String() string
}

var _ IChange = (*Change)(nil)

// Change wrap merkletrie changes for test
type Change struct {
	merkletrie.Change
}

// From return change from
func (c *Change) From() noder.Path {
	return c.Change.From
}

// To return change to
func (c *Change) To() noder.Path {
	return c.Change.To
}

// Path return change path
func (c *Change) Path() string {
	action, err := c.Action()
	if err != nil {
		panic(err)
	}

	var path string
	if action == merkletrie.Delete {
		path = c.Change.From.String()
	} else {
		path = c.Change.To.String()
	}

	return path
}

// Changes used to recored changes between commit, also provider iter function
type Changes struct {
	changes []IChange
	idx     int
}

// NewChanges create a change set
func NewChanges(changes []IChange) *Changes {
	sort.Slice(changes, func(i, j int) bool {
		return strings.Compare(changes[i].Path(), changes[j].Path()) < 0
	})

	return &Changes{changes: changes, idx: -1}
}

// Num return change number
func (c *Changes) Num() int {
	return len(c.changes)
}

// Index get change by array index
func (c *Changes) Index(idx int) IChange {
	return c.changes[idx]
}

// Changes return all changes
func (c *Changes) Changes() []IChange {
	return c.changes
}

// Next get element in array
func (c *Changes) Next() (IChange, error) {
	if c.idx < len(c.changes)-1 {
		c.idx++
		return c.changes[c.idx], nil
	}
	return nil, io.EOF
}

// Has check whether all element was consumed
func (c *Changes) Has() bool {
	return c.idx < len(c.changes)-1
}

func (c *Changes) ForEach(fn func(IChange) error) error {
	for _, change := range c.changes {
		err := fn(change)
		if err == nil {
			continue
		}
		if errors.Is(err, ErrStop) {
			return nil
		}
		return err
	}
	return nil
}

// Back a element in array
func (c *Changes) Back() {
	if c.idx > -1 {
		c.idx--
	}
}

// Reset result change iter
func (c *Changes) Reset() {
	c.idx = -1
}

func newChanges(mChanges merkletrie.Changes) *Changes {
	changes := make([]IChange, len(mChanges))
	for index, change := range mChanges {
		changes[index] = &Change{change}
	}
	return NewChanges(changes)
}

// ConflictResolver resolve conflict between two change
type ConflictResolver func(base IChange, merged IChange) (IChange, error)

// LeastHashResolve use least hash change for test
func LeastHashResolve(base IChange, merged IChange) (IChange, error) {
	baseAction, err := base.Action()
	if err != nil {
		return nil, err
	}

	mergeAction, err := merged.Action()
	if err != nil {
		return nil, err
	}

	if baseAction == merkletrie.Delete {
		return merged, nil
	}
	if mergeAction == merkletrie.Delete {
		return base, nil
	}

	if bytes.Compare(base.To().Hash(), merged.To().Hash()) < 0 {
		return base, nil
	}
	return merged, nil
}

// ChangesMergeIter walk two changes set and try to resolve
type ChangesMergeIter struct {
	baseChanges   *Changes
	mergerChanges *Changes
	resolver      ConflictResolver
}

// NewChangesMergeIter create a changes iter with two changes set and resolver function
func NewChangesMergeIter(baseChanges *Changes, mergerChanges *Changes, resolver ConflictResolver) *ChangesMergeIter {
	return &ChangesMergeIter{baseChanges: baseChanges, mergerChanges: mergerChanges, resolver: resolver}
}

// Has check if any changes
func (cw *ChangesMergeIter) Has() bool {
	return cw.baseChanges.Has() || cw.mergerChanges.Has()
}

// Reset reset changes
func (cw *ChangesMergeIter) Reset() {
	cw.baseChanges.Reset()
	cw.mergerChanges.Reset()
}

// Next find change file, first sort each file in change, pop files from two changes, compare filename,
//
//	base file < merge file, pop base change and put merge file back to queue
//	base file > merge file, pop merge file and put base file back to queue
//	both file name match, try to resolve file changes
//	if one of the queue consume up, pick left change in the other queue
func (cw *ChangesMergeIter) Next() (IChange, error) {
	baseNode, baseErr := cw.baseChanges.Next()
	if baseErr != nil && baseErr != io.EOF {
		return nil, baseErr
	}

	mergeNode, mergerError := cw.mergerChanges.Next()
	if mergerError != nil && mergerError != io.EOF {
		return nil, mergerError
	}

	if baseErr == io.EOF && mergerError == io.EOF {
		return nil, io.EOF
	}

	if baseErr == io.EOF {
		return mergeNode, nil
	}

	if mergerError == io.EOF {
		return baseNode, nil
	}

	compare := strings.Compare(baseNode.Path(), mergeNode.Path())
	if compare > 0 {
		//only merger change
		cw.baseChanges.Back()
		return mergeNode, nil
	} else if compare == 0 {
		return cw.compareBothChange(baseNode, mergeNode)
	}
	//only base change
	cw.mergerChanges.Back()
	return baseNode, nil
}

func (cw *ChangesMergeIter) compareBothChange(base, merge IChange) (IChange, error) {
	baseAction, err := base.Action()
	if err != nil {
		return nil, err
	}
	mergeAction, err := merge.Action()
	if err != nil {
		return nil, err
	}
	switch baseAction {
	case merkletrie.Insert:
		switch mergeAction {
		case merkletrie.Delete:
			return cw.resolveConflict(base, merge)
		case merkletrie.Modify:
			return nil, fmt.Errorf("%s merge should never be Modify while the other diff is Insert, must be a bug, fire issue at https://github.com/jiaozifs/jiaozifs/issues %w", base.Path(), ErrActionNotMatch)
		case merkletrie.Insert:
			if bytes.Equal(base.To().Hash(), merge.To().Hash()) {
				return base, nil
			}
			return cw.resolveConflict(base, merge)
		}
	case merkletrie.Delete:
		switch mergeAction {
		case merkletrie.Delete:
			return base, nil
		case merkletrie.Insert:
			return nil, fmt.Errorf("%s merge should never be Insert while the other diff is Delete, must be a bug, fire issue at https://github.com/jiaozifs/jiaozifs/issues %w", base.Path(), ErrActionNotMatch)
		case merkletrie.Modify:
			return cw.resolveConflict(base, merge)
		}
	case merkletrie.Modify:
		switch mergeAction {
		case merkletrie.Insert:
			return nil, fmt.Errorf("%s merge should never be Insert while the other diff is Modify, must be a bug, fire issue at https://github.com/jiaozifs/jiaozifs/issues %w", base.Path(), ErrActionNotMatch)
		case merkletrie.Delete:
			return cw.resolveConflict(base, merge)
		case merkletrie.Modify:
			if bytes.Equal(base.To().Hash(), merge.To().Hash()) {
				return base, nil
			}
			return cw.resolveConflict(base, merge)
		}
	}
	//should never come here
	return nil, ErrActionNotMatch
}

func (cw *ChangesMergeIter) resolveConflict(base, merge IChange) (IChange, error) {
	if cw.resolver != nil {
		resolveResult, err := cw.resolver(base, merge)
		if err != nil {
			return nil, err
		}
		return resolveResult, nil
	}
	return nil, fmt.Errorf("path %s confilict %w", merge.Path(), ErrConflict)
}
