package versionmgr

import (
	"context"
	"errors"
	"testing"

	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/testhelper"
	"github.com/GitDataAI/jiaozifs/utils/hash"
	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"
)

func TestWorkTree(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repoID := uuid.New()
	objRepo := models.NewFileTree(db, repoID)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	blob := &models.Blob{}
	require.NoError(t, gofakeit.Struct(blob))
	blob.Type = models.BlobObject
	blob.RepositoryID = repoID

	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)

	//add again expect get an error
	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.True(t, errors.Is(err, ErrEntryExit))

	//update path
	blob = &models.Blob{}
	require.NoError(t, gofakeit.Struct(blob))
	blob.Type = models.BlobObject
	blob.RepositoryID = repoID
	err = workTree.ReplaceLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)

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

	err = workTree.RemoveEntry(ctx, "a/b/d.txt")
	require.NoError(t, err)
	require.Equal(t, "", hash.Hash(workTree.Root().Hash()).Hex())
}

func TestRemoveEntry(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repoID := uuid.New()
	objRepo := models.NewFileTree(db, repoID)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	blob := &models.Blob{}
	require.NoError(t, gofakeit.Struct(blob))
	blob.Type = models.BlobObject
	blob.RepositoryID = repoID
	err = workTree.AddLeaf(ctx, "a/b/c.txt", blob)
	require.NoError(t, err)

	//update path
	blob = &models.Blob{}
	require.NoError(t, gofakeit.Struct(blob))
	blob.Type = models.BlobObject
	blob.RepositoryID = repoID
	//add another branch
	err = workTree.AddLeaf(ctx, "a/b/d.txt", blob)
	require.NoError(t, err)

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

func TestCleanPath(t *testing.T) {
	require.Equal(t, "", CleanPath(""))
	require.Equal(t, "", CleanPath("/"))

	require.Equal(t, "a/b/c", CleanPath("a/b/c"))
	require.Equal(t, "a/b/c", CleanPath("/a/b/c/"))
	require.Equal(t, "a/b/c", CleanPath("\\a\\b\\c\\"))
}

func TestWorkTreeGetFiles(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repoID := uuid.New()
	objRepo := models.NewFileTree(db, repoID)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	paths := []string{
		"a/b/c.txt",
		"a/b/d.txt",
		"ff/b/c.txt",
		"ff/b/d.txt",
		"ff/b/e.jpg",
		"ff/b/f.jpg",
		"a.txt",
	}
	for _, path := range paths {
		blob := &models.Blob{}
		require.NoError(t, gofakeit.Struct(blob))
		blob.Type = models.BlobObject
		blob.RepositoryID = repoID
		err = workTree.AddLeaf(ctx, path, blob)
		require.NoError(t, err)
	}

	t.Run("all", func(t *testing.T) {
		manifest, err := workTree.GetTreeManifest(ctx, "*")
		require.NoError(t, err)
		require.Equal(t, 7, len(manifest.FileList))
		require.Equal(t, "a.txt", manifest.FileList)
		require.Equal(t, "a/b/c.txt", manifest.FileList)
		require.Equal(t, "a/b/d.txt", manifest.FileList)
		require.Equal(t, "ff/b/c.txt", manifest.FileList)
		require.Equal(t, "ff/b/d.txt", manifest.FileList)
		require.Equal(t, "ff/b/e.jpg", manifest.FileList)
		require.Equal(t, "ff/b/f.jpg", manifest.FileList)
	})

	t.Run("single file", func(t *testing.T) {
		manifest, err := workTree.GetTreeManifest(ctx, "a/b/d.txt")
		require.NoError(t, err)
		require.Equal(t, 1, len(manifest.FileList))
		require.Equal(t, "a/b/d.txt", manifest.FileList[0])
	})

	t.Run("single path", func(t *testing.T) {
		manifest, err := workTree.GetTreeManifest(ctx, "a")
		require.NoError(t, err)
		require.Equal(t, 0, len(manifest.FileList))
	})

	t.Run("ext match", func(t *testing.T) {
		manifest, err := workTree.GetTreeManifest(ctx, "*.jpg")
		require.NoError(t, err)
		require.Equal(t, 2, len(manifest.FileList))
		require.Equal(t, "ff/b/e.jpg", manifest.FileList[0])
		require.Equal(t, "ff/b/f.jpg", manifest.FileList[1])
	})

	t.Run("filename match", func(t *testing.T) {
		manifest, err := workTree.GetTreeManifest(ctx, "*/e.jpg")
		require.NoError(t, err)
		require.Equal(t, 1, len(manifest.FileList))
		require.Equal(t, "ff/b/e.jpg", manifest.FileList[0])
	})

	t.Run("sub", func(t *testing.T) {
		manifest, err := workTree.GetTreeManifest(ctx, "a/*")
		require.NoError(t, err)
		require.Equal(t, 2, len(manifest.FileList))
		require.Equal(t, "a/b/c.txt", manifest.FileList[0])
		require.Equal(t, "a/b/d.txt", manifest.FileList[1])
	})

}
