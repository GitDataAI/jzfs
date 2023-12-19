package models_test

import (
	"context"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
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
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName("adabbeb", models.PrefixMatch))
		require.NoError(t, err)
		require.Len(t, repos, 1)
	}
	{
		//prefix a
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName("a", models.PrefixMatch))
		require.NoError(t, err)
		require.Len(t, repos, 2)
	}

	{
		//subfix b
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName("b", models.SuffixMatch))
		require.NoError(t, err)
		require.Len(t, repos, 2)
	}
	{
		//like ab
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName("ab", models.LikeMatch))
		require.NoError(t, err)
		require.Len(t, repos, 2)
	}
	{
		//like ab
		repos, _, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetName("adabbeb", models.LikeMatch))
		require.NoError(t, err)
		require.Len(t, repos, 1)
	}
	{
		//amount 1
		repos, has_more, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetAmount(1))
		require.NoError(t, err)
		require.True(t, has_more)
		require.Len(t, repos, 1)
	}
	{
		//amount 2
		repos, has_more, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetAmount(2))
		require.NoError(t, err)
		require.True(t, has_more)
		require.Len(t, repos, 2)
	}
	{
		//amount 3
		repos, has_more, err := repo.List(ctx, models.NewListRepoParams().SetCreatorID(secModel.CreatorID).SetAmount(3))
		require.NoError(t, err)
		require.False(t, has_more)
		require.Len(t, repos, 2)
	}

	//delete
	err = repo.Delete(ctx, models.NewDeleteRepoParams().SetID(secRepo.ID))
	require.NoError(t, err)
	_, err = repo.Get(ctx, models.NewGetRepoParams().SetID(secRepo.ID))
	require.ErrorIs(t, err, models.ErrNotFound)
}
