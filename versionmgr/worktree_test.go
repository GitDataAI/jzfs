package versionmgr

import (
	"bytes"
	"context"
	"errors"
	"io"
	"testing"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/block"

	"github.com/jiaozifs/jiaozifs/block/mem"
	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/testhelper"
)

func TestTreeOpWriteBlob(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	adapter := mem.New(ctx)
	objRepo := models.NewObjectRepo(db)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := workTree.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)
	assert.Equal(t, bLen, blob.Size)
	assert.Equal(t, "f3b39786b86a96372589aa1166966643", blob.Hash.Hex())

	reader, err := workTree.ReadBlob(ctx, adapter, blob, nil)
	require.NoError(t, err)
	content, err := io.ReadAll(reader)
	require.NoError(t, err)
	require.Equal(t, binary, content)
}

func TestTreeOpTreeOp(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	adapter := mem.New(ctx)
	objRepo := models.NewObjectRepo(db)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := workTree.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "3bf643c30934d121ee45d413b165f135", hash.Hash(workTree.Root().Hash()).Hex())

	//add again expect get an error
	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.True(t, errors.Is(err, ErrEntryExit))

	//update path
	binary = []byte(`“At the time, no single team member knew Go, but within a month, everyone was writing in Go and we were building out the endpoints. ”`)
	bLen = int64(len(binary))
	r = bytes.NewReader(binary)
	blob, err = workTree.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	err = workTree.ReplaceLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "8856b15f0f6c7ad21bfabe812df69e83", hash.Hash(workTree.Root().Hash()).Hex())

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
		require.Equal(t, "225f0ca6233681a441969922a7425db2", hash.Hash(workTree.Root().Hash()).Hex())

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
	require.Equal(t, "f90e2d306ad172824fa171b9e0d9e133", hash.Hash(workTree.Root().Hash()).Hex())

	err = workTree.RemoveEntry(ctx, "a/b/d.txt")
	require.NoError(t, err)
	require.Equal(t, "", hash.Hash(workTree.Root().Hash()).Hex())
}

func TestRemoveEntry(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	adapter := mem.New(ctx)
	objRepo := models.NewObjectRepo(db)

	treeOp, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := treeOp.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	err = treeOp.AddLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "3bf643c30934d121ee45d413b165f135", hash.Hash(treeOp.Root().Hash()).Hex())

	//update path
	binary = []byte(`“At the time, no single team member knew Go, but within a month, everyone was writing in Go and we were building out the endpoints. ”`)
	bLen = int64(len(binary))
	r = bytes.NewReader(binary)
	blob, err = treeOp.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	//add another branch
	err = treeOp.AddLeaf(ctx, "a/b/d.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "81173b4a85cc5643feacd38b975e61a1", hash.Hash(treeOp.Root().Hash()).Hex())

	err = treeOp.RemoveEntry(ctx, "a/b")
	require.NoError(t, err)
	require.Equal(t, "", hash.Hash(treeOp.Root().Hash()).Hex())
}
