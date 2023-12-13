package controller

import (
	"context"
	"errors"
	"io"
	"net/http"
	"regexp"
	"time"

	openapi_types "github.com/oapi-codegen/runtime/types"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/versionmgr"

	"github.com/jiaozifs/jiaozifs/auth"

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

func (repositoryCtl RepositoryController) ListRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request) {
	user, err := auth.GetUser(ctx)
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

func (repositoryCtl RepositoryController) CreateRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateRepositoryJSONRequestBody) {
	err := CheckRepositoryName(body.Name)
	if err != nil {
		w.BadRequest(err.Error())
		return
	}

	user, err := auth.GetUser(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	repository := &models.Repository{
		Name:        body.Name,
		Description: body.Description,
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

func (repositoryCtl RepositoryController) GetCommitsInRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repositoryName string, params api.GetCommitsInRepositoryParams) {
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

	refName := repo.HEAD
	if params.RefName != nil {
		refName = *params.RefName
	}
	ref, err := repositoryCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(refName))
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := repositoryCtl.Repo.ObjectRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	var commits []api.Commit
	iter := versionmgr.NewCommitPreorderIter(versionmgr.NewCommitNode(ctx, commit, repositoryCtl.Repo.ObjectRepo()), nil, nil)
	for {
		commit, err := iter.Next()
		if err == nil {
			modelCommit := commit.Commit()
			commits = append(commits, api.Commit{
				Author: api.Signature{
					Email: openapi_types.Email(modelCommit.Author.Email),
					Name:  modelCommit.Author.Name,
					When:  modelCommit.Author.When,
				},
				Committer: api.Signature{
					Email: openapi_types.Email(modelCommit.Committer.Email),
					Name:  modelCommit.Committer.Name,
					When:  modelCommit.Committer.When,
				},
				CreatedAt:    modelCommit.CreatedAt,
				Hash:         modelCommit.Hash.Hex(),
				MergeTag:     modelCommit.MergeTag,
				Message:      modelCommit.Message,
				ParentHashes: hash.HexArrayOfHashes(modelCommit.ParentHashes...),
				TreeHash:     modelCommit.TreeHash.Hex(),
				Type:         int8(modelCommit.Type),
				UpdatedAt:    modelCommit.UpdatedAt,
			})
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

func (repositoryCtl RepositoryController) ListRepositories(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string) {
	user, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(userName))
	if err != nil {
		w.Error(err)
		return
	}

	repos, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, models.NewListRepoParams().SetCreatorID(user.ID))
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(repos)
}
