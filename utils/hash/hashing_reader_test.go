package hash

import (
	"bytes"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestNewHasher(t *testing.T) {
	t.Run("single md5", func(t *testing.T) {
		hasher := NewHasher(HashFunctionMD5)
		hasher.Write([]byte{1, 2, 3, 4, 5})
		md5Hash := hasher.Md5.Sum(nil)
		require.Equal(t, "7cfdd07889b3295d6a550914ab35e068", Hash(md5Hash).Hex())
	})

	t.Run("single sha256", func(t *testing.T) {
		hasher := NewHasher(HashFunctionSHA256)
		hasher.Write([]byte{1, 2, 3, 4, 5})
		sha256Hash := hasher.Sha256.Sum(nil)
		require.Equal(t, "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0", Hash(sha256Hash).Hex())
	})

	t.Run("multi sha256", func(t *testing.T) {
		hasher := NewHasher(HashFunctionMD5, HashFunctionSHA256)
		hasher.Write([]byte{1, 2, 3, 4, 5})

		md5Hash := hasher.Md5.Sum(nil)
		require.Equal(t, "7cfdd07889b3295d6a550914ab35e068", Hash(md5Hash).Hex())

		sha256Hash := hasher.Sha256.Sum(nil)
		require.Equal(t, "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0", Hash(sha256Hash).Hex())
	})
}

func TestHashingReader_Read(t *testing.T) {
	origData := []byte{1, 2, 3, 4, 5, 6, 7}
	hasher := NewHashingReader(bytes.NewReader(origData), HashFunctionMD5, HashFunctionSHA256)

	buf1 := make([]byte, 5)
	wLen, err := hasher.Read(buf1)
	require.NoError(t, err)
	require.Equal(t, 5, wLen)

	md5Hash := hasher.Md5.Sum(nil)
	require.Equal(t, "7cfdd07889b3295d6a550914ab35e068", Hash(md5Hash).Hex())

	sha256Hash := hasher.Sha256.Sum(nil)
	require.Equal(t, "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0", Hash(sha256Hash).Hex())

	buf2 := make([]byte, 5)
	wLen, err = hasher.Read(buf2)
	require.NoError(t, err)
	require.Equal(t, 2, wLen)

	md5Hash = hasher.Md5.Sum(nil)
	require.Equal(t, "498001217bc632cb158588224d7d23c4", Hash(md5Hash).Hex())

	sha256Hash = hasher.Sha256.Sum(nil)
	require.Equal(t, "32bbe378a25091502b2baf9f7258c19444e7a43ee4593b08030acd790bd66e6a", Hash(sha256Hash).Hex())

	require.Equal(t, origData[:5], buf1)
	require.Equal(t, origData[5:], buf2[:2])
}
