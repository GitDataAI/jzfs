package controller

import (
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/version"
	"go.uber.org/fx"
)

type VersionController struct {
	fx.In
}

func (A VersionController) GetVersion(w *api.JiaozifsResponse, _ *http.Request) {
	swagger, err := api.GetSwagger()
	if err != nil {
		w.RespError(err)
		return
	}

	w.RespJSON(api.VersionResult{
		ApiVersion: swagger.Info.Version,
		Version:    version.UserVersion(),
	})
}
