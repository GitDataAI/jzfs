package apiimpl

import (
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/controller"
	"go.uber.org/fx"
)

var _ api.ServerInterface = (*APIController)(nil)

type APIController struct {
	fx.In

	controller.VersionController
	controller.ObjectController
}
