package controller

import (
	"context"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/utils"

	aksk2 "github.com/jiaozifs/jiaozifs/auth/aksk"

	"github.com/jiaozifs/jiaozifs/auth"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type AkSkController struct {
	fx.In

	Repo models.IRepo
}

func (akskCtl AkSkController) CreateAksk(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, params api.CreateAkskParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
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
	w.JSON(aksk, http.StatusCreated)
}

func (akskCtl AkSkController) GetAksk(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, params api.GetAkskParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
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
	w.JSON(aksk)
}

func (akskCtl AkSkController) DeleteAksk(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, params api.DeleteAkskParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	delParams := models.NewDeleteAkSkParams().SetUserID(operator.ID)
	if params.Id != nil {
		delParams.SetID(*params.Id)
	}

	if params.AccessKey != nil {
		delParams.SetAccessKey(utils.StringValue(params.AccessKey))
	}

	aksk, err := akskCtl.Repo.AkskRepo().Delete(ctx, delParams)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(aksk)
}

func (akskCtl AkSkController) ListAksks(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, params api.ListAksksParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
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
	results := make([]api.Aksk, 0, len(aksks))
	for _, repo := range aksks {
		results = append(results, *akskToDto(repo))
	}
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

func akskToDto(in *models.AkSk) *api.Aksk {
	return &api.Aksk{
		AccessKey:   in.AccessKey,
		CreatedAt:   in.CreatedAt.UnixMilli(),
		Description: in.Description,
		Id:          in.ID,
		SecretKey:   in.SecretKey,
		UpdatedAt:   in.UpdatedAt.UnixMilli(),
	}
}
