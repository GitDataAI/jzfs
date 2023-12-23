package models_test

import (
	"context"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"
)

func TestTagRepo(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repoID := uuid.New()
	tagRepo := models.NewTagRepo(db, repoID)
	require.Equal(t, tagRepo.RepositoryID(), repoID)

	tagModel := &models.Tag{}
	require.NoError(t, gofakeit.Struct(tagModel))
	tagModel.RepositoryID = repoID
	newTagModel, err := tagRepo.Insert(ctx, tagModel)
	require.NoError(t, err)
	tagModel, err = tagRepo.Tag(ctx, tagModel.Hash)
	require.NoError(t, err)

	require.True(t, cmp.Equal(tagModel, newTagModel, dbTimeCmpOpt))

	t.Run("mis match repo id", func(t *testing.T) {
		mistMatchModel := &models.Tag{}
		require.NoError(t, gofakeit.Struct(mistMatchModel))
		_, err := tagRepo.Insert(ctx, mistMatchModel)
		require.ErrorIs(t, err, models.ErrRepoIDMisMatch)
	})
}
