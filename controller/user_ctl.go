package controller

import (
	"encoding/json"
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type UserController struct {
	fx.In

	UserRepo *models.IUserRepo
	Config   *config.Config
}

func (A UserController) Login(w *api.JiaozifsResponse, r *http.Request) {
	// Decode requestBody
	var login auth.Login
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&login); err != nil {
		w.RespError(err)
		return
	}

	// Perform login
	resp, err := login.Login(*A.UserRepo, A.Config)
	if err != nil {
		w.RespError(err)
		return
	}

	// resp
	w.RespJSON(resp)
}

func (A UserController) Register(w *api.JiaozifsResponse, r *http.Request) {
	// Decode requestBody
	var register auth.Register
	decoder := json.NewDecoder(r.Body)
	if err := decoder.Decode(&register); err != nil {
		w.RespError(err)
	}
	// Perform register
	msg, err := register.Register(*A.UserRepo)
	if err != nil {
		w.RespError(err)
		return
	}
	// resp
	w.RespJSON(msg)
}

func (A UserController) GetUserInfo(w *api.JiaozifsResponse, r *http.Request) {
	// Get token from Header
	tokenString := r.Header.Get("Authorization")
	userInfo := &auth.UserInfo{Token: tokenString}

	// Perform GetUserInfo
	info, err := userInfo.UserProfile(*A.UserRepo, A.Config)
	if err != nil {
		w.RespError(err)
		return
	}
	// resp
	w.RespJSON(info)
}
