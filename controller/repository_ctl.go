package controller

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/GitDataAI/jiaozifs/auth/rbac"
	"github.com/GitDataAI/jiaozifs/controller/validator"
	"github.com/GitDataAI/jiaozifs/models/rbacmodel"

	"github.com/google/uuid"
	logging "github.com/ipfs/go-log/v2"
	"github.com/GitDataAI/jiaozifs/api"
	"github.com/GitDataAI/jiaozifs/auth"
	"github.com/GitDataAI/jiaozifs/block/factory"
	"github.com/GitDataAI/jiaozifs/block/params"
	"github.com/GitDataAI/jiaozifs/config"
	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/utils"
	"github.com/GitDataAI/jiaozifs/utils/hash"
	"github.com/GitDataAI/jiaozifs/versionmgr"
	"go.uber.org/fx"
)

const DefaultBranchName = "main"

var repoLog = logging.Logger("repo control")

type RepositoryController struct {
	fx.In
	BaseController

	Repo                models.IRepo
	PublicStorageConfig params.AdapterConfig
}

func (repositoryCtl RepositoryController) ListRepositoryOfAuthenticatedUser(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, params api.ListRepositoryOfAuthenticatedUserParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	if !repositoryCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ListRepositoriesAction,
			Resource: rbacmodel.RepoUArn(operator.ID.String()),
		},
	}) {
		return
	}

	listRepoParams := models.NewListRepoParams()
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listRepoParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil {
		listRepoParams.SetAfter(time.UnixMilli(utils.Int64Value(params.After)))
	}
	pageAmount := utils.IntValue(params.Amount)
	if pageAmount > utils.DefaultMaxPerPage || pageAmount <= 0 {
		listRepoParams.SetAmount(utils.DefaultMaxPerPage)
	} else {
		listRepoParams.SetAmount(pageAmount)
	}

	repositories, hasMore, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, listRepoParams.
		SetOwnerID(operator.ID))
	if err != nil {
		w.Error(err)
		return
	}
	results := make([]api.Repository, 0, len(repositories))
	for _, repo := range repositories {
		results = append(results, *repositoryToDto(repo))
	}
	pagMag := utils.PaginationFor(hasMore, results, "UpdatedAt")
	pagination := api.Pagination{
		HasMore:    pagMag.HasMore,
		MaxPerPage: pagMag.MaxPerPage,
		NextOffset: pagMag.NextOffset,
		Results:    pagMag.Results,
	}
	w.JSON(api.RepositoryList{
		Pagination: pagination,
		Results:    results,
	})
}

func (repositoryCtl RepositoryController) ListRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, params api.ListRepositoryParams) {
	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	//TODO should get (private repo repositories has been granted)  and (public repositories)
	if !repositoryCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ListRepositoriesAction,
			Resource: rbacmodel.RepoUArn(owner.ID.String()),
		},
	}) {
		return
	}

	listRepoParams := models.NewListRepoParams().SetOwnerID(owner.ID)
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listRepoParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil {
		listRepoParams.SetAfter(time.UnixMilli(*params.After))
	}
	pageAmount := utils.IntValue(params.Amount)
	if pageAmount > utils.DefaultMaxPerPage || pageAmount <= 0 {
		listRepoParams.SetAmount(utils.DefaultMaxPerPage)
	} else {
		listRepoParams.SetAmount(pageAmount)
	}

	repositories, hasMore, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, listRepoParams)
	if err != nil {
		w.Error(err)
		return
	}
	results := make([]api.Repository, 0, len(repositories))
	for _, repo := range repositories {
		results = append(results, *repositoryToDto(repo))
	}
	pagMag := utils.PaginationFor(hasMore, results, "UpdatedAt")
	pagination := api.Pagination{
		HasMore:    pagMag.HasMore,
		MaxPerPage: pagMag.MaxPerPage,
		NextOffset: pagMag.NextOffset,
		Results:    pagMag.Results,
	}
	w.JSON(api.RepositoryList{
		Pagination: pagination,
		Results:    results,
	})
}

