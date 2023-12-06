package controller

import (
	"encoding/json"
	"net/http"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"go.uber.org/fx"
)

type UserController struct {
	fx.In

	Repo   models.IUserRepo
	Config *config.Config
}

func (A UserController) Login(w *api.JiaozifsResponse, r *http.Request) {
	ctx := r.Context()
	// Decode requestBody
	var login auth.Login
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&login); err != nil {
		w.RespError(err)
		return
	}

	// Perform login
	resp, err := login.Login(ctx, A.Repo, A.Config)
	if err != nil {
		w.RespError(err)
		return
	}

	// resp
	w.RespJSON(resp)
}

func (A UserController) Register(w *api.JiaozifsResponse, r *http.Request) {
	ctx := r.Context()
	// Decode requestBody
	var register auth.Register
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&register); err != nil {
		w.RespError(err)
	}
	// Perform register
	err := register.Register(ctx, A.Repo)
	if err != nil {
		w.RespError(err)
		return
	}
	// resp
	w.RespJSON("registration success")
}

func (A UserController) GetUserInfo(w *api.JiaozifsResponse, r *http.Request) {
	ctx := r.Context()
	// Get token from Header
	tokenString := r.Header.Get("Authorization")
	userInfo := &auth.UserInfo{Token: tokenString}

	// Perform GetUserInfo
	info, err := userInfo.UserProfile(ctx, A.Repo, A.Config)
	if err != nil {
		w.RespError(err)
		return
	}
	// resp
	w.RespJSON(info)
}
