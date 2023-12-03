package versionmgr

import (
	"bytes"
	"context"
	"errors"
	"testing"

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

	treeOp := NewTreeOp(objRepo)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := treeOp.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)
	assert.Equal(t, bLen, blob.Size)
	assert.Equal(t, "f3b39786b86a96372589aa1166966643", blob.Hash.Hex())
}

func TestTreeOpTreeOp(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	adapter := mem.New(ctx)
	objRepo := models.NewObjectRepo(db)

	treeOp := NewTreeOp(objRepo)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := treeOp.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	oriRoot, err := treeOp.AddLeaf(ctx, EmptyRoot, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "3bf643c30934d121ee45d413b165f135", oriRoot.Hash.Hex())

	//add again expect get an error
	_, err = treeOp.AddLeaf(ctx, oriRoot, "a/b/c.txt", blob)
	require.True(t, errors.Is(err, ErrEntryExit))

	//update path
	binary = []byte(`“At the time, no single team member knew Go, but within a month, everyone was writing in Go and we were building out the endpoints. ”`)
	bLen = int64(len(binary))
	r = bytes.NewReader(binary)
	blob, err = treeOp.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	updatedRoot, err := treeOp.ReplaceLeaf(ctx, oriRoot, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "8856b15f0f6c7ad21bfabe812df69e83", updatedRoot.Hash.Hex())

	{
		//add another branch
		updatedRoot, err = treeOp.AddLeaf(ctx, updatedRoot, "a/b/d.txt", blob)
		require.NoError(t, err)
		require.Equal(t, "225f0ca6233681a441969922a7425db2", updatedRoot.Hash.Hex())

	}

	{
		//check fs structure
		rootDir, err := objRepo.TreeNode(ctx, updatedRoot.Hash)
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
		subObjects, err := treeOp.Ls(ctx, updatedRoot, "a")
		require.NoError(t, err)
		require.Len(t, subObjects, 1)
		require.Equal(t, "b", subObjects[0].Name)

		subObjects, err = treeOp.Ls(ctx, updatedRoot, "a/b")
		require.NoError(t, err)
		require.Len(t, subObjects, 2)
		require.Equal(t, "c.txt", subObjects[0].Name)
		require.Equal(t, "d.txt", subObjects[1].Name)
	}

	rootAfterRemove, err := treeOp.RemoveEntry(ctx, updatedRoot, "a/b/c.txt")
	require.NoError(t, err)
	require.Equal(t, "f90e2d306ad172824fa171b9e0d9e133", rootAfterRemove.Hash.Hex())

	rootAfterRemoveAll, err := treeOp.RemoveEntry(ctx, rootAfterRemove, "a/b/d.txt")
	require.NoError(t, err)
	require.Equal(t, "", rootAfterRemoveAll.Hash.Hex())
}

func TestRemoveEntry(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	adapter := mem.New(ctx)
	objRepo := models.NewObjectRepo(db)

	treeOp := NewTreeOp(objRepo)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := treeOp.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	root, err := treeOp.AddLeaf(ctx, EmptyRoot, "a/b/c.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "3bf643c30934d121ee45d413b165f135", root.Hash.Hex())

	//update path
	binary = []byte(`“At the time, no single team member knew Go, but within a month, everyone was writing in Go and we were building out the endpoints. ”`)
	bLen = int64(len(binary))
	r = bytes.NewReader(binary)
	blob, err = treeOp.WriteBlob(ctx, adapter, r, bLen, block.PutOpts{})
	require.NoError(t, err)

	//add another branch
	root, err = treeOp.AddLeaf(ctx, root, "a/b/d.txt", blob)
	require.NoError(t, err)
	require.Equal(t, "81173b4a85cc5643feacd38b975e61a1", root.Hash.Hex())

	rootAfterRemoveAll, err := treeOp.RemoveEntry(ctx, root, "a/b")
	require.NoError(t, err)
	require.Equal(t, "", rootAfterRemoveAll.Hash.Hex())
}
