package pathutil

import (
	"path"

	"github.com/jiaozifs/jiaozifs/utils/hash"
)

func PathOfHash(hash hash.Hash) string {
	hex := hash.Hex()
	return path.Join(hex[:2], hex[2:])
}
