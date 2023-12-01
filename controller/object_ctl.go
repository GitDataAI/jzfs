package controller

import (
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	"go.uber.org/fx"
)

type ObjectController struct {
	fx.In
}

func (A ObjectController) DeleteObject(_ *api.JiaozifsResponse, r *http.Request, repository string, params api.DeleteObjectParams) { //nolint
	//TODO implement me
	panic("implement me")
}

func (A ObjectController) GetObject(_ *api.JiaozifsResponse, r *http.Request, repository string, params api.GetObjectParams) { //nolint
	//TODO implement me
	panic("implement me")
}

func (A ObjectController) HeadObject(_ *api.JiaozifsResponse, r *http.Request, repository string, params api.HeadObjectParams) { //nolint
	//TODO implement me
	panic("implement me")
}

func (A ObjectController) UploadObject(_ *api.JiaozifsResponse, r *http.Request, repository string, params api.UploadObjectParams) { //nolint
	//TODO implement me
	panic("implement me")
}
