package hash

import (
	"bytes"
	"testing"

	"github.com/stretchr/testify/require"
	"github.com/tmthrgd/go-hex"
)

func TestNewHasher(t *testing.T) {
	t.Run("single md5", func(t *testing.T) {
		hasher := NewHasher(Md5)
		_, err := hasher.Write([]byte{1, 2, 3, 4, 5})
		require.NoError(t, err)

		md5Hash := hasher.Md5.Sum(nil)
		require.Equal(t, "7cfdd07889b3295d6a550914ab35e068", Hash(md5Hash).Hex())
	})

	t.Run("single sha256", func(t *testing.T) {
		hasher := NewHasher(SHA256)
		_, err := hasher.Write([]byte{1, 2, 3, 4, 5})
		require.NoError(t, err)

		sha256Hash := hasher.Sha256.Sum(nil)
		require.Equal(t, "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0", Hash(sha256Hash).Hex())
	})

	t.Run("multi sha256", func(t *testing.T) {
		hasher := NewHasher(Md5, SHA256)
		_, err := hasher.Write([]byte{1, 2, 3, 4, 5})
		require.NoError(t, err)

		md5Hash := hasher.Md5.Sum(nil)
		require.Equal(t, "7cfdd07889b3295d6a550914ab35e068", Hash(md5Hash).Hex())

		sha256Hash := hasher.Sha256.Sum(nil)
		require.Equal(t, "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0", Hash(sha256Hash).Hex())
	})
}

func TestHashingReader_Read(t *testing.T) {
	origData := []byte{1, 2, 3, 4, 5, 6, 7}
	hashReader := NewHashingReader(bytes.NewReader(origData), Md5, SHA256)

	buf1 := make([]byte, 5)
	wLen, err := hashReader.Read(buf1)
	require.NoError(t, err)
	require.Equal(t, 5, wLen)
	require.Equal(t, origData[:5], buf1)

	md5Hash := hashReader.Md5.Sum(nil)
	require.Equal(t, "7cfdd07889b3295d6a550914ab35e068", hex.EncodeToString(md5Hash))
	sha256Hash := hashReader.Sha256.Sum(nil)
	require.Equal(t, "74f81fe167d99b4cb41d6d0ccda82278caee9f3e2f25d5e5a3936ff3dcec60d0", hex.EncodeToString(sha256Hash))

	buf2 := make([]byte, 5)
	wLen, err = hashReader.Read(buf2)
	require.NoError(t, err)
	require.Equal(t, 2, wLen)
	require.Equal(t, origData[5:], buf2[:2])

	md5Hash = hashReader.Md5.Sum(nil)
	require.Equal(t, "498001217bc632cb158588224d7d23c4", hex.EncodeToString(md5Hash))
	sha256Hash = hashReader.Sha256.Sum(nil)
	require.Equal(t, "32bbe378a25091502b2baf9f7258c19444e7a43ee4593b08030acd790bd66e6a", hex.EncodeToString(sha256Hash))
}
