package controller

import (
	"context"
	"errors"
	"net/http"

	"github.com/jiaozifs/jiaozifs/auth/rbac"
	"github.com/jiaozifs/jiaozifs/block/params"
	"github.com/jiaozifs/jiaozifs/controller/validator"
	"github.com/jiaozifs/jiaozifs/models/rbacmodel"
	"github.com/jiaozifs/jiaozifs/versionmgr"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"go.uber.org/fx"
)

type BranchController struct {
	fx.In
	BaseController

	Repo                models.IRepo
	PublicStorageConfig params.AdapterConfig
}

func (bct BranchController) ListBranches(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.ListBranchesParams) {
	owner, err := bct.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if !bct.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ListBranchesAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	listBranchParams := models.NewListBranchParams()
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listBranchParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil && len(*params.After) > 0 {
		listBranchParams.SetAfter(*params.After)
	}
	pageAmount := utils.IntValue(params.Amount)
	if pageAmount > utils.DefaultMaxPerPage || pageAmount <= 0 {
		listBranchParams.SetAmount(utils.DefaultMaxPerPage)
	} else {
		listBranchParams.SetAmount(pageAmount)
	}

	branches, hasMore, err := bct.Repo.BranchRepo().List(ctx, listBranchParams.SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}
	results := make([]api.Branch, 0, len(branches))
	for _, branch := range branches {
		r := api.Branch{
			CommitHash:   branch.CommitHash.Hex(),
			CreatedAt:    branch.CreatedAt.UnixMilli(),
			CreatorId:    branch.CreatorID,
			Description:  branch.Description,
			Id:           branch.ID,
			Name:         branch.Name,
			RepositoryId: branch.RepositoryID,
			UpdatedAt:    branch.UpdatedAt.UnixMilli(),
		}
		results = append(results, r)
	}
	pagMag := utils.PaginationFor(hasMore, results, "Name")
	pagination := api.Pagination{
		HasMore:    pagMag.HasMore,
		MaxPerPage: pagMag.MaxPerPage,
		NextOffset: pagMag.NextOffset,
		Results:    pagMag.Results,
	}
	w.JSON(api.BranchList{
		Pagination: pagination,
		Results:    results,
	})
}

func (bct BranchController) CreateBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateBranchJSONRequestBody, ownerName string, repositoryName string) {
	if err := validator.ValidateBranchName(body.Name); err != nil {
		w.BadRequest(err.Error())
		return
	}

	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := bct.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if !bct.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.CreateBranchAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	//get source branch
	sourceBranch, err := bct.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetName(body.Source).SetRepositoryID(repository.ID))
	if err != nil && !errors.Is(err, models.ErrNotFound) {
		w.Error(err)
		return
	}

	workRepo, err := versionmgr.NewWorkRepositoryFromConfig(ctx, operator, repository, bct.Repo, bct.PublicStorageConfig)
	if err != nil {
		w.Error(err)
		return
	}

	err = workRepo.CheckOut(ctx, versionmgr.InCommit, sourceBranch.CommitHash.Hex())
	if err != nil {
		w.Error(err)
		return
	}

	newBranch, err := workRepo.CreateBranch(ctx, body.Name)
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(api.Branch{
		CommitHash:   newBranch.CommitHash.Hex(),
		CreatedAt:    newBranch.CreatedAt.UnixMilli(),
		CreatorId:    newBranch.CreatorID,
		Description:  newBranch.Description,
		Id:           newBranch.ID,
		Name:         newBranch.Name,
		RepositoryId: newBranch.RepositoryID,
		UpdatedAt:    newBranch.UpdatedAt.UnixMilli(),
	}, http.StatusCreated)
}

func (bct BranchController) DeleteBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.DeleteBranchParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := bct.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if !bct.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.DeleteBranchAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	if params.RefName == repository.HEAD {
		w.BadRequest("can not delete HEAD branch")
		return
	}

	workRepo, err := versionmgr.NewWorkRepositoryFromConfig(ctx, operator, repository, bct.Repo, bct.PublicStorageConfig)
	if err != nil {
		w.Error(err)
		return
	}

	err = workRepo.CheckOut(ctx, versionmgr.InBranch, params.RefName)
	if err != nil {
		w.Error(err)
		return
	}

	err = workRepo.DeleteBranch(ctx)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (bct BranchController) GetBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.GetBranchParams) {
	owner, err := bct.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if !bct.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ReadRepositoryAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	// Get branch
	ref, err := bct.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetName(params.RefName).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(api.Branch{
		CommitHash:   ref.CommitHash.Hex(),
		CreatedAt:    ref.CreatedAt.UnixMilli(),
		CreatorId:    ref.CreatorID,
		Description:  ref.Description,
		Id:           ref.ID,
		Name:         ref.Name,
		RepositoryId: ref.RepositoryID,
		UpdatedAt:    ref.UpdatedAt.UnixMilli(),
	})
}
