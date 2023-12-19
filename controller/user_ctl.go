package controller

import (
	"context"
	"net/http"

	openapitypes "github.com/oapi-codegen/runtime/types"

	logging "github.com/ipfs/go-log/v2"

	"github.com/gorilla/sessions"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"go.uber.org/fx"
)

var userCtlLog = logging.Logger("user_ctl")

const (
	AuthHeader = "Authorization"
)

type UserController struct {
	fx.In

	SessionStore sessions.Store
	Repo         models.IRepo
	Config       *config.AuthConfig
}

func (userCtl UserController) Login(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, body api.LoginJSONRequestBody) {
	login := auth.Login{
		Username: body.Username,
		Password: body.Password,
	}

	// perform login
	authToken, err := login.Login(ctx, userCtl.Repo.UserRepo(), userCtl.Config)
	if err != nil {
		w.Error(err)
		return
	}

	internalAuthSession, _ := userCtl.SessionStore.Get(r, auth.InternalAuthSessionName)
	internalAuthSession.Values[auth.TokenSessionKeyName] = authToken.Token
	err = userCtl.SessionStore.Save(r, w, internalAuthSession)
	if err != nil {
		userCtlLog.Errorf("Failed to save internal auth session %v", err)
		w.Code(http.StatusInternalServerError)
		return
	}
	w.JSON(authToken)
}

func (userCtl UserController) Register(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.RegisterJSONRequestBody) {
	register := auth.Register{
		Username: body.Username,
		Email:    string(body.Email),
		Password: body.Password,
	}

	// perform register
	err := register.Register(ctx, userCtl.Repo.UserRepo())
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
		w.Code(http.StatusForbidden)
		return
	}

	// perform GetUserInfo
	userInfo := api.UserInfo{
		CreatedAt:       &user.CreatedAt,
		CurrentSignInAt: &user.CurrentSignInAt,
		CurrentSignInIP: &user.CurrentSignInIP,
		Email:           openapitypes.Email(user.Email),
		LastSignInAt:    &user.LastSignInAt,
		LastSignInIP:    &user.LastSignInIP,
		UpdateAt:        &user.UpdatedAt,
		Username:        user.Name,
	}
	w.JSON(userInfo)
}

func (userCtl UserController) Logout(_ context.Context, w *api.JiaozifsResponse, r *http.Request) {
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
