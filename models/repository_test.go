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
	"github.com/stretchr/testify/require"
)

func TestRepositoryRepo_Insert(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepositoryRepo(db)

	repoModel := &models.Repository{}
	require.NoError(t, gofakeit.Struct(repoModel))
	repoModel.Name = "aaabbbb"
	newRepo, err := repo.Insert(ctx, repoModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newRepo.ID)

	user, err := repo.Get(ctx, models.NewGetRepoParams().SetID(newRepo.ID))
	require.NoError(t, err)
	require.True(t, cmp.Equal(repoModel, user, dbTimeCmpOpt))

	err = repo.UpdateByID(ctx, models.NewUpdateRepoParams(newRepo.ID).SetDescription("description"))
	require.NoError(t, err)
	user, err = repo.Get(ctx, models.NewGetRepoParams().SetID(newRepo.ID))
	require.NoError(t, err)

	require.Equal(t, "description", *user.Description)
	//insert secondary
	secModel := &models.Repository{}
	require.NoError(t, gofakeit.Struct(secModel))
	secModel.CreatorID = repoModel.CreatorID
	secModel.Name = "adabbeb"
	secRepo, err := repo.Insert(ctx, secModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, secRepo.ID)

	//list
	repos, _, err := repo.List(ctx, models.NewListRepoParams())
	require.NoError(t, err)
	require.Len(t, repos, 2)

	{
		//exact adabbeb
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName(utils.String("adabbeb"), models.PrefixMatch))
		require.NoError(t, err)
		require.Len(t, repos, 1)
	}
	{
		//prefix a
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName(utils.String("a"), models.PrefixMatch))
		require.NoError(t, err)
		require.Len(t, repos, 2)
	}

	{
		//subfix b
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName(utils.String("b"), models.SuffixMatch))
		require.NoError(t, err)
		require.Len(t, repos, 2)
	}
	{
		//like ab
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName(utils.String("ab"), models.LikeMatch))
		require.NoError(t, err)
		require.Len(t, repos, 2)
	}
	{
		//like ab
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName(utils.String("adabbeb"), models.LikeMatch))
		require.NoError(t, err)
		require.Len(t, repos, 1)
	}
	{
		//amount 1
		repos, hasMore, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetAmount(1))
		require.NoError(t, err)
		require.True(t, hasMore)
		require.Len(t, repos, 1)
	}
	{
		//amount 2
		repos, hasMore, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetAmount(2))
		require.NoError(t, err)
		require.True(t, hasMore)
		require.Len(t, repos, 2)
	}
	{
		//amount 3
		repos, hasMore, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetAmount(3))
		require.NoError(t, err)
		require.False(t, hasMore)
		require.Len(t, repos, 2)
	}
	{
		//after
		repos, hasMore, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetAfter(utils.Time(secRepo.UpdatedAt)).SetAmount(1))
		require.NoError(t, err)
		require.True(t, hasMore)
		require.Len(t, repos, 1)
	}
	//delete
	deleteParams := models.NewDeleteRepoParams().
		SetID(secRepo.ID).
		SetOwnerID(secRepo.OwnerID).
		SetName(secRepo.Name)
	affectRows, err := repo.Delete(ctx, deleteParams)
	require.NoError(t, err)
	require.Equal(t, int64(1), affectRows)

	_, err = repo.Get(ctx, models.NewGetRepoParams().SetID(secRepo.ID))
	require.ErrorIs(t, err, models.ErrNotFound)
}
