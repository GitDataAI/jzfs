package controller

import (
	"bytes"
	"context"
	"encoding/hex"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/versionmgr"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type WipController struct {
	fx.In

	Repo models.IRepo
}

func (wipCtl WipController) CreateWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, repository string, params api.CreateWipParams) {
	user, err := auth.GetUser(ctx)
	if err != nil {
		w.Error(err)
		return
	}
	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := wipCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreatorID: user.ID,
		Name:      utils.String(repository),
	})
	if err != nil {
		w.Error(err)
		return
	}

	baseCommit, err := wipCtl.Repo.ObjectRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	wip := &models.WorkingInProcess{
		CurrentTree:  baseCommit.TreeHash,
		BaseCommit:   ref.CommitHash,
		RepositoryID: repo.ID,
		RefID:        ref.ID,
		State:        0,
		Name:         params.Name,
		CreatorID:    user.ID,
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

func (wipCtl WipController) GetWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, repositoryName string, params api.GetWipParams) {
	user, err := auth.GetUser(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, &models.GetWipParams{
		RefID:        ref.ID,
		CreatorID:    user.ID,
		RepositoryID: repository.ID,
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(wip)
}

func (wipCtl WipController) ListWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, repositoryName string) {
	user, err := auth.GetUser(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{Name: utils.String(repositoryName)})
	if err != nil {
		w.Error(err)
		return
	}

	wips, err := wipCtl.Repo.WipRepo().List(ctx, models.NewListWipParams().SetCreatorID(user.ID).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(wips)
}

func (wipCtl WipController) CommitWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, repositoryName string, params api.CommitWipParams) {
	user, err := auth.GetUser(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := wipCtl.Repo.ObjectRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetRefID(ref.ID).SetCreatorID(user.ID).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if !bytes.Equal(commit.Hash, wip.BaseCommit) {
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

		wip.BaseCommit = commit.Commit().Hash //set for response
		err = repo.WipRepo().UpdateByID(ctx, models.NewUpdateWipParams(wip.ID).SetBaseCommit(wip.BaseCommit))
		if err != nil {
			return err
		}

		return repo.RefRepo().UpdateByID(ctx, models.NewUpdateRefParams(ref.ID).SetCommitHash(commit.Commit().Hash))
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(wip)
}

// DeleteWip delete a active working in process
func (wipCtl WipController) DeleteWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, repositoryName string, params api.DeleteWipParams) {
	user, err := auth.GetUser(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	deleteWipParams := models.NewDeleteWipParams().
		SetCreatorID(user.ID).
		SetRepositoryID(repository.ID).
		SetRefID(ref.ID)

	err = wipCtl.Repo.WipRepo().Delete(ctx, deleteWipParams)
	if err != nil {
		w.Error(err)
		return
	}

	w.OK()
}

func (wipCtl WipController) GetWipChanges(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, repositoryName string, params api.GetWipChangesParams) {
	user, err := auth.GetUser(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetCreatorID(user.ID).SetRepositoryID(repository.ID).SetRefID(ref.ID))
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := wipCtl.Repo.ObjectRepo().Commit(ctx, wip.BaseCommit)
	if err != nil {
		w.Error(err)
		return
	}

	workTree, err := versionmgr.NewWorkTree(ctx, wipCtl.Repo.ObjectRepo(), models.NewRootTreeEntry(commit.TreeHash))
	if err != nil {
		w.Error(err)
		return
	}

	if bytes.Equal(commit.TreeHash, wip.CurrentTree) {
		w.JSON([]api.Change{}) //no change return nothing
		return
	}

	changes, err := workTree.Diff(ctx, wip.CurrentTree)
	if err != nil {
		w.Error(err)
		return
	}

	var path string
	if params.Path != nil {
		path = *params.Path
	}

	var changesResp []api.Change
	err = changes.ForEach(func(change versionmgr.IChange) error {
		action, err := change.Action()
		if err != nil {
			return err
		}
		fullPath := change.Path()
		if strings.HasPrefix(fullPath, path) {
			apiChange := api.Change{
				Action: int(action),
				Path:   fullPath,
			}
			if change.From() != nil {
				apiChange.BaseHash = utils.String(hex.EncodeToString(change.From().Hash()))
			}
			if change.To() != nil {
				apiChange.ToHash = utils.String(hex.EncodeToString(change.To().Hash()))
			}
			changesResp = append(changesResp, apiChange)
		}
		return nil
	})
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(changes)
}
