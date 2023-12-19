package controller

import (
	"bytes"
	"context"
	"encoding/hex"
	"errors"
	"fmt"
	"net/http"
	"strings"
	"time"

	"github.com/jiaozifs/jiaozifs/utils/hash"

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

// CreateWip create wip of branch
func (wipCtl WipController) CreateWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.CreateWipParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := wipCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name { //todo check permission to operator ownerRepo
		w.Forbidden()
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetRepositoryID(repository.ID).SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	_, err = wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetCreatorID(operator.ID).SetRepositoryID(repository.ID).SetRefID(ref.ID))
	if err == nil {
		w.BadRequest(fmt.Sprintf("ref %s already in wip", params.RefName))
		return
	}
	if err != nil && !errors.Is(err, models.ErrNotFound) {
		w.Error(err)
		return
	}

	currentTreeHash := hash.EmptyHash
	if !ref.CommitHash.IsEmpty() {
		baseCommit, err := wipCtl.Repo.CommitRepo().Commit(ctx, ref.CommitHash)
		if err != nil {
			w.Error(err)
			return
		}
		currentTreeHash = baseCommit.TreeHash
	}

	wip := &models.WorkingInProcess{
		CurrentTree:  currentTreeHash,
		BaseCommit:   ref.CommitHash,
		RepositoryID: repository.ID,
		RefID:        ref.ID,
		State:        0,
		CreatorID:    operator.ID,
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

// GetWip get wip of specific repository, operator only get himself wip
func (wipCtl WipController) GetWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.GetWipParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := wipCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetOwnerID(owner.ID).SetName(repositoryName))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name { //todo check permission to operator ownerRepo
		w.Forbidden()
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetRepositoryID(repository.ID).SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetRefID(ref.ID).SetCreatorID(operator.ID).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(wip)
}

// ListWip return wips of branches, operator only see himself wips in specific repository
func (wipCtl WipController) ListWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := wipCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name { //todo check permission to operator ownerRepo
		w.Forbidden()
		return
	}

	wips, err := wipCtl.Repo.WipRepo().List(ctx, models.NewListWipParams().SetCreatorID(operator.ID).SetRepositoryID(repository.ID))
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(wips)
}

// CommitWip commit wip to branch, operator only could operator himself wip
func (wipCtl WipController) CommitWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName, repositoryName string, params api.CommitWipParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := wipCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name { //todo check permission to operator ownerRepo
		w.Forbidden()
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := wipCtl.Repo.CommitRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetRefID(ref.ID).SetCreatorID(operator.ID).SetRepositoryID(repository.ID))
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
		commit, err := commitOp.AddCommit(ctx, operator, wip.ID, msg)
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

// DeleteWip delete a active working in process operator only can delete himself wip
func (wipCtl WipController) DeleteWip(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.DeleteWipParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := wipCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name { //todo check permission to operator ownerRepo
		w.Forbidden()
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetRepositoryID(repository.ID).SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	deleteWipParams := models.NewDeleteWipParams().
		SetCreatorID(operator.ID). //todo admin delete
		SetRepositoryID(repository.ID).
		SetRefID(ref.ID)

	affectedRaw, err := wipCtl.Repo.WipRepo().Delete(ctx, deleteWipParams)
	if err != nil {
		w.Error(err)
		return
	}

	if affectedRaw == 0 {
		w.NotFound()
		return
	}

	w.OK()
}

// GetWipChanges return wip difference, operator only see himself wip
func (wipCtl WipController) GetWipChanges(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName, repositoryName string, params api.GetWipChangesParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := wipCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := wipCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.Name != owner.Name { //todo check permission to operator ownerRepo
		w.Forbidden()
		return
	}

	ref, err := wipCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetRepositoryID(repository.ID).SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetCreatorID(operator.ID).SetRepositoryID(repository.ID).SetRefID(ref.ID))
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := wipCtl.Repo.CommitRepo().Commit(ctx, wip.BaseCommit)
	if err != nil {
		w.Error(err)
		return
	}

	workTree, err := versionmgr.NewWorkTree(ctx, wipCtl.Repo.FileTreeRepo(), models.NewRootTreeEntry(commit.TreeHash))
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
