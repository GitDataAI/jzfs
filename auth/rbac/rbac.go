package rbac

import (
	"context"
	"fmt"
	"time"

	"github.com/google/uuid"

	"github.com/jiaozifs/jiaozifs/auth/rbac/wildcard"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/rbacModel"
)

var ErrInsufficientPermissions = fmt.Errorf("permission not enough")

// CheckResult - the final result for the authorization is accepted only if it's CheckAllow
type CheckResult int

type BuiltinGroupName = string

const (
	Super         BuiltinGroupName = "Super"
	RepoAdmin     BuiltinGroupName = "RepoAdmin" //do anything in repo
	RepoWrite     BuiltinGroupName = "RepoWrite"
	RepoRead      BuiltinGroupName = "RepoRead"
	UserOwnAccess BuiltinGroupName = "UserOwnAccess"
)
const (
	InvalidUserID = ""
	maxPage       = 1000
	// CheckAllow Permission allowed
	CheckAllow CheckResult = iota
	// CheckNeutral Permission neither allowed nor denied
	CheckNeutral
	// CheckDeny Permission denied
	CheckDeny
)

type Permission struct {
	Action   string
	Resource rbacModel.Resource
}

type NodeType int

const (
	NodeTypeNode NodeType = iota
	NodeTypeOr
	NodeTypeAnd
)

type Node struct {
	Type       NodeType
	Permission Permission
	Nodes      []Node
}

type PermissionCheck interface {
	Authorize(ctx context.Context, req *AuthorizationRequest) (*AuthorizationResponse, error)
	AuthorizeMember(ctx context.Context, repoID uuid.UUID, req *AuthorizationRequest) (*AuthorizationResponse, error)
}

var _ PermissionCheck = (*RbacAuth)(nil)

type RbacAuth struct {
	db models.IRepo
}

func NewRbacAuth(IRepo models.IRepo) *RbacAuth {
	return &RbacAuth{db: IRepo}
}

