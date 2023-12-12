package controller

import (
	"context"
	"errors"
	"net/http"
	"regexp"
	"time"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"go.uber.org/fx"
)

var maxNameLength = 20
var alphanumeric = regexp.MustCompile("^[a-zA-Z0-9_]*$")

var RepoNameBlackList = []string{"repository"}

func CheckRepositoryName(name string) error {
	for _, blackName := range RepoNameBlackList {
		if name == blackName {
			return errors.New("repository name is black list")
		}
	}

	if !alphanumeric.MatchString(name) {
		return errors.New("repository name must be combination of number and letter")
	}
	if len(name) > maxNameLength {
		return errors.New("repository name is too long")
	}
	return nil
}

type RepositoryController struct {
	fx.In

	Repo models.IRepo
}

func (repositoryCtl RepositoryController) ListRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string) {
	user, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(userName))
	if err != nil {
		w.Error(err)
		return
	}

	repositories, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, &models.ListRepoParams{
		CreatorID: user.ID,
	})
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(repositories)
}

func (repositoryCtl RepositoryController) CreateRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, params api.CreateRepositoryParams) {
	err := CheckRepositoryName(params.Name)
	if err != nil {
		w.BadRequest(err.Error())
		return
	}

	user, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(userName))
	if err != nil {
		w.Error(err)
		return
	}

	repository := &models.Repository{
		Name:        params.Name,
		Description: params.Description,
		HEAD:        "main",
		CreatorID:   user.ID,
		CreatedAt:   time.Now(),
		UpdatedAt:   time.Now(),
	}
	repository, err = repositoryCtl.Repo.RepositoryRepo().Insert(ctx, repository)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(repository)
}

func (repositoryCtl RepositoryController) DeleteRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repositoryName string) {
	user, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(userName))
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repositoryName),
	})
	if err != nil {
		w.Error(err)
		return
	}

	err = repositoryCtl.Repo.RepositoryRepo().Delete(ctx, models.NewDeleteRepoParams().SetID(repo.ID))
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (repositoryCtl RepositoryController) GetRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repositoryName string) {
	user, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(userName))
	if err != nil {
		w.Error(err)
		return
	}
	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repositoryName),
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(repo)
}

func (repositoryCtl RepositoryController) UpdateRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.UpdateRepositoryJSONRequestBody, userName string, repositoryName string) {
	user, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(userName))
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repositoryName),
	})
	if err != nil {
		w.Error(err)
		return
	}

	err = repositoryCtl.Repo.RepositoryRepo().UpdateByID(ctx, models.NewUpdateRepoParams(repo.ID).SetDescription(utils.StringValue(body.Description)))
	if err != nil {
		w.Error(err)
		return
	}
}