func (repositoryCtl RepositoryController) ListPublicRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, params api.ListPublicRepositoryParams) {
	listRepoParams := models.NewListRepoParams().SetVisible(true)
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listRepoParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil {
		listRepoParams.SetAfter(time.UnixMilli(*params.After))
	}
	pageAmount := utils.IntValue(params.Amount)
	if pageAmount > utils.DefaultMaxPerPage || pageAmount <= 0 {
		listRepoParams.SetAmount(utils.DefaultMaxPerPage)
	} else {
		listRepoParams.SetAmount(pageAmount)
	}

	repositories, hasMore, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, listRepoParams)
	if err != nil {
		w.Error(err)
		return
	}
	results := make([]api.Repository, 0, len(repositories))
	for _, repo := range repositories {
		results = append(results, *repositoryToDto(repo))
	}
	pagMag := utils.PaginationFor(hasMore, results, "UpdatedAt")
	pagination := api.Pagination{
		HasMore:    pagMag.HasMore,
		MaxPerPage: pagMag.MaxPerPage,
		NextOffset: pagMag.NextOffset,
		Results:    pagMag.Results,
	}
	w.JSON(api.RepositoryList{
		Pagination: pagination,
		Results:    results,
	})
}

func (repositoryCtl RepositoryController) CreateRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateRepositoryJSONRequestBody) {
	err := validator.ValidateRepoName(body.Name)
	if err != nil {
		w.BadRequest(err.Error())
		return
	}

	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	if !repositoryCtl.authorize(ctx, w, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.CreateRepositoryAction,
			Resource: rbacmodel.RepoUArn(operator.ID.String()),
		},
	}) {
		return
	}

	var usePublicStorage = true
	storageConfig := utils.StringValue(body.BlockstoreConfig)
	repoID := uuid.New()
	var storageNamespace *string
	if len(storageConfig) > 0 {
		usePublicStorage = false
		var cfg = config.BlockStoreConfig{}
		err = json.Unmarshal([]byte(storageConfig), &cfg)
		if err != nil {
			w.BadRequest("storage config not json format")
			return
		}

		if cfg.BlockstoreType() == "local" {
			repoLog.Infof("custom storage cnofig can not be local")
			w.Forbidden()
			return
		}
		storageNamespace = utils.String(fmt.Sprintf("%s://%s", cfg.BlockstoreType(), repoID.String()))
	} else {
		storageNamespace = utils.String(fmt.Sprintf("%s://%s", repositoryCtl.PublicStorageConfig.BlockstoreType(), repoID.String()))
	}

	defaultRef := &models.Branch{
		RepositoryID: repoID,
		CommitHash:   hash.Hash{},
		Name:         DefaultBranchName,
		CreatorID:    operator.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	repository := &models.Repository{
		ID:                   repoID,
		Name:                 body.Name,
		Visible:              utils.BoolValue(body.Visible),
		UsePublicStorage:     usePublicStorage,
		StorageAdapterParams: &storageConfig,
		StorageNamespace:     storageNamespace,
		Description:          body.Description,
		HEAD:                 DefaultBranchName,
		OwnerID:              operator.ID, // this api only create repo for operator
		CreatorID:            operator.ID,
		CreatedAt:            time.Now(),
		UpdatedAt:            time.Now(),
	}

	//create default ref
	var createdRepo *models.Repository
	err = repositoryCtl.Repo.Transaction(ctx, func(repo models.IRepo) error {
		_, err := repositoryCtl.Repo.BranchRepo().Insert(ctx, defaultRef)
		if err != nil {
			return err
		}
		createdRepo, err = repositoryCtl.Repo.RepositoryRepo().Insert(ctx, repository)
		return err
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(repositoryToDto(createdRepo), http.StatusCreated)
}

func (repositoryCtl RepositoryController) DeleteRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.DeleteRepositoryParams) {
	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if !repositoryCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.DeleteRepositoryAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	err = repositoryCtl.Repo.Transaction(ctx, func(repo models.IRepo) error {
		// delete repository
		affectRows, err := repositoryCtl.Repo.RepositoryRepo().Delete(ctx, models.NewDeleteRepoParams().SetID(repository.ID))
		if err != nil {
			return err
		}

		if affectRows == 0 {
			return fmt.Errorf("repo not found %w", models.ErrNotFound)
		}

		//delete branch
		_, err = repositoryCtl.Repo.BranchRepo().Delete(ctx, models.NewDeleteBranchParams().SetRepositoryID(repository.ID))
		if err != nil {
			return err
		}

		//delete commit
		_, err = repositoryCtl.Repo.CommitRepo(repository.ID).Delete(ctx, models.NewDeleteParams())
		if err != nil {
			return err
		}

		//delete tag
		_, err = repositoryCtl.Repo.TagRepo(repository.ID).Delete(ctx, models.NewDeleteParams())
		if err != nil {
			return err
		}

		// delete tree
		_, err = repositoryCtl.Repo.FileTreeRepo(repository.ID).Delete(ctx, models.NewDeleteTreeParams())
		if err != nil {
			return err
		}

		//delete wip
		_, err = repositoryCtl.Repo.WipRepo().Delete(ctx, models.NewDeleteWipParams().SetRepositoryID(repository.ID))
		if err != nil {
			return err
		}

		//delete all membership
		_, err = repositoryCtl.Repo.MemberRepo().DeleteMember(ctx, models.NewDeleteMemberParams().SetRepoID(repository.ID))
		return err
	})
	if err != nil {
		w.Error(err)
		return
	}

	//clean repo data
	if repository.UsePublicStorage { //todo for use custom storage, maybe add a config in setting or params in delete repository api
		adapter, err := factory.BuildBlockAdapter(ctx, repositoryCtl.PublicStorageConfig)
		if err != nil {
			w.Error(err)
			return
		}
		err = adapter.RemoveNameSpace(ctx, *repository.StorageNamespace)
		if err != nil {
			w.Error(err)
			return
		}
	} else if utils.BoolValue(params.IsCleanData) {
		cfg := config.BlockStoreConfig{}
		err = json.Unmarshal([]byte(utils.StringValue(repository.StorageAdapterParams)), &cfg)
		if err != nil {
			w.Error(err)
			return
		}
		adapter, err := factory.BuildBlockAdapter(ctx, &cfg)
		if err != nil {
			w.Error(err)
			return
		}
		err = adapter.RemoveNameSpace(ctx, *repository.StorageNamespace)
		if err != nil {
			w.Error(err)
			return
		}
	}

	w.OK()
}

func (repositoryCtl RepositoryController) GetRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string) {
	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if !repositoryCtl.authorizeMember(ctx, w, repo.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ReadRepositoryAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repo.ID.String()),
		},
	}) {
		return
	}

	w.JSON(repositoryToDto(repo))
}

