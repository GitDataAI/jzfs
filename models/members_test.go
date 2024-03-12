package models_test

import (
	"context"
	"testing"
	"time"

	"github.com/GitDataAI/jiaozifs/utils"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"

	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/testhelper"
)

func TestMemberRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	memberRepo := models.NewMemberRepo(db)

	t.Run("insert and get", func(t *testing.T) {
		memberModel := &models.Member{}
		require.NoError(t, gofakeit.Struct(memberModel))

		newMemberModel, err := memberRepo.Insert(ctx, memberModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newMemberModel.ID)

		getMRParams := models.NewGetMemberParams().SetID(newMemberModel.ID).SetUserID(newMemberModel.UserID).SetRepoID(newMemberModel.RepoID)
		actualMember, err := memberRepo.GetMember(ctx, getMRParams)
		require.NoError(t, err)

		require.True(t, cmp.Equal(actualMember, newMemberModel, testhelper.DBTimeCmpOpt))
	})

	t.Run("user repo unique", func(t *testing.T) {
		memberModel := &models.Member{}
		require.NoError(t, gofakeit.Struct(memberModel))

		newMemberModel, err := memberRepo.Insert(ctx, memberModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newMemberModel.ID)

		memberModel.GroupID = uuid.New()
		_, err = memberRepo.Insert(ctx, memberModel)
		require.Error(t, err)
	})

	t.Run("list member", func(t *testing.T) {
		repoID := uuid.New()
		var members []*models.Member
		for i := 0; i < 10; i++ {
			memberModel := &models.Member{}
			require.NoError(t, gofakeit.Struct(memberModel))
			memberModel.RepoID = repoID
			memberModel.CreatedAt = time.Now()
			newMemberModel, err := memberRepo.Insert(ctx, memberModel)
			require.NoError(t, err)
			require.NotEqual(t, uuid.Nil, newMemberModel.ID)
			members = append(members, memberModel)
		}

		listMemberParams := models.NewListMembersParams().SetRepoID(repoID)
		listMembers, err := memberRepo.ListMember(ctx, listMemberParams)
		require.NoError(t, err)
		require.True(t, cmp.Equal(listMembers, utils.Reverse(members), testhelper.DBTimeCmpOpt))
	})

	t.Run("delete member", func(t *testing.T) {
		repoID := uuid.New()
		var firstUserID uuid.UUID
		for i := 0; i < 10; i++ {
			memberModel := &models.Member{}
			require.NoError(t, gofakeit.Struct(memberModel))
			memberModel.RepoID = repoID
			newMemberModel, err := memberRepo.Insert(ctx, memberModel)
			require.NoError(t, err)
			require.NotEqual(t, uuid.Nil, newMemberModel.ID)
			if i == 0 {
				firstUserID = memberModel.UserID
			}
		}

		//delete by repo and user
		deleteMemberParams := models.NewDeleteMemberParams().SetRepoID(repoID).SetUserID(firstUserID)
		deletedRows, err := memberRepo.DeleteMember(ctx, deleteMemberParams)
		require.NoError(t, err)
		require.Equal(t, 1, int(deletedRows))

		//delete by repo
		deleteMemberParams = models.NewDeleteMemberParams().SetRepoID(repoID)
		deletedRows, err = memberRepo.DeleteMember(ctx, deleteMemberParams)
		require.NoError(t, err)
		require.Equal(t, 9, int(deletedRows))

	})

	t.Run("update repo", func(t *testing.T) {
		memberModel := &models.Member{}
		require.NoError(t, gofakeit.Struct(memberModel))

		newMemberModel, err := memberRepo.Insert(ctx, memberModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newMemberModel.ID)

		newGroupID := uuid.New()
		updateMemberParams := models.NewUpdateMemberParams().SetFilterRepoID(memberModel.RepoID).SetFilterUserID(memberModel.UserID).SetUpdateGroupID(newGroupID)
		err = memberRepo.UpdateMember(ctx, updateMemberParams)
		require.NoError(t, err)

		member, err := memberRepo.GetMember(ctx, models.NewGetMemberParams().SetRepoID(memberModel.RepoID).SetUserID(memberModel.UserID))
		require.NoError(t, err)
		require.Equal(t, newGroupID, member.GroupID)

	})
}
