package rbacmodel_test

import (
	"context"
	"sort"
	"testing"

	"github.com/GitDataAI/jiaozifs/models/rbacmodel"
	"github.com/GitDataAI/jiaozifs/testhelper"
	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/go-cmp/cmp"
	"github.com/google/uuid"
	"github.com/stretchr/testify/require"
)

func TestPolicyRepo(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	policyRepo := rbacmodel.NewPolicyRepo(db)

	t.Run("insert and get ", func(t *testing.T) {
		policyModel := &rbacmodel.Policy{}
		require.NoError(t, gofakeit.Struct(policyModel))

		newPolicyModel, err := policyRepo.Insert(ctx, policyModel)
		require.NoError(t, err)
		require.NotEqual(t, uuid.Nil, newPolicyModel.ID)

		getPloicyParams := rbacmodel.NewGetPolicyParams().SetID(newPolicyModel.ID)
		actualPolicy, err := policyRepo.Get(ctx, getPloicyParams)
		require.NoError(t, err)
		require.True(t, cmp.Equal(actualPolicy, newPolicyModel, testhelper.DBTimeCmpOpt))
	})

	t.Run("list", func(t *testing.T) {
		var ids []uuid.UUID
		var policies []*rbacmodel.Policy
		for i := 0; i < 10; i++ {
			policyModel := &rbacmodel.Policy{}
			require.NoError(t, gofakeit.Struct(policyModel))

			newPolicyModel, err := policyRepo.Insert(ctx, policyModel)
			require.NoError(t, err)
			require.NotEqual(t, uuid.Nil, newPolicyModel.ID)
			ids = append(ids, newPolicyModel.ID)
			policies = append(policies, newPolicyModel)
		}

		actualPolicies, err := policyRepo.List(ctx, rbacmodel.NewListPolicyParams().SetIDs(ids...))
		require.NoError(t, err)

		sort.Slice(policies, func(i, j int) bool {
			return policies[i].CreatedAt.Sub(policies[j].CreatedAt) > 0
		})
		require.True(t, cmp.Equal(actualPolicies, policies, testhelper.DBTimeCmpOpt))
	})
}
