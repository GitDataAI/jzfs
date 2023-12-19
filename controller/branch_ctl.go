package controller

import (
	"context"
	"errors"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/auth"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type BranchController struct {
	fx.In

	Repo models.IRepo
}

func (bct BranchController) ListBranches(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string) {
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

	branches, err := bct.Repo.RefRepo().List(ctx, models.NewListRefParams().SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}
	var refs []api.Ref
	for _, branch := range branches {
		ref := api.Ref{
			CommitHash: branch.Name,
			Name:       branch.CommitHash.Hex(),
		}
		refs = append(refs, ref)
	}
	w.JSON(api.RefList{Results: refs})
}

func (bct BranchController) CreateBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateBranchJSONRequestBody, ownerName string, repositoryName string) {
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

	// Get source ref
	ref, err := bct.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(body.Name).SetRepositoryID(repository.ID))
	if err != nil && !errors.Is(err, models.ErrNotFound) {
		w.Error(err)
		return
	}
	// Create branch
	newRef := &models.Ref{
		RepositoryID: repository.ID,
		CommitHash:   ref.CommitHash,
		Name:         body.Name,
		Description:  ref.Description,
		CreatorID:    operator.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	newRef, err = bct.Repo.RefRepo().Insert(ctx, newRef)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(api.Ref{
		CommitHash:   newRef.CommitHash.Hex(),
		CreatedAt:    newRef.CreatedAt,
		CreatorID:    newRef.CreatorID,
		Description:  newRef.Description,
		ID:           newRef.ID,
		Name:         newRef.Name,
		RepositoryID: newRef.RepositoryID,
		UpdatedAt:    newRef.UpdatedAt,
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
	err = bct.Repo.RefRepo().Delete(ctx, models.NewDeleteRefParams().SetName(params.RefName).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
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
	ref, err := bct.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(params.RefName).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(api.Ref{
		CommitHash: ref.CommitHash.Hex(),
		Name:       ref.Name,
	})
}
