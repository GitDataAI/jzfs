package models_test

import (
	"context"
	"testing"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
)

func TestMemberRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	memberRepo := models.NewMemberRepo(db)

	memberModel := &models.Member{}
	require.NoError(t, gofakeit.Struct(memberModel))

	newMemberModel, err := memberRepo.Insert(ctx, memberModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newMemberModel.ID)

	getMRParams := models.NewGetMemberParams().SetID(newMemberModel.ID).SetUserID(newMemberModel.UserID).SetRepoID(newMemberModel.RepoID)
	actualMember, err := memberRepo.GetMember(ctx, getMRParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(actualMember, newMemberModel, testhelper.DbTimeCmpOpt))
}
