package rbac_test

import (
	"context"
	"testing"
	"time"

	"github.com/google/uuid"

	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/models/rbacmodel"

	"github.com/jiaozifs/jiaozifs/auth/rbac"
	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/testhelper"
)

func TestNewRbac(t *testing.T) {
	ctx := context.Background()
	closeDB, _, db := testhelper.SetupDatabase(ctx, t)
	defer closeDB()

	dbRepo := models.NewRepo(db)
	rbacChecker := rbac.NewRbacAuth(dbRepo)

	password, err := auth.HashPassword("123456789")
	require.NoError(t, err)
	superUser := &models.User{
		Name:              "admin",
		Email:             "",
		EncryptedPassword: string(password),
		CurrentSignInAt:   time.Now(),
		LastSignInAt:      time.Now(),
		CreatedAt:         time.Now(),
		UpdatedAt:         time.Now(),
	}
	err = rbacChecker.InitRbac(ctx, superUser)
	require.NoError(t, err)

	addCommonUser := func(name string) *models.User {
		commonUser := &models.User{
			Name:              name,
			Email:             name + "@test.com",
			EncryptedPassword: string(password),
			CurrentSignInAt:   time.Now(),
			LastSignInAt:      time.Now(),
			CreatedAt:         time.Now(),
			UpdatedAt:         time.Now(),
		}
		commonUser, err = dbRepo.UserRepo().Insert(ctx, commonUser)
		require.NoError(t, err)

		userOwnGroup, err := dbRepo.GroupRepo().Get(ctx, rbacmodel.NewGetGroupParams().SetName(rbac.UserOwnAccess))
		require.NoError(t, err)
		//bind own user group
		_, err = dbRepo.UserGroupRepo().Insert(ctx, &rbacmodel.UserGroup{
			UserID:    commonUser.ID,
			GroupID:   userOwnGroup.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		require.NoError(t, err)
		return commonUser
	}

	addRepo := func(name string, ownerID uuid.UUID, isPublic bool) *models.Repository {
		repo, err := dbRepo.RepositoryRepo().Insert(ctx, &models.Repository{
			Name:    name,
			OwnerID: ownerID,
			Visible: isPublic,
			HEAD:    "master",
		})
		require.NoError(t, err)
		return repo
	}

	t.Run("super user", func(t *testing.T) {
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			OperatorID: superUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.ReadUserAction,
					Resource: rbacmodel.UserArn(superUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("super user control other user", func(t *testing.T) {
		commonUser := &models.User{
			Name:              "common",
			Email:             "test@test.com",
			EncryptedPassword: string(password),
			CurrentSignInAt:   time.Now(),
			LastSignInAt:      time.Now(),
			CreatedAt:         time.Now(),
			UpdatedAt:         time.Now(),
		}
		commonUser, err = dbRepo.UserRepo().Insert(ctx, commonUser)
		require.NoError(t, err)

		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			OperatorID: superUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.ReadUserAction,
					Resource: rbacmodel.UserArn(commonUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("common user control himself", func(t *testing.T) {
		commonUser := addCommonUser("common1")
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.ReadUserAction,
					Resource: rbacmodel.UserArn(commonUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("common user cannot control other user", func(t *testing.T) {
		commonUser := addCommonUser("common2")
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.ReadUserAction,
					Resource: rbacmodel.UserArn(superUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.Equal(t, rbac.ErrInsufficientPermissions, resp.Error)
		require.False(t, resp.Allowed)
	})

	t.Run("create repo", func(t *testing.T) {
		commonUser := addCommonUser("common3")
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.CreateRepositoryAction,
					Resource: rbacmodel.RepoUArn(commonUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("create branch in own user", func(t *testing.T) {
		repoID := uuid.New()
		commonUser := addCommonUser("common4")

		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.CreateBranchAction,
					Resource: rbacmodel.RepoURArn(commonUser.ID.String(), repoID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("super create branch in others's repo", func(t *testing.T) {
		commonUser := addCommonUser("common5")
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			OperatorID: superUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.CreateRepositoryAction,
					Resource: rbacmodel.RepoUArn(commonUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("common user cannot create branch in other user", func(t *testing.T) {
		commonUser := addCommonUser("common6")
		other1User := addCommonUser("other1")
		repo := addRepo("aaa", other1User.ID, false)
		resp, err := rbacChecker.AuthorizeMember(ctx, repo.ID, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.CreateBranchAction,
					Resource: rbacmodel.RepoURArn(other1User.ID.String(), repo.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.Equal(t, rbac.ErrInsufficientPermissions, resp.Error)
		require.False(t, resp.Allowed)
	})

	t.Run("can create branch in other user after add in member with write grant", func(t *testing.T) {
		commonUser := addCommonUser("common7")
		other1User := addCommonUser("other2")
		repo := addRepo("aaa", other1User.ID, false)
		repoWriteGroup, err := dbRepo.GroupRepo().Get(ctx, rbacmodel.NewGetGroupParams().SetName(rbac.RepoWrite))
		require.NoError(t, err)
		_, err = dbRepo.MemberRepo().Insert(ctx, &models.Member{
			UserID:    commonUser.ID,
			RepoID:    repo.ID,
			GroupID:   repoWriteGroup.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		require.NoError(t, err)

		resp, err := rbacChecker.AuthorizeMember(ctx, repo.ID, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.CreateBranchAction,
					Resource: rbacmodel.RepoURArn(other1User.ID.String(), repo.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("can not create branch in other user after add in member with read grant", func(t *testing.T) {
		commonUser := addCommonUser("common8")
		other1User := addCommonUser("other3")
		repo := addRepo("aaa", other1User.ID, false)
		repoWriteGroup, err := dbRepo.GroupRepo().Get(ctx, rbacmodel.NewGetGroupParams().SetName(rbac.RepoRead))
		require.NoError(t, err)
		_, err = dbRepo.MemberRepo().Insert(ctx, &models.Member{
			UserID:    commonUser.ID,
			RepoID:    repo.ID,
			GroupID:   repoWriteGroup.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		require.NoError(t, err)

		resp, err := rbacChecker.AuthorizeMember(ctx, repo.ID, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.CreateBranchAction,
					Resource: rbacmodel.RepoURArn(other1User.ID.String(), repo.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.Equal(t, rbac.ErrInsufficientPermissions, resp.Error)
		require.False(t, resp.Allowed)
	})

	t.Run("owner read branch without grant", func(t *testing.T) {
		commonUser := addCommonUser("common9")
		repo := addRepo("aaa", commonUser.ID, false)
		resp, err := rbacChecker.AuthorizeMember(ctx, repo.ID, &rbac.AuthorizationRequest{
			OperatorID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacmodel.ReadBranchAction,
					Resource: rbacmodel.RepoURArn(commonUser.ID.String(), repo.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("test for public repo", func(t *testing.T) {
		commonUser := addCommonUser("common10")
		otherUser := addCommonUser("common11")
		repo := addRepo("aaa", commonUser.ID, true)
		t.Run("can read branch without grant", func(t *testing.T) {
			resp, err := rbacChecker.AuthorizeMember(ctx, repo.ID, &rbac.AuthorizationRequest{
				OperatorID: otherUser.ID,
				RequiredPermissions: rbac.Node{
					Permission: rbac.Permission{
						Action:   rbacmodel.ReadBranchAction,
						Resource: rbacmodel.RepoURArn(commonUser.ID.String(), repo.ID.String()),
					},
				},
			})
			require.NoError(t, err)
			require.NoError(t, resp.Error)
			require.True(t, resp.Allowed)
		})

		t.Run("cannot write branch without grant", func(t *testing.T) {
			resp, err := rbacChecker.AuthorizeMember(ctx, repo.ID, &rbac.AuthorizationRequest{
				OperatorID: otherUser.ID,
				RequiredPermissions: rbac.Node{
					Permission: rbac.Permission{
						Action:   rbacmodel.CreateCommitAction,
						Resource: rbacmodel.RepoURArn(commonUser.ID.String(), repo.ID.String()),
					},
				},
			})
			require.NoError(t, err)
			require.Equal(t, rbac.ErrInsufficientPermissions, resp.Error)
			require.False(t, resp.Allowed)
		})
	})
}
