package versionmgr

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"os"
	"path"
	"testing"

	bserv "github.com/ipfs/boxo/blockservice"
	bstore "github.com/ipfs/boxo/blockstore"
	dag "github.com/ipfs/boxo/ipld/merkledag"
	"github.com/ipfs/boxo/mfs"
	ds "github.com/ipfs/go-datastore"
	dssync "github.com/ipfs/go-datastore/sync"
	offline "github.com/ipfs/go-ipfs-exchange-offline"

	"github.com/ipld/go-car"

	"github.com/stretchr/testify/require"

	"github.com/GitDataAI/jiaozifs/utils"

	"github.com/GitDataAI/jiaozifs/models"
)

type mockWalker struct {
	dirs  []string
	files map[string][]byte
}

func (wk mockWalker) Walk(_ context.Context, fn func(entry *models.TreeEntry, blob *models.Blob, path string) error) error {
	for _, dir := range wk.dirs {
		err := fn(&models.TreeEntry{
			IsDir: true,
		}, nil, dir)
		if err != nil {
			return err
		}
	}
	for path, data := range wk.files {
		err := fn(&models.TreeEntry{
			IsDir: false,
		}, &models.Blob{
			Size: int64(len(data)),
		}, path)
		if err != nil {
			return err
		}
	}
	return nil
}

func TestRepoArchiver_ArchiveZip(t *testing.T) {
	ctx := context.Background()
	wk := &mockWalker{
		dirs: []string{
			"a",
			"a/b",
			"m",
		},
		files: map[string][]byte{
			"1.txt":     []byte("111111111111111111111111"),
			"a/2.txt":   []byte("222222222222222222222222"),
			"a/3.txt":   []byte("3333333333333333333333333"),
			"a/b/4.txt": []byte("4444444444444444444444444444"),
			"m/5.txt":   []byte("555555555555555555555"),
		},
	}
	archiver := NewRepoArchiver(
		"testdir",
		wk,
		func(ctx context.Context, _ *models.Blob, s string) (io.ReadCloser, error) {
			data, ok := wk.files[s]
			if !ok {
				return nil, fmt.Errorf("data not found %s", s)
			}
			return utils.CloserWraper{Reader: bytes.NewReader(data)}, nil
		},
	)

	tmpDir, err := os.MkdirTemp(os.TempDir(), "*")
	require.NoError(t, err)
	tmpFile := path.Join(tmpDir, "test.zip")

	err = archiver.ArchiveZip(ctx, tmpFile)
	require.NoError(t, err)

	fmt.Println(tmpFile)
}

func TestRepoArchiver_ArchiveCar(t *testing.T) {
	ctx := context.Background()
	wk := &mockWalker{
		dirs: []string{
			"a",
			"a/b",
			"m",
		},
		files: map[string][]byte{
			"1.txt":     []byte("111111111111111111111111"),
			"a/2.txt":   []byte("222222222222222222222222"),
			"a/3.txt":   []byte("3333333333333333333333333"),
			"a/b/4.txt": []byte("4444444444444444444444444444"),
			"m/5.txt":   []byte("555555555555555555555"),
		},
	}
	archiver := NewRepoArchiver(
		"testdir",
		wk,
		func(ctx context.Context, _ *models.Blob, s string) (io.ReadCloser, error) {
			data, ok := wk.files[s]
			if !ok {
				return nil, fmt.Errorf("data not found %s", s)
			}
			return utils.CloserWraper{Reader: bytes.NewReader(data)}, nil
		},
	)

	tmpDir, err := os.MkdirTemp(os.TempDir(), "*")
	require.NoError(t, err)
	tmpFile := path.Join(tmpDir, "test.car")

	err = archiver.ArchiveCar(ctx, tmpFile)
	require.NoError(t, err)

	//check data in car
	fs, err := os.Open(tmpFile)
	require.NoError(t, err)

	db := dssync.MutexWrap(ds.NewMapDatastore())
	bs := bstore.NewBlockstore(db)
	header, err := car.LoadCar(ctx, bs, fs)
	require.NoError(t, err)
	blockserv := bserv.New(bs, offline.Exchange(bs))
	dagSrv := dag.NewDAGService(blockserv)

	rootNode, err := dagSrv.Get(ctx, header.Roots[0])
	require.NoError(t, err)
	rd, err := dag.DecodeProtobuf(rootNode.RawData())
	require.NoError(t, err)

	root, err := mfs.NewRoot(ctx, dagSrv, rd, nil)
	require.NoError(t, err)
	for file, data := range wk.files {
		actualData, err := readFile(root, path.Join(archiver.rootPath, file), 0)
		require.NoError(t, err)
		require.Equal(t, data, actualData)
	}
}

func readFile(rt *mfs.Root, path string, offset int64) ([]byte, error) {
	n, err := mfs.Lookup(rt, path)
	if err != nil {
		return nil, err
	}

	fi, ok := n.(*mfs.File)
	if !ok {
		return nil, fmt.Errorf("%s was not a file", path)
	}

	fd, err := fi.Open(mfs.Flags{Read: true})
	if err != nil {
		return nil, err
	}

	_, err = fd.Seek(offset, io.SeekStart)
	if err != nil {
		return nil, err
	}
	defer fd.Close() //nolint:errcheck
	return io.ReadAll(fd)
}
