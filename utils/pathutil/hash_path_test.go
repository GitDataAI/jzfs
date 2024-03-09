package pathutil

import (
	"encoding/hex"
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/GitDataAI/jiaozifs/utils/hash"
)

func TestPathOfHash(t *testing.T) {
	hashBytes, _ := hex.DecodeString("7cfdd07889b3295d6a550914ab35e068")
	require.Equal(t, "7c/fdd07889b3295d6a550914ab35e068", PathOfHash(hash.Hash(hashBytes)))
}
