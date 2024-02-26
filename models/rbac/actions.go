//go:generate go run --tags generate ./codegen ./actions.go ./actions.gen.go

package rbac

import (
	"errors"
	"fmt"
	"strings"
)

var (
	ErrInvalidAction      = errors.New("invalid action")
	ErrInvalidServiceName = errors.New("invalid service name")
)

const (
	ReadRepositoryAction    = "repo:ReadRepository"
	CreateRepositoryAction  = "repo:CreateRepository"
	UpdateRepositoryAction  = "repo:UpdateRepository"
	DeleteRepositoryAction  = "repo:DeleteRepository"
	ListRepositoriesAction  = "repo:ListRepositories"
	ReadObjectAction        = "repo:ReadObject"
	WriteObjectAction       = "repo:WriteObject"
	DeleteObjectAction      = "repo:DeleteObject"
	ListObjectsAction       = "repo:ListObjects"
	CreateCommitAction      = "repo:CreateCommit"
	ReadCommitAction        = "repo:ReadCommit"
	ListCommitsAction       = "repo:ListCommits"
	CreateBranchAction      = "repo:CreateBranch"
	DeleteBranchAction      = "repo:DeleteBranch"
	ReadBranchAction        = "repo:ReadBranch"
	ListBranchesAction      = "repo:ListBranches"
	ReadConfigAction        = "repo:ReadConfig"
	UpdateConfigAction      = "repo:UpdateConfig"
	AddGroupMemberAction    = "repo:AddGroupMember"
	RemoveGroupMemberAction = "repo:RemoveGroupMember"

	ListUsersAction    = "auth:ListUsers"
	ReadGroupAction    = "auth:ReadGroup"
	CreateGroupAction  = "auth:CreateGroup"
	DeleteGroupAction  = "auth:DeleteGroup"
	ListGroupsAction   = "auth:ListGroups"
	ReadPolicyAction   = "auth:ReadPolicy"
	CreatePolicyAction = "auth:CreatePolicy"
	UpdatePolicyAction = "auth:UpdatePolicy"
	DeletePolicyAction = "auth:DeletePolicy"
	ListPoliciesAction = "auth:ListPolicies"
	AttachPolicyAction = "auth:AttachPolicy"
	DetachPolicyAction = "auth:DetachPolicy"

	UserProfileAction                        = "user:UserProfile"
	ReadUserAction                           = "user:ReadUser"
	DeleteUserAction                         = "user:DeleteUser"
	ReadCredentialsAction                    = "user:ReadCredentials"
	CreateCredentialsAction                  = "user:CreateCredentials"
	DeleteCredentialsDeleteCredentialsAction = "user:DeleteCredentials"
	ListCredentialsAction                    = "user:ListCredentials"
)

var serviceSet = map[string]struct{}{
	"repo": {},
	"auth": {},
	"user": {},
}

func IsValidAction(name string) error {
	parts := strings.Split(name, ":")
	const actionParts = 2
	if len(parts) != actionParts {
		return fmt.Errorf("%s: %w", name, ErrInvalidAction)
	}
	if _, ok := serviceSet[parts[0]]; !ok {
		return fmt.Errorf("%s: %w", name, ErrInvalidServiceName)
	}
	return nil
}
