package controller

import (
	"context"
	"encoding/hex"
	"fmt"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/auth/rbac"
	"github.com/jiaozifs/jiaozifs/controller/validator"
	"github.com/jiaozifs/jiaozifs/models/rbacmodel"
	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/go-openapi/swag"
	"github.com/gorilla/sessions"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	openapitypes "github.com/oapi-codegen/runtime/types"
	"go.uber.org/fx"
)

var userCtlLog = logging.Logger("user_ctl")

type UserController struct {
	fx.In
	BaseController

	SessionStore sessions.Store
	Repo         models.IRepo
	Config       *config.AuthConfig

	BasicAuthenticator *auth.BasicAuthenticator
}

func (userCtl UserController) Login(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, body api.LoginJSONRequestBody) {
	user, err := userCtl.BasicAuthenticator.AuthenticateUser(ctx, body.Name, body.Password)
	if err != nil {
		w.Code(http.StatusUnauthorized)
		return
	}
	userCtl.generateAndRespToken(w, r, user.Name)
}

func (userCtl UserController) RefreshToken(ctx context.Context, w *api.JiaozifsResponse, r *http.Request) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	userCtl.generateAndRespToken(w, r, operator.Name)
}

func (userCtl UserController) generateAndRespToken(w *api.JiaozifsResponse, r *http.Request, name string) {
	// Generate user token
	loginTime := time.Now()
	expires := loginTime.Add(auth.ExpirationDuration)
	secretKey, err := hex.DecodeString(userCtl.Config.SecretKey)
	if err != nil {
		w.Error(err)
		return
	}

	tokenString, err := auth.GenerateJWTLogin(secretKey, name, loginTime, expires)
	if err != nil {
		w.Error(err)
		return
	}

	userCtlLog.Infof("user %s login successful", name)

	internalAuthSession, _ := userCtl.SessionStore.Get(r, auth.InternalAuthSessionName)
	internalAuthSession.Values[auth.TokenSessionKeyName] = tokenString
	err = userCtl.SessionStore.Save(r, w, internalAuthSession)
	if err != nil {
		userCtlLog.Errorf("Failed to save internal auth session %v", err)
		w.Code(http.StatusInternalServerError)
		return
	}
	w.JSON(api.AuthenticationToken{
		Token:           tokenString,
		TokenExpiration: swag.Int64(expires.Unix()),
	})
}

func (userCtl UserController) Register(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.RegisterJSONRequestBody) {
	err := validator.ValidateUsername(body.Name)
	if err != nil {
		w.BadRequest(err.Error())
		return
	}
	// check username, email
	count1, err := userCtl.Repo.UserRepo().Count(ctx, models.NewCountUserParam().SetName(body.Name))
	if err != nil {
		w.Error(err)
		return
	}
	count2, err := userCtl.Repo.UserRepo().Count(ctx, models.NewCountUserParam().SetEmail(string(body.Email)))
	if err != nil {
		w.Error(err)
		return
	}

	if count1+count2 > 0 {
		w.BadRequest(fmt.Sprintf("username %s or email %s not found ", body.Name, body.Email))
	}

	// reserve temporarily
	password, err := auth.HashPassword(body.Password)
	if err != nil {
		w.Error(err)
		return
	}

	// insert db
	user := &models.User{
		Name:              body.Name,
		Email:             string(body.Email),
		EncryptedPassword: string(password),
		CurrentSignInAt:   time.Time{},
		LastSignInAt:      time.Time{},
		CurrentSignInIP:   "",
		LastSignInIP:      "",
		CreatedAt:         time.Now(),
		UpdatedAt:         time.Now(),
	}

	var insertUser *models.User
	err = userCtl.Repo.Transaction(ctx, func(repo models.IRepo) error {
		insertUser, err = repo.UserRepo().Insert(ctx, user)
		if err != nil {
			return fmt.Errorf("inser user %s user error %w", body.Name, err)
		}

		userOwnGroup, err := repo.GroupRepo().Get(ctx, rbacmodel.NewGetGroupParams().SetName(rbac.UserOwnAccess))
		if err != nil {
			return err
		}
		//bind own user group
		_, err = repo.UserGroupRepo().Insert(ctx, &rbacmodel.UserGroup{
			UserID:    insertUser.ID,
			GroupID:   userOwnGroup.ID,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		})
		return err
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(userInfoToDto(insertUser), http.StatusCreated)
}

func (userCtl UserController) GetUserInfo(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request) {
	// Get token from Header
	user, err := auth.GetOperator(ctx)
	if err != nil {
		w.Code(http.StatusUnauthorized)
		return
	}

	if !userCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ListRepositoriesAction,
			Resource: rbacmodel.RepoUArn(user.ID.String()),
		},
	}) {
		return
	}

	w.JSON(userInfoToDto(user))
}

func (userCtl UserController) Logout(_ context.Context, w *api.JiaozifsResponse, r *http.Request) {
	//todo only web credencial could logout
	session, err := userCtl.SessionStore.Get(r, auth.InternalAuthSessionName)
	if err != nil {
		w.Error(err)
		return
	}

	session.Options.MaxAge = -1
	if session.Save(r, w) != nil {
		userCtlLog.Errorf("Failed to save internal auth session %v", err)
		w.Error(err)
		return
	}
	http.Redirect(w, r, "/", http.StatusTemporaryRedirect)
}

func userInfoToDto(user *models.User) *api.UserInfo {
	return &api.UserInfo{
		Id:              user.ID,
		Name:            user.Name,
		Email:           openapitypes.Email(user.Email),
		CurrentSignInAt: utils.Int64(user.CurrentSignInAt.UnixMilli()),
		CurrentSignInIp: &user.CurrentSignInIP,
		LastSignInAt:    utils.Int64(user.LastSignInAt.UnixMilli()),
		LastSignInIp:    &user.LastSignInIP,
		UpdatedAt:       user.UpdatedAt.UnixMilli(),
		CreatedAt:       user.CreatedAt.UnixMilli(),
	}
}
