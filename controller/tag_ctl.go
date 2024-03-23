package controller

import (
	"context"
	"errors"
	"net/http"
	"time"

	"github.com/GitDataAI/jiaozifs/auth"
	"github.com/GitDataAI/jiaozifs/auth/rbac"
	"github.com/GitDataAI/jiaozifs/controller/validator"
	"github.com/GitDataAI/jiaozifs/models/rbacmodel"
	"github.com/GitDataAI/jiaozifs/utils"
	"github.com/GitDataAI/jiaozifs/versionmgr"

	"github.com/GitDataAI/jiaozifs/api"
	"github.com/GitDataAI/jiaozifs/block/params"
	"github.com/GitDataAI/jiaozifs/models"
	"go.uber.org/fx"
)

type TagController struct {
	fx.In
	BaseController

	Repo                models.IRepo
	PublicStorageConfig params.AdapterConfig
}

func (tagCtl TagController) CreateTag(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateTagJSONRequestBody, ownerName string, repositoryName string) {
	if err := validator.ValidateTagName(body.Name); err != nil {
		w.BadRequest(err.Error())
		return
	}

	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := tagCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	// Get repo
	repository, err := tagCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if !tagCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.CreateTagAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	workRepo, err := versionmgr.NewWorkRepositoryFromConfig(ctx, operator, repository, tagCtl.Repo, tagCtl.PublicStorageConfig)
	if err != nil {
		w.Error(err)
		return
	}

	//check target is branch
	_, err = tagCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(body.Target))
	if err == nil {
		// branch
		err = workRepo.CheckOut(ctx, versionmgr.InBranch, body.Target)
	} else if errors.Is(err, models.ErrNotFound) {
		// commit
		err = workRepo.CheckOut(ctx, versionmgr.InCommit, body.Target)
	}
	if err != nil {
		w.Error(err)
		return
	}

	newTag, err := workRepo.CreateTag(ctx, body.Name, body.Message)
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(utils.Silent(tagToDto(newTag)), http.StatusCreated)
}

func (tagCtl TagController) DeleteTag(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.DeleteTagParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := tagCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	// Get repo
	repository, err := tagCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if !tagCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.DeleteTagAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	workRepo, err := versionmgr.NewWorkRepositoryFromConfig(ctx, operator, repository, tagCtl.Repo, tagCtl.PublicStorageConfig)
	if err != nil {
		w.Error(err)
		return
	}

	err = workRepo.CheckOut(ctx, versionmgr.InTag, params.RefName)
	if err != nil {
		w.Error(err)
		return
	}

	err = workRepo.DeleteTag(ctx)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (tagCtl TagController) GetTag(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.GetTagParams) {
	owner, err := tagCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	// Get repo
	repository, err := tagCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if !tagCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ReadTagAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	// Get branch
	ref, err := tagCtl.Repo.TagRepo().Get(ctx, models.NewGetTagParams().SetName(params.RefName).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(utils.Silent(tagToDto(ref)))
}

func (tagCtl TagController) ListTags(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.ListTagsParams) {
	owner, err := tagCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := tagCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if !tagCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ListTagsAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	listTagParams := models.NewListTagParams()
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listTagParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil {
		listTagParams.SetAfter(time.UnixMilli(*params.After))
	}

	pageAmount := utils.IntValue(params.Amount)
	if pageAmount > utils.DefaultMaxPerPage || pageAmount <= 0 {
		listTagParams.SetAmount(utils.DefaultMaxPerPage)
	} else {
		listTagParams.SetAmount(pageAmount)
	}

	tags, hasMore, err := tagCtl.Repo.TagRepo().List(ctx, listTagParams.SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}
	results := utils.Silent(utils.ArrMap(tags, tagToDto))
	pagMag := utils.PaginationFor(hasMore, results, "UpdatedAt")
	pagination := api.Pagination{
		HasMore:    pagMag.HasMore,
		MaxPerPage: pagMag.MaxPerPage,
		NextOffset: pagMag.NextOffset,
		Results:    pagMag.Results,
	}
	w.JSON(api.TagList{
		Pagination: pagination,
		Results:    results,
	})
}

func tagToDto(in *models.Tag) (api.Tag, error) {
	return api.Tag{
		CreatedAt:    in.CreatedAt.UnixMilli(),
		Message:      in.Message,
		Name:         in.Name,
		RepositoryId: in.RepositoryID,
		CreatorId:    in.CreatorID,
		Target:       in.Target.Hex(),
		UpdatedAt:    in.UpdatedAt.UnixMilli(),
	}, nil
}
