package controller

import (
	"bytes"
	"context"
	"encoding/hex"
	"errors"
	"fmt"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/versionmgr"

	"github.com/google/uuid"
	openapi_types "github.com/oapi-codegen/runtime/types"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type WipController struct {
	fx.In

	Repo models.IRepo
}

func (wipCtl WipController) CreateWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repository string, refID openapi_types.UUID, params api.CreateWipParams) {
	user, err := wipCtl.Repo.UserRepo().Get(ctx, &models.GetUserParam{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := wipCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreateID: user.ID,
		Name:     utils.String(repository),
	})
	if err != nil {
		w.Error(err)
		return
	}

	baseCommitID, err := hex.DecodeString(params.BaseCommitID)
	if err != nil {
		w.Error(err)
		return
	}

	wip := &models.WorkingInProcess{
		CurrentTree:  baseCommitID,
		BaseTree:     baseCommitID,
		RepositoryID: repo.ID,
		RefID:        refID,
		State:        0,
		Name:         params.Name,
		CreateID:     user.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	wip, err = wipCtl.Repo.WipRepo().Insert(ctx, wip)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(wip, http.StatusCreated)
}

func (wipCtl WipController) GetWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repositoryName string, refID openapi_types.UUID) {
	user, err := wipCtl.Repo.UserRepo().Get(ctx, &models.GetUserParam{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{Name: utils.String(repositoryName)})
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, &models.GetWipParam{
		RefID:        refID,
		CreateID:     user.ID,
		RepositoryID: repository.ID,
	})
	if err != nil {
		if errors.Is(err, models.ErrNotFound) {
			w.NotFound()
			return
		}
		w.Error(err)
		return
	}

	w.JSON(wip)
}

func (wipCtl WipController) ListWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repositoryName string) {
	user, err := wipCtl.Repo.UserRepo().Get(ctx, &models.GetUserParam{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{Name: utils.String(repositoryName)})
	if err != nil {
		w.Error(err)
		return
	}

	wips, err := wipCtl.Repo.WipRepo().List(ctx, &models.ListWipParam{
		CreateID:     user.ID,
		RepositoryID: repository.ID,
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(wips)
}

func (wipCtl WipController) CommitWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repositoryName string, refID openapi_types.UUID, params api.CommitWipParams) {
	user, err := wipCtl.Repo.UserRepo().Get(ctx, &models.GetUserParam{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{Name: utils.String(repositoryName)})
	if err != nil {
		w.Error(err)
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, &models.GetRefParams{ID: refID})
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := wipCtl.Repo.ObjectRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, &models.GetWipParam{
		RefID:        refID,
		CreateID:     user.ID,
		RepositoryID: repository.ID,
	})
	if err != nil {
		w.Error(err)
		return
	}

	if !bytes.Equal(commit.TreeHash, wip.BaseTree) {
		w.Error(fmt.Errorf("base commit not equal with branch, please update wip"))
		return
	}
	var msg string
	if params.Msg != nil {
		msg = *params.Msg
	}

	//add commit
	err = wipCtl.Repo.Transaction(ctx, func(repo models.IRepo) error {
		commitOp := versionmgr.NewCommitOp(repo, commit)
		commit, err := commitOp.AddCommit(ctx, user, wip.ID, msg)
		if err != nil {
			return err
		}

		wip.BaseTree = commit.Commit().TreeHash //set for response
		err = repo.WipRepo().UpdateBaseHash(ctx, wip.ID, commit.Commit().TreeHash)
		if err != nil {
			return err
		}

		return repo.RefRepo().UpdateCommitHash(ctx, refID, commit.Commit().Hash)
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(wip)
}

// DeleteWip delete a active working in process
func (wipCtl WipController) DeleteWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, userName string, repositoryName string, refID openapi_types.UUID) {
	user, err := wipCtl.Repo.UserRepo().Get(ctx, &models.GetUserParam{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{Name: utils.String(repositoryName)})
	if err != nil {
		w.Error(err)
		return
	}

	err = wipCtl.Repo.WipRepo().Delete(ctx, &models.DeleteWipParam{
		ID:           uuid.UUID{},
		CreateID:     user.ID,
		RepositoryID: repository.ID,
		RefID:        refID,
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.OK()
}
