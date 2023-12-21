package versionmgr

import (
	"bytes"
	"context"
	"errors"
	"io"
	"testing"

	"github.com/google/uuid"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/block/mem"
	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/testhelper"
)

func TestTreeWriteBlob(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repoID := uuid.New()
	adapter := mem.New(ctx)
	namespace := "mem://data"
	objRepo := models.NewFileTree(db, repoID)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := workTree.WriteBlob(ctx, adapter, namespace, r, bLen, models.DefaultLeafProperty())
	require.NoError(t, err)
	assert.Equal(t, bLen, blob.Size)
	assert.Equal(t, "99b91d4c517d0cded9506be9298b8d02", blob.Hash.Hex())
	assert.Equal(t, "f3b39786b86a96372589aa1166966643", blob.CheckSum.Hex())

	reader, err := workTree.ReadBlob(ctx, adapter, namespace, blob, nil)
	require.NoError(t, err)
	content, err := io.ReadAll(reader)
	require.NoError(t, err)
	require.Equal(t, binary, content)
}

func TestWorkTreeTreeOp(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repoID := uuid.New()
	adapter := mem.New(ctx)
	namespace := "mem://data"
	objRepo := models.NewFileTree(db, repoID)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := workTree.WriteBlob(ctx, adapter, namespace, r, bLen, models.DefaultLeafProperty())
	require.NoError(t, err)

	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "faf499deee898c13e4ae4a2e6c4230fb", hash.Hash(workTree.Root().Hash()).Hex())

	//add again expect get an error
	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.True(t, errors.Is(err, ErrEntryExit))

	//update path
	binary = []byte(`“At the time, no single team member knew Go, but within a month, everyone was writing in Go and we were building out the endpoints. ”`)
	bLen = int64(len(binary))
	r = bytes.NewReader(binary)
	blob, err = workTree.WriteBlob(ctx, adapter, namespace, r, bLen, models.DefaultLeafProperty())
	require.NoError(t, err)

	err = workTree.ReplaceLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "d08bf786f0b4375dd6edd880859dc47a", hash.Hash(workTree.Root().Hash()).Hex())

	{
		//find blob
		findBlob, name, err := workTree.FindBlob(ctx, "a/b/c.txt")
		require.NoError(t, err)
		require.Equal(t, "c.txt", name)
		require.Equal(t, blob.Hash.Hex(), findBlob.Hash.Hex())
	}
	{
		//add another branch
		err = workTree.AddLeaf(ctx, "a/b/d.txt", blob)
		require.NoError(t, err)
		require.Equal(t, "b37d803cc5431587ef6f6e4d3aa8ada4", hash.Hash(workTree.Root().Hash()).Hex())

	}

	{
		//check fs structure
		rootDir, err := objRepo.TreeNode(ctx, workTree.Root().Hash())
		require.NoError(t, err)
		require.Len(t, rootDir.SubObjects, 1)
		require.Equal(t, "a", rootDir.SubObjects[0].Name)

		aDir, err := objRepo.TreeNode(ctx, rootDir.SubObjects[0].Hash)
		require.NoError(t, err)
		require.Len(t, aDir.SubObjects, 1)
		require.Equal(t, "b", aDir.SubObjects[0].Name)

		bDir, err := objRepo.TreeNode(ctx, aDir.SubObjects[0].Hash)
		require.NoError(t, err)
		require.Len(t, bDir.SubObjects, 2)
		require.Equal(t, "c.txt", bDir.SubObjects[0].Name)
		require.Equal(t, "d.txt", bDir.SubObjects[1].Name)
	}

	{
		//check ls
		subObjects, err := workTree.Ls(ctx, "a")
		require.NoError(t, err)
		require.Len(t, subObjects, 1)
		require.Equal(t, "b", subObjects[0].Name)

		subObjects, err = workTree.Ls(ctx, "a/b")
		require.NoError(t, err)
		require.Len(t, subObjects, 2)
		require.Equal(t, "c.txt", subObjects[0].Name)
		require.Equal(t, "d.txt", subObjects[1].Name)
	}

	err = workTree.RemoveEntry(ctx, "a/b/c.txt")
	require.NoError(t, err)
	require.Equal(t, "291af4419b76a09b60aa0cf911c72d06", hash.Hash(workTree.Root().Hash()).Hex())

	err = workTree.RemoveEntry(ctx, "a/b/d.txt")
	require.NoError(t, err)
	require.Equal(t, "", hash.Hash(workTree.Root().Hash()).Hex())
}

func TestRemoveEntry(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repoID := uuid.New()
	adapter := mem.New(ctx)
	namespace := "mem://data"
	objRepo := models.NewFileTree(db, repoID)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := workTree.WriteBlob(ctx, adapter, namespace, r, bLen, models.DefaultLeafProperty())
	require.NoError(t, err)

	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "faf499deee898c13e4ae4a2e6c4230fb", hash.Hash(workTree.Root().Hash()).Hex())

	//update path
	binary = []byte(`“At the time, no single team member knew Go, but within a month, everyone was writing in Go and we were building out the endpoints. ”`)
	bLen = int64(len(binary))
	r = bytes.NewReader(binary)
	blob, err = workTree.WriteBlob(ctx, adapter, namespace, r, bLen, models.DefaultLeafProperty())
	require.NoError(t, err)

	//add another branch
	err = workTree.AddLeaf(ctx, "a/b/d.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "77e60b4b1f28022818a3b97dfe064a3e", hash.Hash(workTree.Root().Hash()).Hex())

	err = workTree.RemoveEntry(ctx, "a/b")
	require.NoError(t, err)
	require.Equal(t, "", hash.Hash(workTree.Root().Hash()).Hex())
	entries, err := workTree.Ls(ctx, "")
	require.NoError(t, err)
	require.Len(t, entries, 0)

	entries, err = workTree.Ls(ctx, "/")
	require.NoError(t, err)
	require.Len(t, entries, 0)
}
