package models_test

import (
	"context"
	"testing"

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

	getRefParams := models.NewGetRefParams().
		SetID(newRef.ID).
		SetRepositoryID(newRef.RepositoryID).
		SetName(newRef.Name)
	ref, err := repo.Get(ctx, getRefParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(refModel, ref, dbTimeCmpOpt))

	mockHash := hash.Hash("mock hash")
	err = repo.UpdateByID(ctx, models.NewUpdateRefParams(newRef.ID).SetCommitHash(mockHash))
	require.NoError(t, err)

	refAfterUpdated, err := repo.Get(ctx, &models.GetRefParams{
		ID: newRef.ID,
	})
	require.NoError(t, err)
	require.Equal(t, mockHash, refAfterUpdated.CommitHash)

	list, _, err := repo.List(ctx, models.NewListRefParams().SetRepositoryID(ref.RepositoryID))
	require.NoError(t, err)
	require.Len(t, list, 1)

	// second
	secModel := &models.Ref{}
	require.NoError(t, gofakeit.Struct(secModel))
	secModel.RepositoryID = ref.RepositoryID
	secRef, err := repo.Insert(ctx, secModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, secRef.ID)

	getSecRefParams := models.NewGetRefParams().
		SetID(secRef.ID).
		SetRepositoryID(secRef.RepositoryID).
		SetName(secRef.Name)
	sRef, err := repo.Get(ctx, getSecRefParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(secModel, sRef, dbTimeCmpOpt))

	// amount
	list, hasMore, err := repo.List(ctx, models.NewListRefParams().SetRepositoryID(ref.RepositoryID).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list, 1)
	require.True(t, hasMore)

	affectedRows, err := repo.Delete(ctx, models.NewDeleteRefParams().SetID(list[0].ID).SetRepositoryID(list[0].RepositoryID).SetName(list[0].Name))
	require.NoError(t, err)
	require.Equal(t, int64(1), affectedRows)

	list, _, err = repo.List(ctx, models.NewListRefParams().SetRepositoryID(ref.RepositoryID))
	require.NoError(t, err)
	require.Len(t, list, 1)
}
