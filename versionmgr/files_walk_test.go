package versionmgr

import (
	"context"
	"fmt"
	"testing"

	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/testhelper"
	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"
)

func TestFileWalk_Walk(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repoID := uuid.New()
	objRepo := models.NewFileTree(db, repoID)

	workTree, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	require.NoError(t, err)

	addLeaves(ctx, t, workTree, repoID, "b.txt")
	addLeaves(ctx, t, workTree, repoID, "a.txt")

	addLeaves(ctx, t, workTree, repoID, "a/c/f.txt")
	addLeaves(ctx, t, workTree, repoID, "mm/f.txt")
	addLeaves(ctx, t, workTree, repoID, "mm/c/f.txt")
	addLeaves(ctx, t, workTree, repoID, "a/b/c.txt")
	addLeaves(ctx, t, workTree, repoID, "a/b/d.txt")
	addLeaves(ctx, t, workTree, repoID, "a/c/e.txt")
	wk := FileWalk{
		object:  objRepo,
		curNode: workTree.root,
	}
	var paths []string
	err = wk.Walk(ctx, func(_ *models.Blob, path string) error {
		fmt.Println(path)
		paths = append(paths, path)
		return nil
	})
	require.NoError(t, err)
	require.Equal(t, "a.txt", paths[0])
	require.Equal(t, "b.txt", paths[1])
	require.Equal(t, "a/b/c.txt", paths[2])
	require.Equal(t, "a/b/d.txt", paths[3])
	require.Equal(t, "a/c/e.txt", paths[4])
	require.Equal(t, "a/c/f.txt", paths[5])
	require.Equal(t, "mm/f.txt", paths[6])
	require.Equal(t, "mm/c/f.txt", paths[7])
}

func addLeaves(ctx context.Context, t *testing.T, workTree *WorkTree, repoID uuid.UUID, path string) {
	blob := &models.Blob{}
	require.NoError(t, gofakeit.Struct(blob))
	blob.Type = models.BlobObject
	blob.RepositoryID = repoID

	err := workTree.AddLeaf(ctx, path, blob)
	require.NoError(t, err)
}
