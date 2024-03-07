package rbacmodel_test

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/models/rbacmodel"
	"golang.org/x/exp/slices"
)

func TestAllActions(t *testing.T) {
	actions := rbacmodel.Actions

	if !slices.Contains(actions, rbacmodel.ReadUserAction) {
		t.Errorf("Expected actions %v to include %s", actions, rbacmodel.ReadUserAction)
	}

	if !slices.Contains(actions, rbacmodel.ReadRepositoryAction) {
		t.Errorf("Expected actions %v to include %s", actions, rbacmodel.ReadRepositoryAction)
	}

	if slices.Contains(actions, "IsValidAction") {
		t.Errorf("Expected actions %v not to include IsValidAction", actions)
	}
}

func TestIsValidAction(t *testing.T) {
	require.NoError(t, rbacmodel.IsValidAction("repo:test"))
	require.Error(t, rbacmodel.IsValidAction("repo"))
	require.Error(t, rbacmodel.IsValidAction("aaa:test"))
}
