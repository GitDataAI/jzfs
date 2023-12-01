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

func TestRefRepo_Insert(t *testing.T) {
	ctx := context.Background()
	postgres, db := setup(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRefRepo(db)

	refModel := &models.Ref{}
	require.NoError(t, gofakeit.Struct(refModel))
	newRef, err := repo.Insert(ctx, refModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newRef.ID)

	ref, err := repo.Get(ctx, newRef.ID)
	require.NoError(t, err)

	require.True(t, cmp.Equal(refModel, ref, dbTimeCmpOpt))
}
