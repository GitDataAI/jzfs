package controller

import (
	"context"
	"errors"
	"io"
	"net/http"
	"reflect"
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

const (
	DefaultBranchName     = "main"
	DefaultMaxPerPage int = 1000
)

var maxNameLength = 20
var alphanumeric = regexp.MustCompile("^[a-zA-Z0-9_]*$")

// RepoNameBlackList forbid repo name, reserve for routes
var RepoNameBlackList = []string{"repository", "repositories", "wip", "wips", "object", "objects", "commit", "commits", "ref", "refs", "repo", "repos", "user", "users"}

func paginationFor(hasMore bool, results interface{}, fieldName string) api.Pagination {
	pagination := api.Pagination{
		HasMore:    hasMore,
		MaxPerPage: DefaultMaxPerPage,
	}
	if results == nil {
		return pagination
	}
	if reflect.TypeOf(results).Kind() != reflect.Slice {
		panic("results is not a slice")
	}
	s := reflect.ValueOf(results)
	pagination.Results = s.Len()
	if !hasMore || pagination.Results == 0 {
		return pagination
	}
	v := s.Index(pagination.Results - 1)
	token := v.FieldByName(fieldName)
	t := token.Interface().(time.Time)
	pagination.NextOffset = t.String()
	return pagination
}

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

func (repositoryCtl RepositoryController) ListRepositoryOfAuthenticatedUser(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, params api.ListRepositoryOfAuthenticatedUserParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	listParams := models.NewListRepoParams()
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil {
		listParams.SetAfter(*params.After)
	}
	if params.Amount != nil {
		i := int(*params.Amount)
		if i > DefaultMaxPerPage || i <= 0 {
			listParams.SetAmount(DefaultMaxPerPage)
		} else {
			listParams.SetAmount(i)
		}
	} else {
		listParams.SetAmount(DefaultMaxPerPage)
	}

	repositories, has_more, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, listParams.
		SetOwnerID(operator.ID))
	if err != nil {
		w.Error(err)
		return
	}
	results := make([]api.Repository, 0, len(repositories))
	for _, repo := range repositories {
		r := api.Repository{
			CreatedAt:   repo.CreatedAt,
			CreatorID:   repo.CreatorID,
			Description: repo.Description,
			Head:        repo.HEAD,
			ID:          repo.ID,
			Name:        repo.Name,
			UpdatedAt:   repo.UpdatedAt,
		}
		results = append(results, r)
	}
	w.JSON(api.RepositoryList{
		Pagination: paginationFor(has_more, results, "UpdatedAt"),
		Results:    results,
	})
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
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil {
		listParams.SetAfter(*params.After)
	}
	if params.Amount != nil {
		i := int(*params.Amount)
		if i > DefaultMaxPerPage || i <= 0 {
			listParams.SetAmount(DefaultMaxPerPage)
		} else {
			listParams.SetAmount(i)
		}
	} else {
		listParams.SetAmount(DefaultMaxPerPage)
	}

	repositories, has_more, err := repositoryCtl.Repo.RepositoryRepo().List(ctx, listParams)
	if err != nil {
		w.Error(err)
		return
	}
	results := make([]api.Repository, 0, len(repositories))
	for _, repo := range repositories {
		r := api.Repository{
			CreatedAt:   repo.CreatedAt,
			CreatorID:   repo.CreatorID,
			Description: repo.Description,
			Head:        repo.HEAD,
			ID:          repo.ID,
			Name:        repo.Name,
			UpdatedAt:   repo.UpdatedAt,
		}
		results = append(results, r)
	}
	w.JSON(api.RepositoryList{
		Pagination: paginationFor(has_more, results, "UpdatedAt"),
		Results:    results,
	})
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
		defaultRef := &models.Ref{
			RepositoryID: repoID,
			CommitHash:   hash.Hash{},
			Name:         DefaultBranchName,
			CreatorID:    operator.ID,
			CreatedAt:    time.Now(),
			UpdatedAt:    time.Now(),
		}
		defaultRef, err := repositoryCtl.Repo.RefRepo().Insert(ctx, defaultRef)
		if err != nil {
			return err
		}
		repository := &models.Repository{
			ID:          repoID,
			Name:        body.Name,
			Description: body.Description,
			HEAD:        defaultRef.Name,
			OwnerID:     operator.ID,
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

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(operator.ID))
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
	user, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := repositoryCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	if user.Name != ownerName { //todo check public or private
		w.Forbidden()
		return
	}

	repo, err := repositoryCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	refName := repo.HEAD
	if params.RefName != nil {
		refName = *params.RefName
	}
	ref, err := repositoryCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetRepositoryID(repo.ID).SetName(refName))
	if err != nil {
		w.Error(err)
		return
	}

	if ref.CommitHash.IsEmpty() {
		w.JSON([]api.Commit{})
		return
	}

	commit, err := repositoryCtl.Repo.CommitRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	var commits []api.Commit
	iter := versionmgr.NewCommitPreorderIter(versionmgr.NewCommitNode(ctx, commit, repositoryCtl.Repo.CommitRepo()), nil, nil)
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
