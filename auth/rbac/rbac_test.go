package rbac_test

import (
	"context"
	"testing"
	"time"

	"github.com/google/uuid"

	"github.com/jiaozifs/jiaozifs/models/rbacModel"

	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/auth"

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

		userOwnGroup, err := dbRepo.GroupRepo().Get(ctx, rbacModel.NewGetGroupParams().SetName(rbac.UserOwnAccess))
		require.NoError(t, err)
		//bind own user group
		_, err = dbRepo.UserGroupRepo().Insert(ctx, &rbacModel.UserGroup{
			UserID:    commonUser.ID,
			GroupID:   userOwnGroup.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		return commonUser
	}

	addRepo := func(name string, ownerID uuid.UUID) *models.Repository {
		repo, err := dbRepo.RepositoryRepo().Insert(ctx, &models.Repository{
			Name:    name,
			OwnerID: ownerID,
			HEAD:    "master",
		})
		require.NoError(t, err)
		return repo
	}

	t.Run("super user", func(t *testing.T) {
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			UserID: superUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.ReadUserAction,
					Resource: rbacModel.UserArn(superUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("super user controller other user", func(t *testing.T) {
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
			UserID: superUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.ReadUserAction,
					Resource: rbacModel.UserArn(commonUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("common user controller himself", func(t *testing.T) {
		commonUser := addCommonUser("common1")
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			UserID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.ReadUserAction,
					Resource: rbacModel.UserArn(commonUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("common user cannot controller himself", func(t *testing.T) {
		commonUser := addCommonUser("common2")
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			UserID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.ReadUserAction,
					Resource: rbacModel.UserArn(superUser.ID.String()),
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
			UserID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.CreateRepositoryAction,
					Resource: rbacModel.RepoUArn(commonUser.ID.String()),
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
			UserID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.CreateBranchAction,
					Resource: rbacModel.RepoURArn(commonUser.ID.String(), repoID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("super create others repo", func(t *testing.T) {
		commonUser := addCommonUser("common5")
		resp, err := rbacChecker.Authorize(ctx, &rbac.AuthorizationRequest{
			UserID: superUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.CreateRepositoryAction,
					Resource: rbacModel.RepoUArn(commonUser.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("cannt create branch in other user", func(t *testing.T) {
		commonUser := addCommonUser("common6")
		other1User := addCommonUser("other1")
		repo := addRepo("aaa", other1User.ID)
		_, err := rbacChecker.AuthorizeMember(ctx, repo.ID, &rbac.AuthorizationRequest{
			UserID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.CreateBranchAction,
					Resource: rbacModel.RepoURArn(other1User.ID.String(), repo.ID.String()),
				},
			},
		})
		require.Error(t, err)
	})

	t.Run("can create branch in other user after add in member", func(t *testing.T) {
		commonUser := addCommonUser("common7")
		other1User := addCommonUser("other2")
		repo := addRepo("aaa", other1User.ID)
		repoWriteGroup, err := dbRepo.GroupRepo().Get(ctx, rbacModel.NewGetGroupParams().SetName(rbac.RepoWrite))
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
			UserID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.CreateBranchAction,
					Resource: rbacModel.RepoURArn(other1User.ID.String(), repo.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.NoError(t, resp.Error)
		require.True(t, resp.Allowed)
	})

	t.Run("can not create branch in other user after add in member", func(t *testing.T) {
		commonUser := addCommonUser("common8")
		other1User := addCommonUser("other3")
		repo := addRepo("aaa", other1User.ID)
		repoWriteGroup, err := dbRepo.GroupRepo().Get(ctx, rbacModel.NewGetGroupParams().SetName(rbac.RepoRead))
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
			UserID: commonUser.ID,
			RequiredPermissions: rbac.Node{
				Permission: rbac.Permission{
					Action:   rbacModel.CreateBranchAction,
					Resource: rbacModel.RepoURArn(other1User.ID.String(), repo.ID.String()),
				},
			},
		})
		require.NoError(t, err)
		require.Equal(t, rbac.ErrInsufficientPermissions, resp.Error)
		require.False(t, resp.Allowed)
	})
}