func (s *RbacAuth) InitRbac(ctx context.Context, adminUser *models.User) error {
	all := []rbacModel.Resource{rbacModel.All}
	return s.db.Transaction(ctx, func(repo models.IRepo) error {
		//add super
		superGroup, err := s.addGroupPolicy(ctx, repo, Super, &rbacModel.Policy{
			Name:       Super,
			Statements: MakeStatementForPolicyTypeOrDie("AllAccess", all),
			CreatedAt:  time.Now(),
			UpdatedAt:  time.Now(),
		})
		if err != nil {
			return err
		}

		//add repo admin
		_, err = s.addGroupPolicy(ctx, repo, RepoAdmin, &rbacModel.Policy{
			Name: RepoAdmin,
			Statements: rbacModel.Statements{
				{
					Action: []string{
						"repo:*",
					},
					Resource: rbacModel.RepoURArn(rbacModel.UserIDCapture, rbacModel.RepoIDCapture),
					Effect:   rbacModel.StatementEffectAllow,
				},
			},
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		if err != nil {
			return err
		}
		// add repo write
		_, err = s.addGroupPolicy(ctx, repo, RepoWrite, &rbacModel.Policy{
			Name:       RepoWrite,
			Statements: MakeStatementForPolicyTypeOrDie("RepoReadWrite", []rbacModel.Resource{rbacModel.RepoURArn(rbacModel.UserIDCapture, rbacModel.RepoIDCapture)}),
			CreatedAt:  time.Now(),
			UpdatedAt:  time.Now(),
		})
		if err != nil {
			return err
		}

		// add repo read
		_, err = s.addGroupPolicy(ctx, repo, RepoRead, &rbacModel.Policy{
			Name:       RepoRead,
			Statements: MakeStatementForPolicyTypeOrDie("RepoRead", []rbacModel.Resource{rbacModel.RepoURArn(rbacModel.UserIDCapture, rbacModel.RepoIDCapture)}),
			CreatedAt:  time.Now(),
			UpdatedAt:  time.Now(),
		})
		if err != nil {
			return err
		}

		userOwner := MakeStatementForPolicyTypeOrDie("UserFullAccess", []rbacModel.Resource{rbacModel.UserArn(rbacModel.UserIDCapture)})
		_, err = s.addGroupPolicy(ctx, repo, UserOwnAccess, &rbacModel.Policy{
			Name: UserOwnAccess,
			Statements: append(userOwner, rbacModel.Statement{
				Action: []string{
					rbacModel.CreateRepositoryAction,
					"repo:*",
				},
				Resource: rbacModel.RepoUArn(rbacModel.UserIDCapture),
				Effect:   rbacModel.StatementEffectAllow,
			}),
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		if err != nil {
			return err
		}

		adminUser, err = repo.UserRepo().Insert(ctx, adminUser)
		if err != nil {
			return err
		}

		_, err = repo.UserGroupRepo().Insert(ctx, &rbacModel.UserGroup{
			UserID:    adminUser.ID,
			GroupID:   superGroup.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		return err
	})
}

func (s *RbacAuth) addGroupPolicy(ctx context.Context, repo models.IRepo, groupName string, policies ...*rbacModel.Policy) (*rbacModel.Group, error) {
	var policyIds []uuid.UUID
	for _, policy := range policies {
		_, err := repo.PolicyRepo().Insert(ctx, policy)
		if err != nil {
			return nil, err
		}
		policyIds = append(policyIds, policy.ID)
	}

	return repo.GroupRepo().Insert(ctx, &rbacModel.Group{
		Name:      groupName,
		Policies:  policyIds,
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	})
}

type AuthorizationRequest struct {
	UserID              uuid.UUID
	RequiredPermissions Node
}

type AuthorizationResponse struct {
	Allowed bool
	Error   error
}

func (s *RbacAuth) listEffectivePolicies(ctx context.Context, userID uuid.UUID) ([]*rbacModel.Policy, error) {
	group, err := s.db.GroupRepo().GetGroupByUserID(ctx, userID)
	if err != nil {
		return nil, err
	}

	//get group policy
	policies, err := s.db.PolicyRepo().List(ctx, rbacModel.NewListPolicyParams().SetIDs(group.Policies...))
	if err != nil {
		return nil, err
	}
	return policies, err
}

func (s *RbacAuth) getMemberPolicy(ctx context.Context, userID uuid.UUID, repoID uuid.UUID) ([]*rbacModel.Policy, *models.Repository, error) {
	member, err := s.db.MemberRepo().GetMember(ctx, models.NewGetMemberParams().SetUserID(userID).SetRepoID(repoID))
	if err != nil {
		return nil, nil, err
	}

	repo, err := s.db.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetID(member.RepoID))
	if err != nil {
		return nil, nil, err
	}

	group, err := s.db.GroupRepo().Get(ctx, rbacModel.NewGetGroupParams().SetID(member.GroupID))
	if err != nil {
		return nil, nil, err
	}

	policy, err := s.db.PolicyRepo().List(ctx, rbacModel.NewListPolicyParams().SetIDs(group.Policies...))
	if err != nil {
		return nil, nil, err
	}
	return policy, repo, err
}

func (s *RbacAuth) Authorize(ctx context.Context, req *AuthorizationRequest) (*AuthorizationResponse, error) {
	policies, err := s.listEffectivePolicies(ctx, req.UserID)
	if err != nil {
		return nil, err
	}

	allowed := checkPermissions(ctx, req.RequiredPermissions, ResourceParams{UserID: req.UserID}, policies)

	if allowed != CheckAllow {
		return &AuthorizationResponse{
			Allowed: false,
			Error:   ErrInsufficientPermissions,
		}, nil
	}

	// we're allowed!
	return &AuthorizationResponse{Allowed: true}, nil
	return nil, nil
}

type ResourceParams struct {
	UserID uuid.UUID
	RepoID uuid.UUID
}

func (rp ResourceParams) Render(resource rbacModel.Resource) rbacModel.Resource {
	if rp.UserID != uuid.Nil {
		resource = resource.WithUserID(rp.UserID.String())
	}

	if rp.RepoID != uuid.Nil {
		resource = resource.WithRepoID(rp.RepoID.String())
	}
	return resource
}

func (s *RbacAuth) AuthorizeMember(ctx context.Context, repoID uuid.UUID, req *AuthorizationRequest) (*AuthorizationResponse, error) {
	policies, repo, err := s.getMemberPolicy(ctx, req.UserID, repoID)
	if err != nil {
		return nil, err
	}

	resourceParams := ResourceParams{UserID: repo.OwnerID, RepoID: repoID}
	allowed := checkPermissions(ctx, req.RequiredPermissions, resourceParams, policies)

	if allowed != CheckAllow {
		return &AuthorizationResponse{
			Allowed: false,
			Error:   ErrInsufficientPermissions,
		}, nil
	}

	// we're allowed!
	return &AuthorizationResponse{Allowed: true}, nil
	return nil, nil
}

func checkPermissions(ctx context.Context, node Node, params ResourceParams, policies []*rbacModel.Policy) CheckResult {
	allowed := CheckNeutral
	switch node.Type {
	case NodeTypeNode:
		// check whether the permission is allowed, denied or natural (not allowed and not denied)
		for _, policy := range policies {
			for _, stmt := range policy.Statements {
				resource := params.Render(stmt.Resource)
				if !ArnMatch(resource.String(), node.Permission.Resource.String()) {
					continue
				}
				for _, action := range stmt.Action {
					if !wildcard.Match(action, node.Permission.Action) {
						continue // not a matching action
					}

					if stmt.Effect == rbacModel.StatementEffectDeny {
						// this is a "Deny" and it takes precedence
						return CheckDeny
					}

					allowed = CheckAllow
				}
			}
		}

	case NodeTypeOr:
		// returns:
		// Allowed - at least one of the permissions is allowed and no one is denied
		// Denied - one of the permissions is Deny
		// Natural - otherwise
		for _, node := range node.Nodes {
			result := checkPermissions(ctx, node, params, policies)
			if result == CheckDeny {
				return CheckDeny
			}
			if allowed != CheckAllow {
				allowed = result
			}
		}

	case NodeTypeAnd:
		// returns:
		// Allowed - all the permissions are allowed
		// Denied - one of the permissions is Deny
		// Natural - otherwise
		for _, node := range node.Nodes {
			result := checkPermissions(ctx, node, params, policies)
			if result == CheckNeutral || result == CheckDeny {
				return result
			}
		}
		return CheckAllow

	default:
		return CheckDeny
	}
	return allowed
}
