package models_test

import (
	"context"
	"testing"

	"github.com/google/uuid"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"
)

func TestAkskRepo_Delete(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repo := models.NewAkskRepo(db)

	t.Run("insert and get ", func(t *testing.T) {
		akskModel := &models.AkSk{}
		require.NoError(t, gofakeit.Struct(akskModel))

		aksk, err := repo.Insert(ctx, akskModel)
		require.NoError(t, err)

		expectAksk, err := repo.Get(ctx, models.NewGetAkSkParams().SetAccessKey(aksk.AccessKey).SetID(aksk.ID).SetUserID(aksk.UserID))
		require.NoError(t, err)
		require.True(t, cmp.Equal(expectAksk, aksk, testhelper.DbTimeCmpOpt))
	})
	t.Run("list", func(t *testing.T) {
		userID := uuid.New()
		for i := 0; i < 5; i++ {
			akskModel := &models.AkSk{}
			require.NoError(t, gofakeit.Struct(akskModel))
			akskModel.UserID = userID
			_, err := repo.Insert(ctx, akskModel)
			require.NoError(t, err)
		}

		userID = uuid.New()
		for i := 0; i < 5; i++ {
			akskModel := &models.AkSk{}
			require.NoError(t, gofakeit.Struct(akskModel))
			akskModel.UserID = userID
			_, err := repo.Insert(ctx, akskModel)
			require.NoError(t, err)
		}

		aksks, hasMore, err := repo.List(ctx, models.NewListAkSkParams().SetUserID(userID))
		require.NoError(t, err)
		require.False(t, hasMore)
		require.Len(t, aksks, 5)

		aksks, hasMore, err = repo.List(ctx, models.NewListAkSkParams().SetUserID(userID).SetAmount(2))
		require.NoError(t, err)
		require.True(t, hasMore)
		require.Len(t, aksks, 2)
	})

	t.Run("delete by id", func(t *testing.T) {
		akskModel := &models.AkSk{}
		require.NoError(t, gofakeit.Struct(akskModel))

		aksk, err := repo.Insert(ctx, akskModel)
		require.NoError(t, err)

		deleteRows, err := repo.Delete(ctx, models.NewDeleteAkSkParams().SetUserID(aksk.UserID).SetID(aksk.ID))
		require.NoError(t, err)
		require.Equal(t, int64(1), deleteRows)
	})

	t.Run("delete by ak", func(t *testing.T) {
		akskModel := &models.AkSk{}
		require.NoError(t, gofakeit.Struct(akskModel))

		aksk, err := repo.Insert(ctx, akskModel)
		require.NoError(t, err)

		deleteRows, err := repo.Delete(ctx, models.NewDeleteAkSkParams().SetUserID(aksk.UserID).SetAccessKey(aksk.AccessKey))
		require.NoError(t, err)
		require.Equal(t, int64(1), deleteRows)
	})
}
