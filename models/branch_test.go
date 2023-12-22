package models_test

import (
	"context"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/stretchr/testify/require"
)

func TestRefRepoInsert(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewBranchRepo(db)

	branchModel := &models.Branches{}
	require.NoError(t, gofakeit.Struct(branchModel))
	branchModel.Name = "feat/abc/aaa"
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

	// second
	secModel := &models.Branches{}
	require.NoError(t, gofakeit.Struct(secModel))
	secModel.RepositoryID = branch.RepositoryID
	branchModel.Name = "feat/bba/ccc"
	secRef, err := repo.Insert(ctx, secModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, secRef.ID)

	getSecRefParams := models.NewGetBranchParams().
		SetID(secRef.ID).
		SetRepositoryID(secRef.RepositoryID).
		SetName(secRef.Name)
	sRef, err := repo.Get(ctx, getSecRefParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(secModel, sRef, dbTimeCmpOpt))

	// ExactMatch
	list1, hasMore, err := repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID).SetName(utils.String(secModel.Name), models.ExactMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list1, 1)
	require.True(t, hasMore)

	// PrefixMatch
	list2, hasMore, err := repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID).SetName(utils.String(secModel.Name[:3]), models.PrefixMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list2, 1)
	require.True(t, hasMore)

	// SuffixMatch
	list3, hasMore, err := repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID).SetName(utils.String(secModel.Name[3:]), models.SuffixMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list3, 1)
	require.True(t, hasMore)

	// LikeMatch
	list4, hasMore, err := repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID).SetName(utils.String(secModel.Name[2:4]), models.LikeMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list4, 1)
	require.True(t, hasMore)

	// After
	list5, hasMore, err := repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID).SetAfter(utils.String("feat/abcd/aaa")))
	require.NoError(t, err)
	require.Len(t, list5, 1)
	require.False(t, hasMore)

	affectedRows, err := repo.Delete(ctx, models.NewDeleteBranchParams().SetID(list[0].ID).SetRepositoryID(list[0].RepositoryID).SetName(list[0].Name))
	require.NoError(t, err)
	require.Equal(t, int64(1), affectedRows)

	list6, _, err := repo.List(ctx, models.NewListBranchParams().SetRepositoryID(branch.RepositoryID))
	require.NoError(t, err)
	require.Len(t, list6, 1)
}
