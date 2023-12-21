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

	repo := models.NewBranchRepo(db)

	branchModel := &models.Branches{}
	require.NoError(t, gofakeit.Struct(branchModel))
	newBrance, err := repo.Insert(ctx, branchModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newBrance.ID)

	getBranchParams := models.NewGetBranchParams().
		SetID(newBrance.ID).
		SetRepositoryID(newBrance.RepositoryID).
		SetName(newBrance.Name)
	branch, err := repo.Get(ctx, getBranchParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(branchModel, branch, dbTimeCmpOpt))

	mockHash := hash.Hash("mock hash")
	err = repo.UpdateByID(ctx, models.NewUpdateBranchParams(newBrance.ID).SetCommitHash(mockHash))
	require.NoError(t, err)

	branchAfterUpdated, err := repo.Get(ctx, &models.GetBranchParams{
		ID: newBrance.ID,
	})
	require.NoError(t, err)
	require.Equal(t, mockHash, branchAfterUpdated.CommitHash)

	list, _, err := repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID))
	require.NoError(t, err)
	require.Len(t, list, 1)

	affectedRows, err := repo.Delete(ctx, models.NewDeleteBranchParams().SetID(list[0].ID).SetRepositoryID(list[0].RepositoryID).SetName(list[0].Name))
	require.NoError(t, err)
	require.Equal(t, int64(1), affectedRows)

	list, _, err = repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID))
	require.NoError(t, err)
	require.Len(t, list, 0)
}
