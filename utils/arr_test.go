package utils

import (
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
