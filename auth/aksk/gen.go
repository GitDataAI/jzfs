package aksk

import (
	"crypto/rand"
	"encoding/hex"
	"io"
)

func GenerateAksk() (string, string, error) {
	akBytes, err := io.ReadAll(io.LimitReader(rand.Reader, 16))
	if err != nil {
		return "", "", err
	}
	ak := hex.EncodeToString(akBytes)

	skBytes, err := io.ReadAll(io.LimitReader(rand.Reader, 16))
	if err != nil {
		return "", "", err
	}
	sk := hex.EncodeToString(skBytes)
	return ak, sk, nil
}
