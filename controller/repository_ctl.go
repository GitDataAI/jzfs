package controller

import (
	"context"
	"errors"
	"io"
	"net/http"
	"regexp"
	"time"

	"github.com/google/uuid"

	openapi_types "github.com/oapi-codegen/runtime/types"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/versionmgr"

	"github.com/jiaozifs/jiaozifs/auth"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"go.uber.org/fx"
)

const DefaultBranchName = "main"

var maxNameLength = 20
var alphanumeric = regexp.MustCompile("^[a-zA-Z0-9_]*$")

// RepoNameBlackList forbid repo name, reserve for routes
var RepoNameBlackList = []string{"repository", "repositories", "wip", "wips", "object", "objects", "commit", "commits", "ref", "refs", "repo", "repos", "user", "users"}

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

func (repositoryCtl RepositoryController) ListRepositoryOfAuthenticatedUser(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	repositories, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, models.NewListRepoParams().SetOwnerID(operator.ID)) //operator is owner
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(repositories)
}

func (repositoryCtl RepositoryController) ListRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, params api.ListRepositoryParams) {
	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}
	if owner.ID != operator.ID { //todo check public or private and allow  access public repos
		w.Forbidden()
		return
	}

	listParams := models.NewListRepoParams().SetOwnerID(owner.ID)
	if params.RepoPrefix != nil && len(*params.RepoPrefix) > 0 {
		listParams.SetName(*params.RepoPrefix, models.PrefixMatch)
	}
	repositories, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, listParams)
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

	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}
	//create default ref
	var createdRepo *models.Repository
	err = repositoryCtl.Repo.Transaction(ctx, func(repo models.IRepo) error {
		repoID := uuid.New()
		defaultRef := &models.Branches{
			RepositoryID: repoID,
			CommitHash:   hash.Hash{},
			Name:         DefaultBranchName,
			CreatorID:    operator.ID,
			CreatedAt:    time.Now(),
			UpdatedAt:    time.Now(),
		}
		defaultRef, err := repositoryCtl.Repo.BranchRepo().Insert(ctx, defaultRef)
		if err != nil {
			return err
		}
		repository := &models.Repository{
			ID:          repoID,
			Name:        body.Name,
			Description: body.Description,
			HEAD:        defaultRef.Name,
			OwnerID:     operator.ID, // this api only create repo for operator
			CreatorID:   operator.ID,
			CreatedAt:   time.Now(),
			UpdatedAt:   time.Now(),
		}
		createdRepo, err = repositoryCtl.Repo.RepositoryRepo().Insert(ctx, repository)
		return err
	})

	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(api.Repository{
		CreatedAt:   createdRepo.CreatedAt,
		CreatorID:   createdRepo.CreatorID,
		Description: createdRepo.Description,
		Head:        createdRepo.HEAD,
		ID:          createdRepo.ID,
		Name:        createdRepo.Name,
		UpdatedAt:   createdRepo.UpdatedAt,
	})
}

func (repositoryCtl RepositoryController) DeleteRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name {
		w.Forbidden()
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	affectRows, err := repositoryCtl.Repo.RepositoryRepo().Delete(ctx, models.NewDeleteRepoParams().SetID(repo.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if affectRows == 0 {
		w.NotFound()
		return
	}
	w.OK()
}

func (repositoryCtl RepositoryController) GetRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name { //todo check public or private / and permission
		w.Forbidden()
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(repo)
}

func (repositoryCtl RepositoryController) UpdateRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.UpdateRepositoryJSONRequestBody, ownerName string, repositoryName string) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != ownerName { //todo check permission to modify owner repo
		w.Forbidden()
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
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

func (repositoryCtl RepositoryController) GetCommitsInRepository(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.GetCommitsInRepositoryParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != ownerName { //todo check public or private
		w.Forbidden()
		return
	}

	repository, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
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
	commitNode := versionmgr.NewCommitNode(ctx, commit, repositoryCtl.Repo.CommitRepo(repository.ID))
	iter := versionmgr.NewCommitPreorderIter(commitNode, nil, nil)
	for {
		commit, err := iter.Next()
		if err == nil {
			modelCommit := commit.Commit()
			commits = append(commits, api.Commit{
				RepositoryID: modelCommit.RepositoryID,
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
