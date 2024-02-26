package rbac_test

import (
	"testing"

	"github.com/jiaozifs/jiaozifs/models/rbac"
	"golang.org/x/exp/slices"
)

func TestAllActions(t *testing.T) {
	actions := rbac.Actions

	if !slices.Contains(actions, rbac.ReadUserAction) {
		t.Errorf("Expected actions %v to include %s", actions, rbac.ReadUserAction)
	}

	if !slices.Contains(actions, rbac.ReadRepositoryAction) {
		t.Errorf("Expected actions %v to include %s", actions, rbac.ReadRepositoryAction)
	}

	if slices.Contains(actions, "IsValidAction") {
		t.Errorf("Expected actions %v not to include IsValidAction", actions)
	}
}
