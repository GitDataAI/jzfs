package rbacModel_test

import (
	"testing"

	"github.com/jiaozifs/jiaozifs/models/rbacModel"
	"golang.org/x/exp/slices"
)

func TestAllActions(t *testing.T) {
	actions := rbacModel.Actions

	if !slices.Contains(actions, rbacModel.ReadUserAction) {
		t.Errorf("Expected actions %v to include %s", actions, rbacModel.ReadUserAction)
	}

	if !slices.Contains(actions, rbacModel.ReadRepositoryAction) {
		t.Errorf("Expected actions %v to include %s", actions, rbacModel.ReadRepositoryAction)
	}

	if slices.Contains(actions, "IsValidAction") {
		t.Errorf("Expected actions %v not to include IsValidAction", actions)
	}
}
