package rbacModel_test

import (
	"context"
	"github.com/jiaozifs/jiaozifs/models/rbacModel"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"
)

func TestUserGroupRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	userGroupRepo := rbacModel.NewUserGroupRepo(db)

	t.Run("insert and get ", func(t *testing.T) {
		userGroupModel := &rbacModel.UserGroup{}
		require.NoError(t, gofakeit.Struct(userGroupModel))

		newUserGroup, err := userGroupRepo.Insert(ctx, userGroupModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newUserGroup.ID)

		getUserGroupParams := rbacModel.NewGetUserGroupParams().SetUserID(userGroupModel.UserID).SetGroupID(userGroupModel.GroupID)
		actualUserGroup, err := userGroupRepo.Get(ctx, getUserGroupParams)
		require.NoError(t, err)
		require.True(t, cmp.Equal(actualUserGroup, newUserGroup, testhelper.DbTimeCmpOpt))

		_, err = userGroupRepo.Insert(ctx, userGroupModel)
		require.Error(t, err)
	})

}
