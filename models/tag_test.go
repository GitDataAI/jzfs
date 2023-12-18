package models_test

import (
	"context"
	"testing"

	"github.com/jiaozifs/jiaozifs/models"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"

	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"
)

func TestTagRepo(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	tagRepo := models.NewTagRepo(db)

	tagModel := &models.Tag{}
	require.NoError(t, gofakeit.Struct(tagModel))
	newTagModel, err := tagRepo.Insert(ctx, tagModel)
	require.NoError(t, err)
	tagModel, err = tagRepo.Tag(ctx, tagModel.Hash)
	require.NoError(t, err)

	require.True(t, cmp.Equal(tagModel, newTagModel, dbTimeCmpOpt))
}
