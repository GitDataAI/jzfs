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

func TestMergeRequestRepoInsert(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	mrRepo := models.NewMergeRequestRepo(db)

	mrModel := &models.MergeRequest{}
	require.NoError(t, gofakeit.Struct(mrModel))
	newMrModel, err := mrRepo.Insert(ctx, mrModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newMrModel.ID)

	getMRParams := models.NewGetMergeRequestParams().
		SetID(newMrModel.ID)
	mrModel, err = mrRepo.Get(ctx, getMRParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(mrModel, newMrModel, dbTimeCmpOpt))
}
