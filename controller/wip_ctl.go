package controller

import (
	"bytes"
	"context"
	"encoding/hex"
	"errors"
	"net/http"
	"strings"
	"time"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/block/params"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/jiaozifs/jiaozifs/versionmgr"
	"go.uber.org/fx"
)

type WipController struct {
	fx.In

	Repo                models.IRepo
	PublicStorageConfig params.AdapterConfig
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

	ref, err := wipCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetCreatorID(operator.ID).SetRepositoryID(repository.ID).SetRefID(ref.ID))
	if err == nil {
		w.JSON(wip)
		return
	}
	if err != nil && !errors.Is(err, models.ErrNotFound) {
		w.Error(err)
		return
	}

	currentTreeHash := hash.EmptyHash
	if !ref.CommitHash.IsEmpty() {
		baseCommit, err := wipCtl.Repo.CommitRepo(repository.ID).Commit(ctx, ref.CommitHash)
		if err != nil {
			w.Error(err)
			return
		}
		currentTreeHash = baseCommit.TreeHash
	}

	wip = &models.WorkingInProcess{
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

	ref, err := wipCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetRefID(ref.ID).SetCreatorID(operator.ID).SetRepositoryID(repository.ID))
	if err == nil {
		w.JSON(wip)
		return
	}

	if err != nil && !errors.Is(err, models.ErrNotFound) {
		w.Error(err)
		return
	}

	// if not found create a wip
	currentTreeHash := hash.EmptyHash
	if !ref.CommitHash.IsEmpty() {
		baseCommit, err := wipCtl.Repo.CommitRepo(repository.ID).Commit(ctx, ref.CommitHash)
		if err != nil {
			w.Error(err)
			return
		}
		currentTreeHash = baseCommit.TreeHash
	}

	wip = &models.WorkingInProcess{
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

	workRepo, err := versionmgr.NewWorkRepositoryFromConfig(ctx, operator, repository, wipCtl.Repo, wipCtl.PublicStorageConfig)
	if err != nil {
		w.Error(err)
		return
	}

	err = workRepo.CheckOut(ctx, versionmgr.InWip, params.RefName)
	if err != nil {
		w.Error(err)
		return
	}

	_, err = workRepo.CommitChanges(ctx, params.Msg)
	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(workRepo.CurWip(), http.StatusCreated)
}

// DeleteWip delete active working in process operator only can delete himself wip
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

	ref, err := wipCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(params.RefName))
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

	ref, err := wipCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(params.RefName))
	if err != nil {
		w.Error(err)
		return
	}

	wip, err := wipCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetCreatorID(operator.ID).SetRepositoryID(repository.ID).SetRefID(ref.ID))
	if err != nil {
		w.Error(err)
		return
	}

	treeHash := hash.EmptyHash
	if !wip.BaseCommit.IsEmpty() {
		commit, err := wipCtl.Repo.CommitRepo(repository.ID).Commit(ctx, wip.BaseCommit)
		if err != nil {
			w.Error(err)
			return
		}
		treeHash = commit.TreeHash
	}

	if bytes.Equal(treeHash, wip.CurrentTree) {
		w.JSON([]api.Change{}) //no change return nothing
		return
	}

	workTree, err := versionmgr.NewWorkTree(ctx, wipCtl.Repo.FileTreeRepo(repository.ID), models.NewRootTreeEntry(treeHash))
	if err != nil {
		w.Error(err)
		return
	}

	changes, err := workTree.Diff(ctx, wip.CurrentTree)
	if err != nil {
		w.Error(err)
		return
	}

	path := versionmgr.CleanPath(utils.StringValue(params.Path))

	var changesResp []api.Change
	err = changes.ForEach(func(change versionmgr.IChange) error {
		action, err := change.Action()
		if err != nil {
			return err
		}
		fullPath := change.Path()
		if strings.HasPrefix(fullPath, path) {
			apiChange := api.Change{
				Action: api.ChangeAction(action),
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

	w.JSON(changesResp)
}
