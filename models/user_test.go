package models_test

import (
	"context"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"
)

func TestNewUserRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repo := models.NewUserRepo(db)

	userModel := &models.User{}
	require.NoError(t, gofakeit.Struct(userModel))
	newUser, err := repo.Insert(ctx, userModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newUser.ID)

	user, err := repo.Get(ctx, models.NewGetUserParams().SetID(newUser.ID))
	require.NoError(t, err)

	require.True(t, cmp.Equal(userModel, user, testhelper.DBTimeCmpOpt))

	ep, err := repo.GetEPByName(ctx, newUser.Name)
	require.NoError(t, err)
	require.True(t, cmp.Equal(userModel.EncryptedPassword, ep))

	userByEmail, err := repo.Get(ctx, models.NewGetUserParams().SetEmail(newUser.Email))
	require.NoError(t, err)
	require.True(t, cmp.Equal(userModel, userByEmail, testhelper.DBTimeCmpOpt))

	userByName, err := repo.Get(ctx, models.NewGetUserParams().SetName(newUser.Name))
	require.NoError(t, err)
	require.True(t, cmp.Equal(userModel, userByName, testhelper.DBTimeCmpOpt))
}

func TestCount(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repo := models.NewUserRepo(db)

	var users []*models.User
	for i := 0; i < 5; i++ {
		userModel := &models.User{}
		require.NoError(t, gofakeit.Struct(userModel))
		newUser, err := repo.Insert(ctx, userModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newUser.ID)
		users = append(users, newUser)
	}

	count, err := repo.Count(ctx, models.NewCountUserParams().SetName(users[0].Name))
	require.NoError(t, err)
	require.Equal(t, 1, count)

	count, err = repo.Count(ctx, models.NewCountUserParams().SetEmail(users[0].Email))
	require.NoError(t, err)
	require.Equal(t, 1, count)
}
