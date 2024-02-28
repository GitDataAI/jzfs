package rbac

import (
	"errors"
	"fmt"

	"github.com/jiaozifs/jiaozifs/models/rbacModel"
)

var (
	ErrStatementNotFound = errors.New("statement not found")
)

// statementForPolicyType holds the Statement for a policy by its name,
// without the required ARN.
var statementByName = map[string]rbacModel.Statement{
	"AllAccess": {
		Action: []string{"repo:*", "auth:*", "user:*"},
		Effect: rbacModel.StatementEffectAllow,
	},
	"RepoReadWrite": {
		Action: []string{
			rbacModel.ReadRepositoryAction,
			rbacModel.UpdateRepositoryAction,
			rbacModel.DeleteRepositoryAction,
			rbacModel.ListRepositoriesAction,

			rbacModel.ReadObjectAction,
			rbacModel.WriteObjectAction,
			rbacModel.DeleteObjectAction,
			rbacModel.ListObjectsAction,

			rbacModel.CreateCommitAction,
			rbacModel.ReadCommitAction,
			rbacModel.ListCommitsAction,

			rbacModel.CreateBranchAction,
			rbacModel.DeleteBranchAction,
			rbacModel.ReadBranchAction,
			rbacModel.ListBranchesAction,
			rbacModel.WriteBranchAction,
			rbacModel.DeleteWipAction,

			rbacModel.CreateMergeRequestAction,
			rbacModel.ReadMergeRequestAction,
			rbacModel.UpdateMergeRequestAction,
			rbacModel.ListMergeRequestAction,

			rbacModel.ReadConfigAction,
			rbacModel.WriteConfigAction,

			rbacModel.ReadWipAction,
			rbacModel.ListWipAction,
			rbacModel.WriteWipAction,
			rbacModel.CreateWipAction,
		},
		Effect: rbacModel.StatementEffectAllow,
	},
	"RepoRead": {
		Action: []string{
			"repo:Read*",
			"repo:List*",
		},
		Effect: rbacModel.StatementEffectAllow,
	},
	"RepoReadConfig": {
		Action: []string{
			rbacModel.ReadConfigAction,
		},
		Effect: rbacModel.StatementEffectAllow,
	},
	"RepoWriteConfig": {
		Action: []string{
			rbacModel.ReadConfigAction,
			rbacModel.WriteConfigAction,

			rbacModel.AddGroupMemberAction,
			rbacModel.RemoveGroupMemberAction,
		},
		Effect: rbacModel.StatementEffectAllow,
	},
	"RepoMemberRead": {
		Action: []string{
			rbacModel.GetGroupMemberAction,
		},
		Effect: rbacModel.StatementEffectAllow,
	},
	"RepoMemberAccess": {
		Action: []string{
			rbacModel.GetGroupMemberAction,
			rbacModel.AddGroupMemberAction,
			rbacModel.RemoveGroupMemberAction,
		},
		Effect: rbacModel.StatementEffectAllow,
	},
	"UserFullAccess": {
		Action: []string{
			"auth:*",
			"user:*",
		},

		Effect: rbacModel.StatementEffectAllow,
	},
}

// GetActionsForPolicyType returns the actions for police type typ.
func GetActionsForPolicyType(typ string) ([]string, error) {
	statement, ok := statementByName[typ]
	if !ok {
		return nil, fmt.Errorf("%w: %s", ErrStatementNotFound, typ)
	}
	actions := make([]string, len(statement.Action))
	copy(actions, statement.Action)
	return actions, nil
}

func GetActionsForPolicyTypeOrDie(typ string) []string {
	ret, err := GetActionsForPolicyType(typ)
	if err != nil {
		panic(err)
	}
	return ret
}

// MakeStatementForPolicyType returns statements for policy type typ,
// limited to resources.
func MakeStatementForPolicyType(typ string, resources []rbacModel.Resource) (rbacModel.Statements, error) {
	statement, ok := statementByName[typ]
	if !ok {
		return nil, fmt.Errorf("%w: %s", ErrStatementNotFound, typ)
	}
	statements := make(rbacModel.Statements, len(resources))
	for i, resource := range resources {
		if statement.Resource == "" {
			statements[i] = statement
			statements[i].Resource = resource
		}
	}
	return statements, nil
}

func MakeStatementForPolicyTypeOrDie(typ string, resources []rbacModel.Resource) rbacModel.Statements {
	statements, err := MakeStatementForPolicyType(typ, resources)
	if err != nil {
		panic(err)
	}
	return statements
}
