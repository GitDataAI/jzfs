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

func TestWipRepo(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewWipRepo(db)

	wipModel := &models.WorkingInProcess{}
	require.NoError(t, gofakeit.Struct(wipModel))
	newWipModel, err := repo.Insert(ctx, wipModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newWipModel.ID)

	user, err := repo.Get(ctx, &models.GetWipParam{
		ID:           newWipModel.ID,
		CreateID:     newWipModel.CreateID,
		RepositoryID: newWipModel.RepositoryID,
		RefID:        newWipModel.RefID,
	})
	require.NoError(t, err)
	require.True(t, cmp.Equal(newWipModel, user, dbTimeCmpOpt))

	err = repo.UpdateCurrentHash(ctx, newWipModel.ID, hash.Hash("mock hash"))
	require.NoError(t, err)
	updatedUser, err := repo.Get(ctx, &models.GetWipParam{ID: newWipModel.ID})
	require.NoError(t, err)
	require.Equal(t, "mock hash", string(updatedUser.CurrentTree))

	err = repo.UpdateBaseHash(ctx, newWipModel.ID, hash.Hash("mock base hash"))
	require.NoError(t, err)
	updatedUser, err = repo.Get(ctx, &models.GetWipParam{ID: newWipModel.ID})
	require.NoError(t, err)
	require.Equal(t, "mock base hash", string(updatedUser.BaseTree))

	err = repo.UpdateState(ctx, newWipModel.ID, models.Completed)
	require.NoError(t, err)
	updatedUser, err = repo.Get(ctx, &models.GetWipParam{ID: newWipModel.ID})
	require.NoError(t, err)
	require.Equal(t, models.Completed, updatedUser.State)

	t.Run("list", func(t *testing.T) {
		secWipModel := &models.WorkingInProcess{}
		require.NoError(t, gofakeit.Struct(secWipModel))
		secWipModel.CreateID = newWipModel.CreateID
		secWipModel.RepositoryID = newWipModel.RepositoryID
		secWipModel.RefID = newWipModel.RefID
		secNewWipModel, err := repo.Insert(ctx, secWipModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, secNewWipModel.ID)

		list, err := repo.List(ctx, &models.ListWipParam{
			CreateID:     secNewWipModel.CreateID,
			RepositoryID: secNewWipModel.RepositoryID,
			RefID:        secWipModel.RefID,
		})
		require.NoError(t, err)
		require.Len(t, list, 2)

		err = repo.Delete(ctx, &models.DeleteWipParam{
			ID:           secWipModel.ID,
			CreateID:     secWipModel.CreateID,
			RepositoryID: secWipModel.RepositoryID,
			RefID:        secWipModel.RefID,
		})
		require.NoError(t, err)

		_, err = repo.Get(ctx, &models.GetWipParam{ID: secWipModel.ID})
		require.ErrorIs(t, err, models.ErrNotFound)
	})
}
