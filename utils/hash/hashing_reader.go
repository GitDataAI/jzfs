package hash

import (
	"crypto/md5" //nolint:gosec
	"crypto/sha256"
	"encoding/binary"
	"hash"
	"io"
	"strconv"
)

type HashType int //nolint

const (
	Md5 HashType = iota
	SHA256
)

type Hasher struct {
	Md5    hash.Hash
	Sha256 hash.Hash
}

func NewHasher(hashTypes ...HashType) *Hasher {
	s := new(Hasher)
	for _, hashType := range hashTypes {
		switch hashType {
		case Md5:
			if s.Md5 == nil {
				s.Md5 = md5.New() //nolint:gosec
			}
		case SHA256:
			if s.Sha256 == nil {
				s.Sha256 = sha256.New()
			}
		default:
			panic("wrong hash type number " + strconv.Itoa(int(hashType)))
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

func (hasher *Hasher) WriteInt8(data int8) error {
	_, err := hasher.Write([]byte{uint8(data)})
	return err
}

func (hasher *Hasher) WriteUint8(data uint) error {
	_, err := hasher.Write([]byte{byte(data)})
	return err
}

func (hasher *Hasher) WriteString(data string) error {
	_, err := hasher.Write([]byte(data))
	return err
}

func (hasher *Hasher) WriteInt32(data int32) error {
	buf := [4]byte{}
	binary.BigEndian.PutUint32(buf[:], uint32(data))
	_, err := hasher.Write(buf[:])
	return err
}

func (hasher *Hasher) WriteUint32(data uint32) error {
	buf := [4]byte{}
	binary.BigEndian.PutUint32(buf[:], data)
	_, err := hasher.Write(buf[:])
	return err
}

func (hasher *Hasher) WritInt64(data int64) error {
	buf := [8]byte{}
	binary.BigEndian.PutUint64(buf[:], uint64(data))
	_, err := hasher.Write(buf[:])
	return err
}

func (hasher *Hasher) WritUint64(data uint64) error {
	buf := [4]byte{}
	binary.BigEndian.PutUint64(buf[:], data)
	_, err := hasher.Write(buf[:])
	return err
}

type HashingReader struct {
	*Hasher
	originalReader io.Reader
	CopiedSize     int64
}

func (s *HashingReader) Read(p []byte) (int, error) {
	nb, err := s.originalReader.Read(p)
	s.CopiedSize += int64(nb)
	if s.Md5 != nil {
		if _, err2 := s.Md5.Write(p[0:nb]); err2 != nil {
			return nb, err2
		}
	}
	if s.Sha256 != nil {
		if _, err2 := s.Sha256.Write(p[0:nb]); err2 != nil {
			return nb, err2
		}
	}
	return nb, err
}

func NewHashingReader(body io.Reader, hashTypes ...HashType) *HashingReader {
	s := new(HashingReader)
	s.originalReader = body
	s.Hasher = NewHasher(hashTypes...)
	return s
}
