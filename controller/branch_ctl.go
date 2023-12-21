package controller

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"reflect"
	"regexp"
	"strings"
	"time"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/auth"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

var MaxBranchNameLength = 40
var branchNameRegex = regexp.MustCompile("^[a-zA-Z0-9_]*$")

func paginationForBranches(hasMore bool, results interface{}, fieldName string) api.Pagination {
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
	pagination.NextOffset = token.String()
	return pagination
}

func CheckBranchName(name string) error {
	for _, blackName := range RepoNameBlackList {
		if name == blackName {
			return errors.New("repository name is black list")
		}
	}

	if len(name) > MaxBranchNameLength {
		return fmt.Errorf("branch name is too long")
	}

	seg := strings.Split(name, "/")
	if len(seg) > 2 {
		return fmt.Errorf("branch format must be <name> or <name>/<name>")
	}

	if !branchNameRegex.Match([]byte(seg[0])) || !branchNameRegex.Match([]byte(seg[1])) {
		return fmt.Errorf("branch name must be combination of number and letter or combine with '/'")
	}
	return nil
}

type BranchController struct {
	fx.In

	Repo models.IRepo
}

func (bct BranchController) ListBranches(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.ListBranchesParams) {
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

	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name {
		w.Forbidden()
		return
	}

	listBranchParams := models.NewListBranchParams()
	if params.Prefix != nil && len(*params.Prefix) > 0 {
		listBranchParams.SetName(*params.Prefix, models.PrefixMatch)
	}
	if params.After != nil {
		listBranchParams.SetAfter(*params.After)
	}
	if params.Amount != nil {
		i := *params.Amount
		if i > DefaultMaxPerPage || i <= 0 {
			listBranchParams.SetAmount(DefaultMaxPerPage)
		} else {
			listBranchParams.SetAmount(i)
		}
	} else {
		listBranchParams.SetAmount(DefaultMaxPerPage)
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
			CreatedAt:    branch.CreatedAt,
			CreatorID:    branch.CreatorID,
			Description:  branch.Description,
			ID:           branch.ID,
			Name:         branch.Name,
			RepositoryID: branch.RepositoryID,
			UpdatedAt:    branch.UpdatedAt,
		}
		results = append(results, r)
	}
	w.JSON(api.BranchList{
		Pagination: paginationForBranches(hasMore, results, "Name"),
		Results:    results,
	})
}

func (bct BranchController) CreateBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateBranchJSONRequestBody, ownerName string, repositoryName string) {
	if err := CheckBranchName(body.Name); err != nil {
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

	if operator.Name != owner.Name {
		w.Forbidden()
		return
	}

	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	//check exit
	_, err = bct.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetName(body.Name).SetRepositoryID(repository.ID))
	if err == nil {
		w.BadRequest(fmt.Sprintf("%s already exit", body.Name))
		return
	}
	if err != nil && !errors.Is(err, models.ErrNotFound) {
		w.Error(err)
		return
	}
	//get source branch
	sourceBranch, err := bct.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetName(body.Source).SetRepositoryID(repository.ID))
	if err != nil && !errors.Is(err, models.ErrNotFound) {
		w.Error(err)
		return
	}

	commitHash := hash.EmptyHash
	if sourceBranch != nil {
		commitHash = sourceBranch.CommitHash
	}

	// Create branch
	newBranch := &models.Branches{
		RepositoryID: repository.ID,
		CommitHash:   commitHash,
		Name:         body.Name,
		CreatorID:    operator.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	newBranch, err = bct.Repo.BranchRepo().Insert(ctx, newBranch)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(api.Branch{
		CommitHash:   newBranch.CommitHash.Hex(),
		CreatedAt:    newBranch.CreatedAt,
		CreatorID:    newBranch.CreatorID,
		Description:  newBranch.Description,
		ID:           newBranch.ID,
		Name:         newBranch.Name,
		RepositoryID: newBranch.RepositoryID,
		UpdatedAt:    newBranch.UpdatedAt,
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

	if operator.Name != owner.Name {
		w.Forbidden()
		return
	}

	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	// Delete branch
	affectedRows, err := bct.Repo.BranchRepo().Delete(ctx, models.NewDeleteBranchParams().SetName(params.RefName).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}
	if affectedRows == 0 {
		w.NotFound()
		return
	}
	w.OK()
}

func (bct BranchController) GetBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.GetBranchParams) {
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

	if operator.Name != owner.Name {
		w.Forbidden()
		return
	}

	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
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
		CreatedAt:    ref.CreatedAt,
		CreatorID:    ref.CreatorID,
		Description:  ref.Description,
		ID:           ref.ID,
		Name:         ref.Name,
		RepositoryID: ref.RepositoryID,
		UpdatedAt:    ref.UpdatedAt,
	})
}
