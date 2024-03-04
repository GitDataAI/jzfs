package rbac

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"

	"github.com/jiaozifs/jiaozifs/auth/rbac/wildcard"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/rbacmodel"
)

var ErrInsufficientPermissions = fmt.Errorf("permission not enough")

// CheckResult - the final result for the authorization is accepted only if it's CheckAllow
type CheckResult int

type BuiltinGroupName = string

const (
	// Super do anything in system
	Super BuiltinGroupName = "Super"
	// RepoAdmin do anything in this repo
	RepoAdmin BuiltinGroupName = "RepoAdmin"
	// RepoWrite read and write in this repo
	RepoWrite BuiltinGroupName = "RepoWrite"
	// RepoRead only read in this repo
	RepoRead BuiltinGroupName = "RepoRead"
	// UserOwnAccess could manage user, credential, create repo
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
	Resource rbacmodel.Resource
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

type RbacAuth struct { //nolint
	db models.IRepo
}

func NewRbacAuth(IRepo models.IRepo) *RbacAuth {
	return &RbacAuth{db: IRepo}
}

func (s *RbacAuth) InitRbac(ctx context.Context, adminUser *models.User) error {
	all := []rbacmodel.Resource{rbacmodel.All}
	return s.db.Transaction(ctx, func(repo models.IRepo) error {
		//add super
		superGroup, err := s.addGroupPolicy(ctx, repo, Super, &rbacmodel.Policy{
			Name:       Super,
			Statements: MakeStatementForPolicyTypeOrDie("AllAccess", all),
			CreatedAt:  time.Now(),
			UpdatedAt:  time.Now(),
		})
		if err != nil {
			return err
		}

		//add repo admin
		_, err = s.addGroupPolicy(ctx, repo, RepoAdmin, &rbacmodel.Policy{
			Name: RepoAdmin,
			Statements: rbacmodel.Statements{
				{
					Action: []string{
						"repo:*",
					},
					Resource: rbacmodel.RepoURArn(rbacmodel.UserIDCapture, rbacmodel.RepoIDCapture),
					Effect:   rbacmodel.StatementEffectAllow,
				},
			},
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		if err != nil {
			return err
		}
		// add repo write
		_, err = s.addGroupPolicy(ctx, repo, RepoWrite, &rbacmodel.Policy{
			Name:       RepoWrite,
			Statements: MakeStatementForPolicyTypeOrDie("RepoReadWrite", []rbacmodel.Resource{rbacmodel.RepoURArn(rbacmodel.UserIDCapture, rbacmodel.RepoIDCapture)}),
			CreatedAt:  time.Now(),
			UpdatedAt:  time.Now(),
		})
		if err != nil {
			return err
		}

		// add repo read
		_, err = s.addGroupPolicy(ctx, repo, RepoRead, &rbacmodel.Policy{
			Name:       RepoRead,
			Statements: MakeStatementForPolicyTypeOrDie("RepoRead", []rbacmodel.Resource{rbacmodel.RepoURArn(rbacmodel.UserIDCapture, rbacmodel.RepoIDCapture)}),
			CreatedAt:  time.Now(),
			UpdatedAt:  time.Now(),
		})
		if err != nil {
			return err
		}

		userOwner := MakeStatementForPolicyTypeOrDie("UserFullAccess", []rbacmodel.Resource{
			rbacmodel.UserArn(rbacmodel.UserIDCapture),
			rbacmodel.UserAkskArn(rbacmodel.UserIDCapture),
		})
		_, err = s.addGroupPolicy(ctx, repo, UserOwnAccess, &rbacmodel.Policy{
			Name: UserOwnAccess,
			Statements: append(userOwner, rbacmodel.Statement{
				Action: []string{
					rbacmodel.CreateRepositoryAction,
					"repo:*",
				},
				Resource: rbacmodel.RepoUArn(rbacmodel.UserIDCapture),
				Effect:   rbacmodel.StatementEffectAllow,
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

		_, err = repo.UserGroupRepo().Insert(ctx, &rbacmodel.UserGroup{
			UserID:    adminUser.ID,
			GroupID:   superGroup.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		return err
	})
}

func (s *RbacAuth) addGroupPolicy(ctx context.Context, repo models.IRepo, groupName string, policies ...*rbacmodel.Policy) (*rbacmodel.Group, error) {
	var policyIds []uuid.UUID
	for _, policy := range policies {
		_, err := repo.PolicyRepo().Insert(ctx, policy)
		if err != nil {
			return nil, err
		}
		policyIds = append(policyIds, policy.ID)
	}

	return repo.GroupRepo().Insert(ctx, &rbacmodel.Group{
		Name:      groupName,
		Policies:  policyIds,
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	})
}

type AuthorizationRequest struct {
	OperatorID          uuid.UUID
	RequiredPermissions Node
}

type AuthorizationResponse struct {
	Allowed bool
	Error   error
}

func (s *RbacAuth) listEffectivePolicies(ctx context.Context, userID uuid.UUID) ([]*rbacmodel.Policy, error) {
	group, err := s.db.GroupRepo().GetGroupByUserID(ctx, userID)
	if err != nil {
		return nil, err
	}

	//get group policy
	policies, err := s.db.PolicyRepo().List(ctx, rbacmodel.NewListPolicyParams().SetIDs(group.Policies...))
	if err != nil {
		return nil, err
	}
	return policies, err
}

func (s *RbacAuth) Authorize(ctx context.Context, req *AuthorizationRequest) (*AuthorizationResponse, error) {
	policies, err := s.listEffectivePolicies(ctx, req.OperatorID)
	if err != nil {
		return nil, err
	}

	allowed := checkPermissions(ctx, req.RequiredPermissions, ResourceParams{UserID: req.OperatorID}, policies)

	if allowed != CheckAllow {
		return &AuthorizationResponse{
			Allowed: false,
			Error:   ErrInsufficientPermissions,
		}, nil
	}

	// we're allowed!
	return &AuthorizationResponse{Allowed: true}, nil
}

type ResourceParams struct {
	UserID uuid.UUID
	RepoID uuid.UUID
}

func (rp ResourceParams) Render(resource rbacmodel.Resource) rbacmodel.Resource {
	if rp.UserID != uuid.Nil {
		resource = resource.WithUserID(rp.UserID.String())
	}

	if rp.RepoID != uuid.Nil {
		resource = resource.WithRepoID(rp.RepoID.String())
	}
	return resource
}

func (s *RbacAuth) getMemberPolicy(ctx context.Context, operatorID uuid.UUID, repoID uuid.UUID) ([]*rbacmodel.Policy, error) {
	member, err := s.db.MemberRepo().GetMember(ctx, models.NewGetMemberParams().SetUserID(operatorID).SetRepoID(repoID))
	if err != nil {
		return nil, err
	}

	group, err := s.db.GroupRepo().Get(ctx, rbacmodel.NewGetGroupParams().SetID(member.GroupID))
	if err != nil {
		return nil, err
	}

	policy, err := s.db.PolicyRepo().List(ctx, rbacmodel.NewListPolicyParams().SetIDs(group.Policies...))
	if err != nil {
		return nil, err
	}
	return policy, err
}

func (s *RbacAuth) AuthorizeMember(ctx context.Context, repoID uuid.UUID, req *AuthorizationRequest) (*AuthorizationResponse, error) {
	repo, err := s.db.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetID(repoID))
	if err != nil {
		return nil, err
	}

	if repo.OwnerID == req.OperatorID {
		//owner has all permission
		return &AuthorizationResponse{Allowed: true}, nil
	}

	policies, err := s.getMemberPolicy(ctx, req.OperatorID, repoID)
	if err != nil {
		if errors.Is(err, models.ErrNotFound) {
			return &AuthorizationResponse{
				Allowed: false,
				Error:   ErrInsufficientPermissions,
			}, nil
		}
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
}

func checkPermissions(ctx context.Context, node Node, params ResourceParams, policies []*rbacmodel.Policy) CheckResult {
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

					if stmt.Effect == rbacmodel.StatementEffectDeny {
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
