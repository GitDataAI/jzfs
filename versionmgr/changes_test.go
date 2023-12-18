package versionmgr

import (
	"bytes"
	"fmt"
	"path/filepath"
	"strconv"
	"strings"
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/versionmgr/merkletrie"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/versionmgr/merkletrie/noder"
)

var _ noder.Noder = (*MockNode)(nil)

type MockNode struct {
	name string
	hash hash.Hash
}

func NewMockNode(name string, hash hash.Hash) *MockNode {
	return &MockNode{name: name, hash: hash}
}

func (m MockNode) Hash() []byte {
	return m.hash
}

func (m MockNode) Equal(other noder.Noder) bool {
	return bytes.Equal(m.Hash(), other.Hash())
}

func (m MockNode) String() string {
	//TODO implement me
	panic("implement me")
}

func (m MockNode) Name() string {
	return m.name
}

func (m MockNode) IsDir() bool {
	//TODO implement me
	panic("implement me")
}

func (m MockNode) Children() ([]noder.Noder, error) {
	//TODO implement me
	panic("implement me")
}

func (m MockNode) NumChildren() (int, error) {
	//TODO implement me
	panic("implement me")
}

func (m MockNode) Skip() bool {
	//TODO implement me
	panic("implement me")
}

var _ IChange = (*mockChange)(nil)

type mockChange struct {
	action merkletrie.Action
	path   string
	from   noder.Path
	to     noder.Path
}

func (m mockChange) Action() (merkletrie.Action, error) {
	return m.action, nil
}

func (m mockChange) From() noder.Path {
	return m.from
}

func (m mockChange) To() noder.Path {
	return m.to
}

func (m mockChange) Path() string {
	return m.path
}

func (m mockChange) String() string {
	//TODO implement me
	panic("implement me")
}

func makeMockChange(testData string) (*Changes, error) {
	var changes []IChange
	for _, line := range strings.Split(testData, "\n") {
		line = strings.TrimSpace(line)
		if len(line) == 0 {
			continue
		}
		segs := strings.Split(line, "|")
		num, err := strconv.Atoi(segs[0])
		if err != nil {
			return nil, err
		}
		action := merkletrie.Action(num)
		pHash := hash.Hash(strings.Trim(segs[2], " \t/"))
		fullPath := filepath.Clean(strings.Trim(segs[1], " \t/"))
		pathSeg := strings.Split(fullPath, "/")
		path := make([]noder.Noder, len(pathSeg))
		for index, p := range pathSeg {
			path[index] = NewMockNode(p, pHash)
		}
		c := &mockChange{
			action: action,
			path:   fullPath,
		}
		switch action {
		case merkletrie.Delete:
			c.from = path
			c.to = nil
		case merkletrie.Insert:
			c.from = nil
			c.to = path
		case merkletrie.Modify:
			c.from = nil
			c.to = path
		}

		changes = append(changes, c)
	}

	return NewChanges(changes), nil
}

