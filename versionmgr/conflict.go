package versionmgr

import (
	"bytes"
	"fmt"

	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/jiaozifs/jiaozifs/versionmgr/merkletrie"
)

// ConflictResolver resolve conflict between two change
type ConflictResolver func(left IChange, right IChange) (IChange, error)

// LeastHashResolve use the least hash change for test
func LeastHashResolve(left IChange, right IChange) (IChange, error) {
	leftAction, err := left.Action()
	if err != nil {
		return nil, err
	}

	rightAction, err := right.Action()
	if err != nil {
		return nil, err
	}

	if leftAction == merkletrie.Delete {
		return right, nil
	}
	if rightAction == merkletrie.Delete {
		return left, nil
	}

	if bytes.Compare(left.To().Hash(), right.To().Hash()) < 0 {
		return left, nil
	}
	return right, nil
}

func ForbidResolver(_ IChange, _ IChange) (IChange, error) {
	return nil, fmt.Errorf("not allow conflict in this mode")
}

type BaseChange struct {
	Action merkletrie.Action
	Hash   hash.Hash
}

type Conflict struct {
	Path  string
	Left  BaseChange
	Right BaseChange
}

func OneSideResolver(useLeft bool) ConflictResolver {
	return func(left IChange, right IChange) (IChange, error) {
		if useLeft {
			return left, nil
		}
		return right, nil
	}
}

func ResolveFromSelector(resolveMsg map[string]string) ConflictResolver {
	return func(left IChange, right IChange) (IChange, error) {
		if resolveMsg[left.Path()] == "left" {
			return left, nil
		}
		return right, nil
	}
}
