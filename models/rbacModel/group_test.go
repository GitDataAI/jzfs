package rbacModel_test

import (
	"context"
	"testing"
	"time"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/jiaozifs/jiaozifs/models/rbacModel"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"

	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
)

func TestGroupRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	groupRepo := rbacModel.NewGroupRepo(db)
	userGroupRepo := rbacModel.NewUserGroupRepo(db)

	t.Run("insert and get ", func(t *testing.T) {
		groupModel := &rbacModel.Group{}
		require.NoError(t, gofakeit.Struct(groupModel))

		newMemberModel, err := groupRepo.Insert(ctx, groupModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newMemberModel.ID)

		getMRParams := rbacModel.NewGetGroupParams().SetID(newMemberModel.ID)
		actualMember, err := groupRepo.Get(ctx, getMRParams)
		require.NoError(t, err)

		require.True(t, cmp.Equal(actualMember, newMemberModel, testhelper.DbTimeCmpOpt))
	})

	t.Run("insert and get by name ", func(t *testing.T) {
		groupModel := &rbacModel.Group{}
		require.NoError(t, gofakeit.Struct(groupModel))

		newGrouppModel, err := groupRepo.Insert(ctx, groupModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newGrouppModel.ID)

		userId := uuid.New()
		_, err = userGroupRepo.Insert(ctx, &rbacModel.UserGroup{
			UserID:    userId,
			GroupID:   newGrouppModel.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		require.NoError(t, err)

		actualGroup, err := groupRepo.GetGroupByUserID(ctx, userId)
		require.NoError(t, err)

		require.True(t, cmp.Equal(actualGroup, newGrouppModel, testhelper.DbTimeCmpOpt))
	})
}
