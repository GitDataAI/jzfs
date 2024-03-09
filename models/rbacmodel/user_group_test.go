package rbacmodel_test

import (
	"context"
	"testing"

	"github.com/GitDataAI/jiaozifs/models/rbacmodel"
	"github.com/GitDataAI/jiaozifs/testhelper"
	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"
)

func TestUserGroupRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	userGroupRepo := rbacmodel.NewUserGroupRepo(db)

	t.Run("insert and get ", func(t *testing.T) {
		userGroupModel := &rbacmodel.UserGroup{}
		require.NoError(t, gofakeit.Struct(userGroupModel))

		newUserGroup, err := userGroupRepo.Insert(ctx, userGroupModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newUserGroup.ID)

		getUserGroupParams := rbacmodel.NewGetUserGroupParams().SetUserID(userGroupModel.UserID).SetGroupID(userGroupModel.GroupID)
		actualUserGroup, err := userGroupRepo.Get(ctx, getUserGroupParams)
		require.NoError(t, err)
		require.True(t, cmp.Equal(actualUserGroup, newUserGroup, testhelper.DBTimeCmpOpt))

		_, err = userGroupRepo.Insert(ctx, userGroupModel)
		require.Error(t, err)
	})

}
