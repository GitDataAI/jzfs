package controller

import (
	"context"
	"net/http"
	"time"

	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"go.uber.org/fx"
)

var branchLog = logging.Logger("branch_ctl")

type BranchController struct {
	fx.In

	Repo models.IRepo
}

func (bct BranchController) ListBranches(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repoName string) {
	// Get user
	user, err := bct.Repo.UserRepo().Get(ctx, &models.GetUserParams{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}
	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repoName),
	})
	if err != nil {
		w.Error(err)
		return
	}
	// List branches
	branches, err := bct.Repo.RefRepo().List(ctx, repository.ID)
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

func (bct BranchController) CreateBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.CreateBranchJSONRequestBody, userName string, repoName string) {
	// Decode request body
	bc := api.BranchCreation{
		Name:   body.Name,
		Source: body.Source,
	}
	branchLog.Info(bc)
	// Get user
	user, err := bct.Repo.UserRepo().Get(ctx, &models.GetUserParams{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}
	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repoName),
	})
	if err != nil {
		w.Error(err)
		return
	}
	// Get source ref
	params := models.NewGetRefParams()
	params.SetName(bc.Source)
	params.SetRepositoryID(repository.ID)
	ref, err := bct.Repo.RefRepo().Get(ctx, params)
	// Create branch
	newRef := &models.Ref{
		RepositoryID: repository.ID,
		CommitHash:   ref.CommitHash,
		Name:         bc.Name,
		Description:  ref.Description,
		CreatorID:    user.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	_, err = bct.Repo.RefRepo().Insert(ctx, newRef)
	if err != nil {
		w.Error(err)
		return
	}
	w.String("Branch created successfully")
}

func (bct BranchController) DeleteBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repoName string, branch string) {
	// Get user
	user, err := bct.Repo.UserRepo().Get(ctx, &models.GetUserParams{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}
	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repoName),
	})
	if err != nil {
		w.Error(err)
		return
	}
	// Delete branch
	params := models.NewDeleteRefParams()
	params.SetName(branch)
	params.SetRepositoryID(repository.ID)
	err = bct.Repo.RefRepo().Delete(ctx, params)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (bct BranchController) GetBranch(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repoName string, branch string) {
	// Get user
	user, err := bct.Repo.UserRepo().Get(ctx, &models.GetUserParams{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}
	// Get repo
	repository, err := bct.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repoName),
	})
	if err != nil {
		w.Error(err)
		return
	}
	// Get branch
	params := models.NewGetRefParams()
	params.SetName(branch)
	params.SetRepositoryID(repository.ID)
	ref, err := bct.Repo.RefRepo().Get(ctx, params)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(api.Ref{
		CommitHash: ref.CommitHash.Hex(),
		Name:       ref.Name,
	})
}
