package controller

import (
	"context"
	"net/http"
	"time"

	"github.com/GitDataAI/jiaozifs/models/rbacmodel"

	"github.com/GitDataAI/jiaozifs/auth/rbac"

	"github.com/GitDataAI/jiaozifs/utils"

	aksk2 "github.com/GitDataAI/jiaozifs/auth/aksk"

	"github.com/GitDataAI/jiaozifs/auth"

	"github.com/GitDataAI/jiaozifs/api"
	"github.com/GitDataAI/jiaozifs/models"
	"go.uber.org/fx"
)

type AkSkController struct {
	fx.In
	BaseController

	Repo models.IRepo
}

func (akskCtl AkSkController) CreateAksk(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, params api.CreateAkskParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	if !akskCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.CreateCredentialsAction,
			Resource: rbacmodel.UserAkskArn(operator.ID.String()),
		},
	}) {
		return
	}

	ak, sk, err := aksk2.GenerateAksk()
	if err != nil {
		w.Error(err)
		return
	}

	aksk := &models.AkSk{
		UserID:      operator.ID,
		AccessKey:   ak,
		SecretKey:   sk,
		Description: params.Description,
		CreatedAt:   time.Now(),
		UpdatedAt:   time.Now(),
	}
	aksk, err = akskCtl.Repo.AkskRepo().Insert(ctx, aksk)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(utils.Silent(akskToDto(aksk)), http.StatusCreated)
}

func (akskCtl AkSkController) GetAksk(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, params api.GetAkskParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	if !akskCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ReadCredentialsAction,
			Resource: rbacmodel.UserAkskArn(operator.ID.String()),
		},
	}) {
		return
	}

	getParams := models.NewGetAkSkParams().SetUserID(operator.ID)
	if params.Id != nil {
		getParams.SetID(*params.Id)
	}

	if params.AccessKey != nil {
		getParams.SetAccessKey(utils.StringValue(params.AccessKey))
	}

	aksk, err := akskCtl.Repo.AkskRepo().Get(ctx, getParams)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(utils.Silent(akskToSafeDto(aksk)))
}

func (akskCtl AkSkController) DeleteAksk(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, params api.DeleteAkskParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	if !akskCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.DeleteCredentialsAction,
			Resource: rbacmodel.UserAkskArn(operator.ID.String()),
		},
	}) {
		return
	}

	delParams := models.NewDeleteAkSkParams().SetUserID(operator.ID)
	if params.Id != nil {
		delParams.SetID(*params.Id)
	}

	if params.AccessKey != nil {
		delParams.SetAccessKey(utils.StringValue(params.AccessKey))
	}

	_, err = akskCtl.Repo.AkskRepo().Delete(ctx, delParams)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (akskCtl AkSkController) ListAksks(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, params api.ListAksksParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	if !akskCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ListCredentialsAction,
			Resource: rbacmodel.UserAkskArn(operator.ID.String()),
		},
	}) {
		return
	}

	listParams := models.NewListAkSkParams().SetUserID(operator.ID)
	if params.After != nil {
		listParams.SetAfter(time.UnixMilli(utils.Int64Value(params.After)))
	}

	if params.Amount != nil {
		listParams.SetAmount(utils.IntValue(params.Amount))
	}

	aksks, hasMore, err := akskCtl.Repo.AkskRepo().List(ctx, listParams)
	if err != nil {
		w.Error(err)
		return
	}
	results := utils.Silent(utils.ArrMap(aksks, akskToSafeDto))
	pagMag := utils.PaginationFor(hasMore, results, "UpdatedAt")
	pagination := api.Pagination{
		HasMore:    pagMag.HasMore,
		MaxPerPage: pagMag.MaxPerPage,
		NextOffset: pagMag.NextOffset,
		Results:    pagMag.Results,
	}
	w.JSON(api.AkskList{
		Pagination: pagination,
		Results:    results,
	})
}

func akskToDto(in *models.AkSk) (api.Aksk, error) {
	return api.Aksk{
		AccessKey:   in.AccessKey,
		CreatedAt:   in.CreatedAt.UnixMilli(),
		Description: in.Description,
		Id:          in.ID,
		SecretKey:   in.SecretKey,
		UpdatedAt:   in.UpdatedAt.UnixMilli(),
	}, nil
}

func akskToSafeDto(in *models.AkSk) (api.SafeAksk, error) {
	return api.SafeAksk{
		AccessKey:   in.AccessKey,
		CreatedAt:   in.CreatedAt.UnixMilli(),
		Description: in.Description,
		Id:          in.ID,
		UpdatedAt:   in.UpdatedAt.UnixMilli(),
	}, nil
}
