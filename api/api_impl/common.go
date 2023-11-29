package apiimpl

import (
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/version"
	"go.uber.org/fx"
)

var _ api.ServerInterface = (*APIController)(nil)

type APIController struct {
	fx.In
}

func (A APIController) GetVersion(w *api.JiaozifsResponse, _ *http.Request) {
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
