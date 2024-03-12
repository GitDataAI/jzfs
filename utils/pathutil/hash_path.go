package pathutil

import (
	"path"

	"github.com/GitDataAI/jiaozifs/utils/hash"
)

func PathOfHash(hash hash.Hash) string {
	hex := hash.Hex()
	return path.Join(hex[:2], hex[2:])
}
