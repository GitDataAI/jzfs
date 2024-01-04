package controller

import (
	"context"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/block/params"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type MergeRequestController struct {
	fx.In

	Repo                models.IRepo
	PublicStorageConfig params.AdapterConfig
}

func (mrCtl MergeRequestController) ListMergeRequests(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, ownerName string, repositoryName string, params api.ListMergeRequestsParams) {
	//TODO implement me
	panic("implement me")
}

func (mrCtl MergeRequestController) CreateMergeRequest(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateMergeRequestJSONRequestBody, ownerName string, repositoryName string) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := mrCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name {
		w.Forbidden()
		return
	}

	// Get repo
	repository, err := mrCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	sorceBranch, err := mrCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(body.SourceBranchName))
	if err != nil {
		w.Error(err)
		return
	}

	targetBranch, err := mrCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(body.TargetBranchName))
	if err != nil {
		w.Error(err)
		return
	}

	mrModel, err := mrCtl.Repo.MergeRequestRepo().Insert(ctx, &models.MergeRequest{
		TargetBranch: targetBranch.ID,
		SourceBranch: sorceBranch.ID,
		SourceRepoID: repository.ID,
		TargetRepoID: repository.ID,
		Title:        body.Title,
		MergeStatus:  models.InitMergeStatus,
		Description:  body.Description,
		AuthorID:     operator.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	})

	if err != nil {
		w.Error(err)
		return
	}
	//get merge state
	w.JSON(mrModel, http.StatusCreated)
}

func (mrCtl MergeRequestController) DeleteMergeRequest(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, ownerName string, repositoryName string, mrID uint64) {
	//TODO implement me
	panic("implement me")
}

func (mrCtl MergeRequestController) GetMergeRequest(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, ownerName string, repositoryName string, mrID uint64) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := mrCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name {
		w.Forbidden()
		return
	}

	// Get repo
	repository, err := mrCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}
}

func (mrCtl MergeRequestController) UpdateMergeRequest(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, body api.UpdateMergeRequestJSONRequestBody, ownerName string, repositoryName string, mrID uint64) {
	//TODO implement me
	panic("implement me")
}
