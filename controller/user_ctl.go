package controller

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"go.uber.org/fx"
)

const (
	AuthHeader = "Authorization"
)

type UserController struct {
	fx.In

	Repo   models.IRepo
	Config *config.Config
}

func (userCtl UserController) Login(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.LoginJSONRequestBody) {
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

func (userCtl UserController) GetUserInfo(ctx context.Context, w *api.JiaozifsResponse, r *http.Request) {
	// Get token from Header
	tokenString := r.Header.Get(AuthHeader)
	userInfo := &auth.UserInfo{Token: tokenString}

	// perform GetUserInfo
	usrInfo, err := userInfo.UserProfile(ctx, userCtl.Repo.UserRepo(), userCtl.Config)
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(usrInfo)
}
