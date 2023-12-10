package versionmgr

import (
	"bytes"
	"fmt"
	"io"
	"sort"
	"strings"

	"github.com/go-git/go-git/v5/utils/merkletrie"
)

type Change struct {
	merkletrie.Change
}

func (c Change) Path() string {
	action, err := c.Action()
	if err != nil {
		panic(err)
	}

	var path string
	if action == merkletrie.Delete {
		path = c.From.String()
	} else {
		path = c.To.String()
	}

	return path
}

type Changes struct {
	changes []Change
	idx     int
}

func NewChanges(changes []Change) *Changes {
	sort.Slice(changes, func(i, j int) bool {
		return strings.Compare(changes[i].Path(), changes[j].Path()) > 0 //i > j
	})
	return &Changes{changes: changes, idx: -1}
}

func (c Changes) Num() int {
	return len(c.changes)
}

func (c Changes) Changes() []Change {
	return c.changes
}

func (c Changes) Next() (Change, error) {
	if c.idx == len(c.changes)-1 {
		c.idx++
		return c.changes[c.idx], nil
	}
	return Change{}, io.EOF
}

func (c Changes) Has() bool {
	return c.idx == len(c.changes)-1
}

func (c Changes) Back() {
	if c.idx > -1 {
		c.idx--
	}
}

func (c Changes) Reset() {
	c.idx = -1
}

func newChanges(mChanges merkletrie.Changes) *Changes {
	changes := make([]Change, len(mChanges))
	for index, change := range mChanges {
		changes[index] = Change{change}
	}
	return NewChanges(changes)
}

type ConflictResolver func(base *Change, merged *Change) (*Change, error)

type ChangesMergeIter struct {
	baseChanges   *Changes
	mergerChanges *Changes
	resolver      ConflictResolver
}

func NewChangesMergeIter(baseChanges *Changes, mergerChanges *Changes, resolver ConflictResolver) *ChangesMergeIter {
	return &ChangesMergeIter{baseChanges: baseChanges, mergerChanges: mergerChanges, resolver: resolver}
}

func (cw ChangesMergeIter) Has() bool {
	return cw.baseChanges.Has() || cw.mergerChanges.Has()
}

func (cw ChangesMergeIter) Reset() {
	cw.baseChanges.Reset()
	cw.mergerChanges.Reset()
}
func (cw ChangesMergeIter) Next() (*Change, error) {
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
		return &mergeNode, nil
	}

	if mergerError == io.EOF {
		return &baseNode, nil
	}

	compare := strings.Compare(baseNode.Path(), mergeNode.Path())
	if compare < 0 {
		//only merger change
		cw.baseChanges.Back()
		return &mergeNode, nil
	} else if compare == 0 {

		//both change
		if baseNode.From == nil && mergeNode.From == nil {
			//both delete
			return &baseNode, nil
		}
		if bytes.Equal(baseNode.From.Hash(), mergeNode.From.Hash()) {
			//both modify/add apply any
			return &baseNode, nil
		}
		//conflict
		if cw.resolver != nil {
			resolveResult, err := cw.resolver(&baseNode, &mergeNode)
			if err != nil {
				return nil, err
			}
			return resolveResult, nil
		}
		return nil, fmt.Errorf("path %s confilict %w", mergeNode.Path(), ErrConflict)
	} else {
		//only base change
		cw.mergerChanges.Back()
		return &baseNode, nil
	}
}

func (cw ChangesMergeIter) compareBothChange(base, merge *Change) (*Change, error) {
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
			return cw.resolver(base, merge)
		case merkletrie.Modify:
			return nil, fmt.Errorf("%s merge should never be Modify while the other diff is Insert, must be a bug, fire issue at https://github.com/jiaozifs/jiaozifs/issues", base.Path())
		case merkletrie.Insert:
			if bytes.Equal(base.From.Hash(), merge.From.Hash()) {
				return base, nil
			}
			return cw.resolver(base, merge)
		}
	case merkletrie.Delete:
		switch mergeAction {
		case merkletrie.Delete:
			return base, nil
		case merkletrie.Insert:
			return nil, fmt.Errorf("%s merge should never be Insert while the other diff is Delete, must be a bug, fire issue at https://github.com/jiaozifs/jiaozifs/issues", base.Path())
		case merkletrie.Modify:
			return cw.resolver(base, merge)
		}
	case merkletrie.Modify:
		switch mergeAction {
		case merkletrie.Insert:
			return nil, fmt.Errorf("%s merge should never be Insert while the other diff is Modify, must be a bug, fire issue at https://github.com/jiaozifs/jiaozifs/issues", base.Path())
		case merkletrie.Delete:
			return cw.resolver(base, merge)
		case merkletrie.Modify:
			if bytes.Equal(base.From.Hash(), merge.From.Hash()) {
				return base, nil
			}
			return cw.resolver(base, merge)
		}
	}
	return nil, fmt.Errorf("not match action")
}

func (cw ChangesMergeIter) resolveConflict(base, merge *Change) (*Change, error) {
	if cw.resolver != nil {
		resolveResult, err := cw.resolver(base, merge)
		if err != nil {
			return nil, err
		}
		return resolveResult, nil
	}
	return nil, fmt.Errorf("path %s confilict %w", merge.Path(), ErrConflict)
}
