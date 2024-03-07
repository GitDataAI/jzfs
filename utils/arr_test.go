package utils

import (
	"errors"
	"strconv"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestContain(t *testing.T) {

	t.Run("string", func(t *testing.T) {
		require.True(t, Contain([]string{"a", "b", "c"}, "a"))
		require.False(t, Contain([]string{"a", "b", "c"}, "d"))
	})

	t.Run("int", func(t *testing.T) {
		require.True(t, Contain([]int{1, 2, 3}, 1))
		require.False(t, Contain([]int{1, 2, 3}, 4))
	})

	t.Run("float", func(t *testing.T) {
		require.True(t, Contain([]float64{1.0, 2.0, 3.0}, 1.0))
		require.False(t, Contain([]float64{1.0, 2.0, 3.0}, 4.0))
	})

	t.Run("bool", func(t *testing.T) {
		require.True(t, Contain([]bool{true, false}, true))
		require.False(t, Contain([]bool{true, true}, false))
	})
}

func TestReverse(t *testing.T) {
	t.Run("empty", func(t *testing.T) {
		require.Len(t, Reverse([]string{}), 0)
	})

	t.Run("reverse", func(t *testing.T) {
		require.Equal(t, Reverse([]int{1, 2}), []int{2, 1})
	})

	t.Run("reverse", func(t *testing.T) {
		require.Equal(t, Reverse([]int{1, 2, 3, 4, 5}), []int{5, 4, 3, 2, 1})
	})
}

func TestArrMap(t *testing.T) {
	t.Run("success", func(t *testing.T) {
		result, err := ArrMap[int, string]([]int{1, 2, 3}, func(i int) (string, error) {
			return strconv.Itoa(i), nil
		})
		require.NoError(t, err)
		require.Equal(t, []string{"1", "2", "3"}, result)
	})

	t.Run("fail", func(t *testing.T) {
		var stopErr = errors.New("mock error")
		_, gotErr := ArrMap[int, string]([]int{1, 2, 3}, func(i int) (string, error) {
			return strconv.Itoa(i), stopErr
		})
		require.Equal(t, stopErr, gotErr)
	})
}
