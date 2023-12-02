package hash

import (
	"crypto/md5" //nolint:gosec
	"crypto/sha256"
	"hash"
	"io"
	"strconv"
)

const (
	HashFunctionMD5 = iota
	HashFunctionSHA256
)

type Hasher struct {
	Md5    hash.Hash
	Sha256 hash.Hash
}

func NewHasher(hashTypes ...int) *Hasher {
	s := new(Hasher)
	for _, hashType := range hashTypes {
		switch hashType {
		case HashFunctionMD5:
			if s.Md5 == nil {
				s.Md5 = md5.New() //nolint:gosec
			}
		case HashFunctionSHA256:
			if s.Sha256 == nil {
				s.Sha256 = sha256.New()
			}
		default:
			panic("wrong hash type number " + strconv.Itoa(hashType))
		}
	}
	return s
}

func (hasher *Hasher) Write(data []byte) (int, error) {
	if hasher.Md5 != nil {
		if _, err := hasher.Md5.Write(data); err != nil {
			return 0, err
		}
	}
	if hasher.Sha256 != nil {
		if _, err := hasher.Sha256.Write(data); err != nil {
			return 0, err
		}
	}
	return len(data), nil
}

type HashingReader struct {
	*Hasher
	originalReader io.Reader
	CopiedSize     int64
}

func (s *HashingReader) Read(p []byte) (int, error) {
	nb, err := s.originalReader.Read(p)
	if err != nil {
		return nb, err
	}
	s.CopiedSize += int64(nb)
	_, err = s.Hasher.Write(p[0:nb])
	return nb, err
}

func NewHashingReader(body io.Reader, hashTypes ...int) *HashingReader {
	s := new(HashingReader)
	s.originalReader = body
	s.Hasher = NewHasher(hashTypes...)
	return s
}