func (repositoryCtl RepositoryController) UpdateRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.UpdateRepositoryJSONRequestBody, ownerName string, repositoryName string) {
	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if !repositoryCtl.authorizeMember(ctx, w, repo.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.UpdateRepositoryAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repo.ID.String()),
		},
	}) {
		return
	}

	params := models.NewUpdateRepoParams(repo.ID)
	if body.Head != nil {
		_, err = repositoryCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repo.ID).SetName(utils.StringValue(body.Head)))
		if err != nil {
			w.Error(err)
			return
		}

		params.SetHead(utils.StringValue(body.Head))
	}

	if body.Description != nil {
		params.SetDescription(utils.StringValue(body.Description))
	}

	err = repositoryCtl.Repo.RepositoryRepo().UpdateByID(ctx, params)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (repositoryCtl RepositoryController) GetCommitsInRef(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.GetCommitsInRefParams) {
	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if !repositoryCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ReadCommitAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	refName := repository.HEAD
	if params.RefName != nil {
		refName = *params.RefName
	}
	ref, err := repositoryCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(refName))
	if err != nil {
		w.Error(err)
		return
	}

	if ref.CommitHash.IsEmpty() {
		w.JSON([]api.Commit{})
		return
	}

	commit, err := repositoryCtl.Repo.CommitRepo(repository.ID).Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	var commits []api.Commit
	commitNode := versionmgr.NewWrapCommitNode(repositoryCtl.Repo.CommitRepo(repository.ID), commit)
	iter := versionmgr.NewCommitPreorderIter(ctx, commitNode, nil, nil)
	for {
		commit, err := iter.Next()
		if err == nil {
			if params.After != nil {
				parseTime := time.UnixMilli(*params.After)
				if commit.Commit().Committer.When.Add(time.Nanosecond).After(parseTime) {
					continue
				}
			}
			if params.Amount != nil && len(commits) == *params.Amount {
				break
			}
			modelCommit := commit.Commit()
			commits = append(commits, *commitToDto(modelCommit))
			continue
		}
		if err == io.EOF {
			break
		}
		w.Error(err)
		return
	}
	w.JSON(commits)
}

func (repositoryCtl RepositoryController) ChangeVisible(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.ChangeVisibleParams) {
	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if !repositoryCtl.authorizeMember(ctx, w, repo.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.UpdateVisibleAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repo.ID.String()),
		},
	}) {
		return
	}

	updateParams := models.NewUpdateRepoParams(repo.ID).SetVisible(params.Visible)
	err = repositoryCtl.Repo.RepositoryRepo().UpdateByID(ctx, updateParams)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func repositoryToDto(repository *models.Repository) *api.Repository {
	return &api.Repository{
		CreatedAt:   repository.CreatedAt.UnixMilli(),
		CreatorId:   repository.CreatorID,
		Description: repository.Description,
		Head:        repository.HEAD,
		Id:          repository.ID,
		Name:        repository.Name,
		UpdatedAt:   repository.UpdatedAt.UnixMilli(),
	}
}
