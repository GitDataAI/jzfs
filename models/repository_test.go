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

func TestRepositoryRepo_Insert(t *testing.T) {
	ctx := context.Background()
	postgres, db := setup(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepositoryRepo(db)

	repoModel := &models.Repository{}
	require.NoError(t, gofakeit.Struct(repoModel))
	newUser, err := repo.Insert(ctx, repoModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newUser.ID)

	user, err := repo.Get(ctx, newUser.ID)
	require.NoError(t, err)

	require.True(t, cmp.Equal(repoModel, user, dbTimeCmpOpt))
}
