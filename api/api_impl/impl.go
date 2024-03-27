package apiimpl

import (
	"github.com/GitDataAI/jiaozifs/api"
	"github.com/GitDataAI/jiaozifs/controller"
	"go.uber.org/fx"
)

var _ api.ServerInterface = (*APIController)(nil)

type APIController struct {
	fx.In

	controller.CommonController
	controller.ObjectController
	controller.UserController
	controller.WipController
	controller.CommitController
	controller.RepositoryController
	controller.BranchController
	controller.MergeRequestController
	controller.AkSkController

	controller.GroupController
	controller.MemberController
	controller.TagController
}
