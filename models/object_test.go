package models_test

import (
	"context"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/stretchr/testify/require"
)

func TestObjectRepo_Insert(t *testing.T) {
	ctx := context.Background()
	postgres, db := setup(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewObjectRepo(db)

	objModel := &models.Object{}
	require.NoError(t, gofakeit.Struct(objModel))
	newObj, err := repo.Insert(ctx, objModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newObj.ID)

	count, err := repo.Count(ctx)
	require.NoError(t, err)
	require.Equal(t, 1, count)

	list, err := repo.List(ctx)
	require.NoError(t, err)
	require.Equal(t, 1, len(list))

	ref, err := repo.Get(ctx, newObj.ID)
	require.NoError(t, err)

	require.True(t, cmp.Equal(newObj, ref, dbTimeCmpOpt))
}
