package versionmgr

import (
	"context"
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
	var filePath []string
	var dirPaths []string
	err = wk.Walk(ctx, func(entry *models.TreeEntry, _ *models.Blob, path string) error {
		if entry.IsDir {
			dirPaths = append(dirPaths, path)
		} else {
			filePath = append(filePath, path)
		}
		return nil
	})
	require.NoError(t, err)
	require.Equal(t, "a", dirPaths[0])
	require.Equal(t, "mm", dirPaths[1])
	require.Equal(t, "a/b", dirPaths[2])
	require.Equal(t, "a/c", dirPaths[3])
	require.Equal(t, "mm/c", dirPaths[4])

	require.Equal(t, "a.txt", filePath[0])
	require.Equal(t, "b.txt", filePath[1])
	require.Equal(t, "a/b/c.txt", filePath[2])
	require.Equal(t, "a/b/d.txt", filePath[3])
	require.Equal(t, "a/c/e.txt", filePath[4])
	require.Equal(t, "a/c/f.txt", filePath[5])
	require.Equal(t, "mm/f.txt", filePath[6])
	require.Equal(t, "mm/c/f.txt", filePath[7])
}

func addLeaves(ctx context.Context, t *testing.T, workTree *WorkTree, repoID uuid.UUID, path string) {
	blob := &models.Blob{}
	require.NoError(t, gofakeit.Struct(blob))
	blob.Type = models.BlobObject
	blob.RepositoryID = repoID

	err := workTree.AddLeaf(ctx, path, blob)
	require.NoError(t, err)
}
