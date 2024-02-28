package controller

import (
	"context"
	"encoding/hex"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/auth/rbac"
	"github.com/jiaozifs/jiaozifs/controller/validator"
	"github.com/jiaozifs/jiaozifs/models/rbacModel"

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

	register := auth.Register{
		Username: body.Name,
		Email:    string(body.Email),
		Password: body.Password,
	}

	// perform register
	err = register.Register(ctx, userCtl.Repo.UserRepo())
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
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
			Action:   rbacModel.ListRepositoriesAction,
			Resource: rbacModel.RepoUArn(user.ID.String()),
		},
	}) {
		return
	}

	// perform GetUserInfo
	userInfo := api.UserInfo{
		Name:            user.Name,
		Email:           openapitypes.Email(user.Email),
		CurrentSignInAt: utils.Int64(user.CurrentSignInAt.UnixMilli()),
		CurrentSignInIp: &user.CurrentSignInIP,
		LastSignInAt:    utils.Int64(user.LastSignInAt.UnixMilli()),
		LastSignInIp:    &user.LastSignInIP,
		UpdatedAt:       user.UpdatedAt.UnixMilli(),
		CreatedAt:       user.CreatedAt.UnixMilli(),
	}
	w.JSON(userInfo)
}

func (userCtl UserController) Logout(ctx context.Context, w *api.JiaozifsResponse, r *http.Request) {
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
