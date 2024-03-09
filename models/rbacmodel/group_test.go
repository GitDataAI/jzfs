package rbacmodel_test

import (
	"context"
	"testing"
	"time"

	"github.com/GitDataAI/jiaozifs/models/rbacmodel"
	"github.com/GitDataAI/jiaozifs/utils"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/GitDataAI/jiaozifs/testhelper"
	"github.com/stretchr/testify/require"

	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
)

func TestGroupRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	groupRepo := rbacmodel.NewGroupRepo(db)
	userGroupRepo := rbacmodel.NewUserGroupRepo(db)

	t.Run("insert and get ", func(t *testing.T) {
		groupModel := &rbacmodel.Group{}
		require.NoError(t, gofakeit.Struct(groupModel))

		newGroupModel, err := groupRepo.Insert(ctx, groupModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newGroupModel.ID)

		getMRParams := rbacmodel.NewGetGroupParams().SetID(newGroupModel.ID)
		actualMember, err := groupRepo.Get(ctx, getMRParams)
		require.NoError(t, err)

		require.True(t, cmp.Equal(actualMember, newGroupModel, testhelper.DBTimeCmpOpt))
	})

	t.Run("insert and get by name ", func(t *testing.T) {
		groupModel := &rbacmodel.Group{}
		require.NoError(t, gofakeit.Struct(groupModel))

		newGrouppModel, err := groupRepo.Insert(ctx, groupModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newGrouppModel.ID)

		userID := uuid.New()
		_, err = userGroupRepo.Insert(ctx, &rbacmodel.UserGroup{
			UserID:    userID,
			GroupID:   newGrouppModel.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		require.NoError(t, err)

		actualGroup, err := groupRepo.GetGroupByUserID(ctx, userID)
		require.NoError(t, err)

		require.True(t, cmp.Equal(actualGroup, newGrouppModel, testhelper.DBTimeCmpOpt))
	})

	t.Run("list", func(t *testing.T) {
		var groups []*rbacmodel.Group
		var names []string
		for i := 0; i < 10; i++ {
			groupModel := &rbacmodel.Group{}
			require.NoError(t, gofakeit.Struct(groupModel))
			groupModel.CreatedAt = time.Now()

			newGroupModel, err := groupRepo.Insert(ctx, groupModel)
			require.NoError(t, err)
			require.NotEqual(t, uuid.Nil, newGroupModel.ID)
			groups = append(groups, newGroupModel)
			names = append(names, newGroupModel.Name)
		}

		listGroupParmas := rbacmodel.NewListGroupParams().SetNames(names...)
		listGroups, err := groupRepo.List(ctx, listGroupParmas)
		require.NoError(t, err)
		require.True(t, cmp.Equal(utils.Reverse(listGroups), groups, testhelper.DBTimeCmpOpt))
	})
}
