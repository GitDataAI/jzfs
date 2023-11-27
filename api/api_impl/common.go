package api_impl

import (
	"github.com/jiaozifs/jiaozifs/api"
	"go.uber.org/fx"
	"net/http"
)

var _ api.ServerInterface = (*APIController)(nil)

type APIController struct {
	fx.In
}

func (A APIController) GetVersion(w http.ResponseWriter, r *http.Request) {

}
