package models_test

import (
	"context"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/filemode"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"
)

func Test_sortSubObjects(t *testing.T) {
	entries := []models.TreeEntry{
		{
			Name: "c.txt",
			Hash: nil,
		},
		{
			Name: "a.txt",
			Hash: nil,
		},
		{
			Name: "b.txt",
			Hash: nil,
		},
	}

	models.SortSubObjects(entries)
	require.Equal(t, "a.txt", entries[0].Name)
	require.Equal(t, "b.txt", entries[1].Name)
	require.Equal(t, "c.txt", entries[2].Name)
}

func TestObjectRepo_Insert(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repoID := uuid.New()
	repo := models.NewFileTree(db, repoID)
	require.Equal(t, repo.RepositoryID(), repoID)

	objModel := &models.FileTree{}
	require.NoError(t, gofakeit.Struct(objModel))
	objModel.RepositoryID = repoID
	objModel.Properties.Mode = filemode.Regular
	newObj, err := repo.Insert(ctx, objModel)
	require.NoError(t, err)
	require.NotEqual(t, nil, newObj.Hash)

	count, err := repo.Count(ctx)
	require.NoError(t, err)
	require.Equal(t, 1, count)

	list, err := repo.List(ctx)
	require.NoError(t, err)
	require.Equal(t, 1, len(list))

	ref, err := repo.Get(ctx, models.NewGetObjParams().SetHash(newObj.Hash))
	require.NoError(t, err)

	require.True(t, cmp.Equal(newObj, ref, dbTimeCmpOpt))
	t.Run("mis match repo id", func(t *testing.T) {
		mistMatchModel := &models.FileTree{}
		require.NoError(t, gofakeit.Struct(mistMatchModel))
		mistMatchModel.Properties.Mode = filemode.Regular
		_, err := repo.Insert(ctx, mistMatchModel)
		require.ErrorIs(t, err, models.ErrRepoIDMisMatch)
	})
}
