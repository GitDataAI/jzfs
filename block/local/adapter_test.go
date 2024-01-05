package local_test

import (
	"path"
	"regexp"
	"testing"

	"github.com/jiaozifs/jiaozifs/block"
	"github.com/jiaozifs/jiaozifs/block/blocktest"
	"github.com/jiaozifs/jiaozifs/block/local"
	"github.com/stretchr/testify/require"
)

const testStorageNamespace = "local://test"

func TestLocalAdapter(t *testing.T) {
	tmpDir := t.TempDir()
	localPath := path.Join(tmpDir, "lakefs")
	externalPath := block.BlockstoreTypeLocal + "://" + path.Join(tmpDir, "lakefs", "external")
	adapter, err := local.NewAdapter(localPath, local.WithRemoveEmptyDir(false))
	if err != nil {
		t.Fatal("Failed to create new adapter", err)
	}
	blocktest.AdapterTest(t, adapter, testStorageNamespace, externalPath)
}

func TestAdapterNamespace(t *testing.T) {
	tmpDir := t.TempDir()
	localPath := path.Join(tmpDir, "lakefs")
	adapter, err := local.NewAdapter(localPath, local.WithRemoveEmptyDir(false))
	require.NoError(t, err, "create new adapter")
	expr, err := regexp.Compile(adapter.GetStorageNamespaceInfo().ValidityRegex)
	require.NoError(t, err)

	tests := []struct {
		Name      string
		Namespace string
		Success   bool
	}{
		{
			Name:      "valid_path",
			Namespace: "local://test/path/to/repo1",
			Success:   true,
		},
		{
			Name:      "invalid_path",
			Namespace: "~/test/path/to/repo1",
			Success:   false,
		},
		{
			Name:      "s3",
			Namespace: "s3://test/adls/core/windows/net",
			Success:   false,
		},
		{
			Name:      "invalid_string",
			Namespace: "this is a bad string",
			Success:   false,
		},
	}
	for _, tt := range tests {
		t.Run(tt.Name, func(t *testing.T) {
			require.Equal(t, tt.Success, expr.MatchString(tt.Namespace))
		})
	}
}

//func TestAdapter_Clean(t *testing.T) {
//	ctx := context.Background()
//	tmpDir := t.TempDir()
//	localPath := path.Join(tmpDir, "jiaozifs")
//	adapter, err := local.NewAdapter(localPath, local.WithRemoveEmptyDir(false))
//	require.NoError(t, err, "create new adapter")
//	expr, err := regexp.Compile(adapter.GetStorageNamespaceInfo().ValidityRegex)
//	require.NoError(t, err)
//
//	testData := "test data"
//	reader := strings.NewReader(testData)
//
//	hashingReader := hash.NewHashingReader(reader, hash.Md5)
//
//	tempf, err := os.CreateTemp("", "*")
//	require.NoError(t, err)
//
//	_, err = io.Copy(tempf, hashingReader)
//	require.NoError(t, err)
//
//	checkSum := hash.Hash(hashingReader.Md5.Sum(nil))
//
//	address := pathutil.PathOfHash(checkSum)
//	pointer := block.ObjectPointer{
//		StorageNamespace: "local://test/repo1",
//		Identifier:       address,
//		IdentifierType:   block.IdentifierTypeRelative,
//	}
//	err = adapter.Put(ctx, pointer, int64(len(testData)), tempf, block.PutOpts{})
//	require.NoError(t, err)
//	err = adapter.Clean(ctx, config.DefaultLocalBSPath, "local://test/repo1")
//	require.NoError(t, err)
//}
