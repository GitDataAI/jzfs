package models_test

import (
	"context"
	"testing"
	"time"

	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/testhelper"
	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"
)

func TestTagRepoInsert(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	repo := models.NewTagRepo(db)

	tagModel := &models.Tag{}
	require.NoError(t, gofakeit.Struct(tagModel))
	tagModel.Name = "atagName"
	tagModel.UpdatedAt = time.Now()
	newTag, err := repo.Insert(ctx, tagModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, newTag.ID)

	getTagParams := models.NewGetTagParams().
		SetID(newTag.ID).
		SetRepositoryID(newTag.RepositoryID).
		SetName(newTag.Name)
	branch, err := repo.Get(ctx, getTagParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(tagModel, branch, testhelper.DBTimeCmpOpt))

	list, _, err := repo.List(ctx, models.NewListTagParams().SetRepositoryID(branch.RepositoryID))
	require.NoError(t, err)
	require.Len(t, list, 1)

	// SecondModel
	secModel := &models.Tag{}
	require.NoError(t, gofakeit.Struct(secModel))
	secModel.RepositoryID = branch.RepositoryID
	secModel.Name = "feat_bba_ccc"
	secModel.UpdatedAt = time.Now()
	secRef, err := repo.Insert(ctx, secModel)
	require.NoError(t, err)
	require.NotEqual(t, uuid.Nil, secRef.ID)

	getSecRefParams := models.NewGetTagParams().
		SetID(secRef.ID).
		SetRepositoryID(secRef.RepositoryID).
		SetName(secRef.Name)
	sRef, err := repo.Get(ctx, getSecRefParams)
	require.NoError(t, err)

	require.True(t, cmp.Equal(secModel, sRef, testhelper.DBTimeCmpOpt))

	// ExactMatch
	list1, hasMore, err := repo.List(ctx, models.NewListTagParams().SetRepositoryID(branch.RepositoryID).SetName(secModel.Name, models.ExactMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list1, 1)
	require.True(t, hasMore)

	// PrefixMatch
	list2, hasMore, err := repo.List(ctx, models.NewListTagParams().SetRepositoryID(branch.RepositoryID).SetName(secModel.Name[:3], models.PrefixMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list2, 1)
	require.True(t, hasMore)

	// SuffixMatch
	list3, hasMore, err := repo.List(ctx, models.NewListTagParams().SetRepositoryID(branch.RepositoryID).SetName(secModel.Name[3:], models.SuffixMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list3, 1)
	require.True(t, hasMore)

	// LikeMatch
	list4, hasMore, err := repo.List(ctx, models.NewListTagParams().SetRepositoryID(branch.RepositoryID).SetName(secModel.Name[2:4], models.LikeMatch).SetAmount(1))
	require.NoError(t, err)
	require.Len(t, list4, 1)
	require.True(t, hasMore)

	// After
	list5, hasMore, err := repo.List(ctx, models.NewListTagParams().SetRepositoryID(branch.RepositoryID).SetAfter(tagModel.UpdatedAt))
	require.NoError(t, err)
	require.Len(t, list5, 1)
	require.False(t, hasMore)

	affectedRows, err := repo.Delete(ctx, models.NewDeleteTagParams().SetRepositoryID(list[0].RepositoryID).SetID(secModel.ID))
	require.NoError(t, err)
	require.Equal(t, int64(1), affectedRows)

	list6, _, err := repo.List(ctx, models.NewListTagParams().SetRepositoryID(branch.RepositoryID))
	require.NoError(t, err)
	require.Len(t, list6, 1)
}
