package controller

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/version"
	"go.uber.org/fx"
)

type VersionController struct {
	fx.In
}

func (A VersionController) GetVersion(_ context.Context, w *api.JiaozifsResponse, _ *http.Request) {
	swagger, err := api.GetSwagger()
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(api.VersionResult{
		ApiVersion: swagger.Info.Version,
		Version:    version.UserVersion(),
	})
}
