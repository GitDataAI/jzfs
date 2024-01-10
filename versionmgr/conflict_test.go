package versionmgr

import (
	"testing"

	"github.com/jiaozifs/jiaozifs/versionmgr/merkletrie"

	"github.com/stretchr/testify/require"
)

func TestLeastHashResolve(t *testing.T) {
	t.Run("select least hash", func(t *testing.T) {
		ch1, err := makeMockChange("1|a.txt|h1")
		require.NoError(t, err)
		ch2, err := makeMockChange("1|a.txt|h2")
		require.NoError(t, err)

		chSelect, err := LeastHashResolve(ch1, ch2)
		require.NoError(t, err)
		require.Equal(t, chSelect.To().Hash(), ch1.To().Hash())
	})
	t.Run("left is delete", func(t *testing.T) {
		ch1, err := makeMockChange("2|a.txt|h1")
		require.NoError(t, err)
		ch2, err := makeMockChange("1|a.txt|h2")
		require.NoError(t, err)

		chSelect, err := LeastHashResolve(ch1, ch2)
		require.NoError(t, err)
		require.Equal(t, chSelect.To().Hash(), ch2.To().Hash())
	})
	t.Run("right is delete", func(t *testing.T) {
		ch1, err := makeMockChange("1|a.txt|h1")
		require.NoError(t, err)
		ch2, err := makeMockChange("2|a.txt|h2")
		require.NoError(t, err)

		chSelect, err := LeastHashResolve(ch1, ch2)
		require.NoError(t, err)
		require.Equal(t, chSelect.To().Hash(), ch1.To().Hash())
	})
}

func TestForbidResolver(t *testing.T) {
	_, err := ForbidResolver(nil, nil)
	require.Error(t, err)
}

func TestOneSideResolver(t *testing.T) {
	t.Run("select left", func(t *testing.T) {
		resolver := OneSideResolver(true)
		ch1, err := makeMockChange("1|a.txt|h1")
		require.NoError(t, err)
		ch2, err := makeMockChange("1|a.txt|h2")
		require.NoError(t, err)

		chSelect, err := resolver(ch1, ch2)
		require.NoError(t, err)
		require.Equal(t, chSelect.To().Hash(), ch1.To().Hash())
	})
	t.Run("select right", func(t *testing.T) {
		resolver := OneSideResolver(false)
		ch1, err := makeMockChange("1|a.txt|h1")
		require.NoError(t, err)
		ch2, err := makeMockChange("1|a.txt|h2")
		require.NoError(t, err)

		chSelect, err := resolver(ch1, ch2)
		require.NoError(t, err)
		require.Equal(t, chSelect.To().Hash(), ch2.To().Hash())
	})
}

func TestResolveFromSelector(t *testing.T) {
	resolver := ResolveFromSelector(map[string]string{
		"a.txt": "left",
		"b.txt": "right",
		"c.txt": "right",
	})
	changeData1 := `
1|a.txt|h1
3|b.txt|h3
3|c.txt|h5
`
	changeSet1, err := makeMockChanges(changeData1)
	require.NoError(t, err)
	changeData2 := `
1|a.txt|h2
3|b.txt|h4
2|c.txt|h1
`
	changeSet2, err := makeMockChanges(changeData2)
	require.NoError(t, err)
	iter := NewChangesPairIter(changeSet1, changeSet2)

	{
		change, err := iter.Next()
		require.NoError(t, err)
		selectCh, err := resolver(change.Left, change.Right)
		require.NoError(t, err)
		selectAct, err := selectCh.Action()
		require.NoError(t, err)
		require.Equal(t, merkletrie.Insert, selectAct)
		require.Equal(t, "h1", string(selectCh.To().Hash()))
	}
	{
		change, err := iter.Next()
		require.NoError(t, err)
		selectCh, err := resolver(change.Left, change.Right)
		require.NoError(t, err)
		selectAct, err := selectCh.Action()
		require.NoError(t, err)
		require.Equal(t, merkletrie.Modify, selectAct)
		require.Equal(t, "h4", string(selectCh.To().Hash()))
	}
	{
		change, err := iter.Next()
		require.NoError(t, err)
		selectCh, err := resolver(change.Left, change.Right)
		require.NoError(t, err)
		selectAct, err := selectCh.Action()
		require.NoError(t, err)
		require.Equal(t, merkletrie.Delete, selectAct)
	}
	require.False(t, iter.Has())
}
