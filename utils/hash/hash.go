// Package hash provides a way for managing the
// underlying hash implementations used across go-git.
package hash

import (
	"github.com/tmthrgd/go-hex"
)

type Hash []byte

func (hash Hash) Hex() string {
	return hex.EncodeToString(hash)
}

func (hash Hash) IsEmpty() bool {
	if hash == nil {
		return true
	}
	return len(hash) == 0
}
