package rbac

import (
	"errors"
	"fmt"

	"github.com/GitDataAI/jiaozifs/models/rbacmodel"
)

var (
	ErrStatementNotFound = errors.New("statement not found")
)

// statementForPolicyType holds the Statement for a policy by its name,
// without the required ARN.
var statementByName = map[string]rbacmodel.Statement{
	"AllAccess": {
		Action: []string{"repo:*", "auth:*", "user:*"},
		Effect: rbacmodel.StatementEffectAllow,
	},
	"RepoReadWrite": {
		Action: []string{
			rbacmodel.ReadRepositoryAction,
			rbacmodel.UpdateRepositoryAction,
			rbacmodel.DeleteRepositoryAction,
			rbacmodel.ListRepositoriesAction,

			rbacmodel.ReadObjectAction,
			rbacmodel.WriteObjectAction,
			rbacmodel.DeleteObjectAction,
			rbacmodel.ListObjectsAction,

			rbacmodel.CreateCommitAction,
			rbacmodel.ReadCommitAction,
			rbacmodel.ListCommitsAction,

			rbacmodel.CreateBranchAction,
			rbacmodel.DeleteBranchAction,
			rbacmodel.ReadBranchAction,
			rbacmodel.ListBranchesAction,
			rbacmodel.WriteBranchAction,
			rbacmodel.DeleteWipAction,

			rbacmodel.CreateMergeRequestAction,
			rbacmodel.ReadMergeRequestAction,
			rbacmodel.UpdateMergeRequestAction,
			rbacmodel.ListMergeRequestAction,

			rbacmodel.ReadConfigAction,
			rbacmodel.WriteConfigAction,

			rbacmodel.ReadWipAction,
			rbacmodel.ListWipAction,
			rbacmodel.WriteWipAction,
			rbacmodel.CreateWipAction,
		},
		Effect: rbacmodel.StatementEffectAllow,
	},
	"RepoRead": {
		Action: []string{
			"repo:Read*",
			"repo:List*",
		},
		Effect: rbacmodel.StatementEffectAllow,
	},
	"RepoReadConfig": {
		Action: []string{
			rbacmodel.ReadConfigAction,
		},
		Effect: rbacmodel.StatementEffectAllow,
	},
	"RepoWriteConfig": {
		Action: []string{
			rbacmodel.ReadConfigAction,
			rbacmodel.WriteConfigAction,

			rbacmodel.AddGroupMemberAction,
			rbacmodel.RemoveGroupMemberAction,
		},
		Effect: rbacmodel.StatementEffectAllow,
	},
	"RepoMemberRead": {
		Action: []string{
			rbacmodel.GetGroupMemberAction,
			rbacmodel.ListGroupMemberAction,
		},
		Effect: rbacmodel.StatementEffectAllow,
	},
	"RepoMemberAccess": {
		Action: []string{
			rbacmodel.GetGroupMemberAction,
			rbacmodel.ListGroupMemberAction,
			rbacmodel.AddGroupMemberAction,
			rbacmodel.RemoveGroupMemberAction,
		},
		Effect: rbacmodel.StatementEffectAllow,
	},
	"UserFullAccess": {
		Action: []string{
			"auth:*",
			"user:*",
		},

		Effect: rbacmodel.StatementEffectAllow,
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
func MakeStatementForPolicyType(typ string, resources []rbacmodel.Resource) (rbacmodel.Statements, error) {
	statement, ok := statementByName[typ]
	if !ok {
		return nil, fmt.Errorf("%w: %s", ErrStatementNotFound, typ)
	}
	statements := make(rbacmodel.Statements, len(resources))
	for i, resource := range resources {
		if statement.Resource == "" {
			statements[i] = statement
			statements[i].Resource = resource
		}
	}
	return statements, nil
}

func MakeStatementForPolicyTypeOrDie(typ string, resources []rbacmodel.Resource) rbacmodel.Statements {
	statements, err := MakeStatementForPolicyType(typ, resources)
	if err != nil {
		panic(err)
	}
	return statements
}
