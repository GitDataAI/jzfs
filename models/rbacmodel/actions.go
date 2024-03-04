//go:generate go run --tags generate ./codegen ./actions.go ./actions.gen.go

package rbacmodel

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
	ReadRepositoryAction   = "repo:ReadRepository"
	CreateRepositoryAction = "repo:CreateRepository"
	UpdateRepositoryAction = "repo:UpdateRepository"
	DeleteRepositoryAction = "repo:DeleteRepository"
	ListRepositoriesAction = "repo:ListRepositories"

	ReadObjectAction   = "repo:ReadObject"
	WriteObjectAction  = "repo:WriteObject"
	DeleteObjectAction = "repo:DeleteObject"
	ListObjectsAction  = "repo:ListObjects"

	CreateCommitAction = "repo:CreateCommit"
	ReadCommitAction   = "repo:ReadCommit"
	ListCommitsAction  = "repo:ListCommits"

	CreateBranchAction = "repo:CreateBranch"
	DeleteBranchAction = "repo:DeleteBranch"
	ReadBranchAction   = "repo:ReadBranch"
	WriteBranchAction  = "repo:ReadBranch"
	ListBranchesAction = "repo:ListBranches"

	ReadWipAction   = "repo:GetWip"
	ListWipAction   = "repo:ListWip"
	WriteWipAction  = "repo:WriteWip"
	CreateWipAction = "repo:CreateWip"
	DeleteWipAction = "repo:DeleteWip"

	ReadConfigAction  = "repo:ReadConfig"
	WriteConfigAction = "repo:WriteConfig"

	CreateMergeRequestAction = "repo:CreateMergeRequest"
	ReadMergeRequestAction   = "repo:ReadMergeRequest"
	UpdateMergeRequestAction = "repo:UpdateMergeRequest"
	ListMergeRequestAction   = "repo:ListMergeRequest"
	MergeMergeRequestAction  = "repo:MergeMergeRequest"

	AddGroupMemberAction    = "repo:AddGroupMember"
	RemoveGroupMemberAction = "repo:RemoveGroupMember"
	GetGroupMemberAction    = "repo:GetGroupMember"
	ListGroupMemberAction   = "repo:GetGroupMember"

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

	UserProfileAction       = "user:UserProfile"
	ReadUserAction          = "user:ReadUser"
	ListUsersAction         = "user:ListUsers"
	DeleteUserAction        = "user:DeleteUser"
	ReadCredentialsAction   = "user:ReadCredentials"
	CreateCredentialsAction = "user:CreateCredentials"
	DeleteCredentialsAction = "user:DeleteCredentials"
	ListCredentialsAction   = "user:ListCredentials"
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
