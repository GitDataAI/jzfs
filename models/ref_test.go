package models_test

import (
	"context"
	"testing"

	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"
)

func TestRefRepoInsert(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRefRepo(db)

	refModel := &models.Ref{}
	require.NoError(t, gofakeit.Struct(refModel))
	newRef, err := repo.Insert(ctx, refModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newRef.ID)

	ref, err := repo.Get(ctx, &models.GetRefParams{
		ID:           newRef.ID,
		RepositoryID: newRef.RepositoryID,
		Name:         utils.String(newRef.Name),
	})
	require.NoError(t, err)

	require.True(t, cmp.Equal(refModel, ref, dbTimeCmpOpt))

	mockHash := hash.Hash("mock hash")
	err = repo.UpdateCommitHash(ctx, ref.ID, mockHash)
	require.NoError(t, err)

	refAfterUpdated, err := repo.Get(ctx, &models.GetRefParams{
		ID: newRef.ID,
	})
	require.NoError(t, err)
	require.Equal(t, mockHash, refAfterUpdated.CommitHash)
}
