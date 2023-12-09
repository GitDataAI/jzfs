package hash

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestHashJSON(t *testing.T) {
	type A struct {
		H Hash
	}

	t.Run("success", func(t *testing.T) {
		data, err := json.Marshal(A{H: Hash("aaaa")})
		require.NoError(t, err)
		require.Equal(t, "{\"H\":\"61616161\"}", string(data))

		a := A{}
		err = json.Unmarshal(data, &a)
		require.NoError(t, err)
		require.Equal(t, "aaaa", string(a.H))
	})
	t.Run("null", func(t *testing.T) {
		data, err := json.Marshal(A{})
		require.NoError(t, err)
		require.Equal(t, "{\"H\":\"\"}", string(data))

		a := A{}
		err = json.Unmarshal(data, &a)
		require.NoError(t, err)
		require.Equal(t, "", string(a.H))
	})
}
