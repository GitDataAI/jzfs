package controller

import (
	"context"
	"encoding/json"
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

	Repo   models.IUserRepo
	Config *config.Config
}

func (A UserController) Login(ctx context.Context, w *api.JiaozifsResponse, r *http.Request) {
	// Decode requestBody
	var login auth.Login
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&login); err != nil {
		w.Error(err)
		return
	}

	// perform login
	authToken, err := login.Login(ctx, A.Repo, A.Config)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(authToken)
}

func (A UserController) Register(ctx context.Context, w *api.JiaozifsResponse, r *http.Request) {
	// Decode requestBody
	var register auth.Register
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&register); err != nil {
		w.Error(err)
		return
	}
	// perform register
	err := register.Register(ctx, A.Repo)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (A UserController) GetUserInfo(ctx context.Context, w *api.JiaozifsResponse, r *http.Request) {
	// Get token from Header
	tokenString := r.Header.Get(AuthHeader)
	userInfo := &auth.UserInfo{Token: tokenString}

	// perform GetUserInfo
	usrInfo, err := userInfo.UserProfile(ctx, A.Repo, A.Config)
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(usrInfo)
}
