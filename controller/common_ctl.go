package controller

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/version"
	"go.uber.org/fx"
)

type CommonController struct {
	fx.In
}

func (c CommonController) GetVersion(_ context.Context, w *api.JiaozifsResponse, _ *http.Request) {
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

func (c CommonController) GetSetupState(ctx context.Context, w *api.JiaozifsResponse, r *http.Request) {
	//TODO implement me
	panic("implement me")
}