func TestNewChangesMergeIter(t *testing.T) {
	t.Run("simple just add", func(t *testing.T) {
		changeData1 := `
1|a.txt|h1
1|b/a.txt|h2
`
		changeSet1, err := makeMockChange(changeData1)
		require.NoError(t, err)
		changeData2 := `
1|c.txt|h1
1|d/a.txt|h2
`
		changeSet2, err := makeMockChange(changeData2)
		require.NoError(t, err)
		iter := NewChangesMergeIter(changeSet1, changeSet2, nil)

		var finalChjange []IChange
		for iter.Has() {
			change, err := iter.Next()
			require.NoError(t, err)
			finalChjange = append(finalChjange, change)
		}
		require.Len(t, finalChjange, 4)
		require.Equal(t, "a.txt", finalChjange[0].Path())
		require.Equal(t, "b/a.txt", finalChjange[1].Path())
		require.Equal(t, "c.txt", finalChjange[2].Path())
		require.Equal(t, "d/a.txt", finalChjange[3].Path())
	})

	t.Run("keep same hash", func(t *testing.T) {
		changeData1 := `
1|a.txt|h1
2|b/d.txt|h2
3|b/a.txt|h2
`
		changeSet1, err := makeMockChange(changeData1)
		require.NoError(t, err)
		changeData2 := `
1|a.txt|h1
2|b/d.txt|h2
3|b/a.txt|h2
`
		changeSet2, err := makeMockChange(changeData2)
		require.NoError(t, err)
		iter := NewChangesMergeIter(changeSet1, changeSet2, nil)

		var finalChjange []IChange
		for iter.Has() {
			change, err := iter.Next()
			require.NoError(t, err)
			finalChjange = append(finalChjange, change)
		}
		require.Len(t, finalChjange, 3)
		require.Equal(t, "a.txt", finalChjange[0].Path())
		require.Equal(t, "b/a.txt", finalChjange[1].Path())
		require.Equal(t, "b/d.txt", finalChjange[2].Path())
	})

	t.Run("error while add conflict", func(t *testing.T) {
		changeData1 := `
1|a.txt|h1
`
		changeSet1, err := makeMockChange(changeData1)
		require.NoError(t, err)
		changeData2 := `
1|a.txt|h2
`
		changeSet2, err := makeMockChange(changeData2)
		require.NoError(t, err)
		iter := NewChangesMergeIter(changeSet1, changeSet2, nil)

		for iter.Has() {
			_, err = iter.Next()
			require.ErrorIs(t, err, ErrConflict)
		}
	})

	t.Run("error while modify conflict", func(t *testing.T) {
		changeData1 := `
3|a.txt|h1
`
		changeSet1, err := makeMockChange(changeData1)
		require.NoError(t, err)
		changeData2 := `
3|a.txt|h2
`
		changeSet2, err := makeMockChange(changeData2)
		require.NoError(t, err)
		iter := NewChangesMergeIter(changeSet1, changeSet2, nil)

		for iter.Has() {
			_, err = iter.Next()
			require.ErrorIs(t, err, ErrConflict)
		}
	})

	t.Run("not conflict while both delete", func(t *testing.T) {
		changeData1 := `
2|a.txt|h1
`
		changeSet1, err := makeMockChange(changeData1)
		require.NoError(t, err)
		changeData2 := `
2|a.txt|h2
`
		changeSet2, err := makeMockChange(changeData2)
		require.NoError(t, err)
		iter := NewChangesMergeIter(changeSet1, changeSet2, nil)

		var finalChjange []IChange
		for iter.Has() {
			change, err := iter.Next()
			require.NoError(t, err)
			finalChjange = append(finalChjange, change)
		}
		require.Len(t, finalChjange, 1)
		require.Equal(t, "a.txt", finalChjange[0].Path())
	})

	t.Run("resovler select conflict", func(t *testing.T) {
		changeData1 := `
1|a.txt|h1
2|b.txt|h3
`
		changeSet1, err := makeMockChange(changeData1)
		require.NoError(t, err)
		changeData2 := `
1|a.txt|h2
2|b.txt|h4
`
		changeSet2, err := makeMockChange(changeData2)
		require.NoError(t, err)
		iter := NewChangesMergeIter(changeSet1, changeSet2, LeastHashResolve)

		var finalChjange []IChange
		for iter.Has() {
			change, err := iter.Next()
			require.NoError(t, err)
			finalChjange = append(finalChjange, change)
		}
		require.Len(t, finalChjange, 2)
		require.Equal(t, "a.txt", finalChjange[0].Path())
		require.Equal(t, "h1", string(finalChjange[0].To().Hash()))

		require.Equal(t, "b.txt", finalChjange[1].Path())
		require.Equal(t, "h3", string(finalChjange[1].From().Hash()))
	})
}

func TestChanges_ForEach(t *testing.T) {
	changeData1 := `
1|c.txt|h3
1|a.txt|h1
1|b.txt|h2
`
	changeSet, err := makeMockChange(changeData1)
	require.NoError(t, err)

	t.Run("simple", func(t *testing.T) {
		var path []string
		err := changeSet.ForEach(func(change IChange) error {
			path = append(path, change.Path())
			return nil
		})
		require.NoError(t, err)
		require.Equal(t, []string{"a.txt", "b.txt", "c.txt"}, path)
	})

	t.Run("check stop", func(t *testing.T) {
		var path []string
		err := changeSet.ForEach(func(change IChange) error {
			path = append(path, change.Path())
			if change.Path() == "b.txt" {
				return ErrStop
			}
			return nil
		})
		require.NoError(t, err)
		require.Equal(t, []string{"a.txt", "b.txt"}, path)
	})
	t.Run("error check", func(t *testing.T) {
		var path []string
		var stopErr = fmt.Errorf("stop at b,txt")
		err := changeSet.ForEach(func(change IChange) error {
			path = append(path, change.Path())
			if change.Path() == "b.txt" {
				return stopErr
			}
			return nil
		})
		require.ErrorIs(t, err, stopErr)
		require.Equal(t, []string{"a.txt", "b.txt"}, path)
	})
}
