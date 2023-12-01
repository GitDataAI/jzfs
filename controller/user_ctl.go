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
