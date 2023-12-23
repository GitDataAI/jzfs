package controller

import (
	"context"
	"encoding/hex"
	"errors"
	"net/http"
	"strings"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/block/params"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/jiaozifs/jiaozifs/versionmgr"
	"go.uber.org/fx"
)

type CommitController struct {
	fx.In

	Repo                models.IRepo
	PublicStorageConfig params.AdapterConfig
}

func (commitCtl CommitController) GetEntriesInRef(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.GetEntriesInRefParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := commitCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := commitCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	refName := repository.HEAD
	if params.Path != nil {
		refName = *params.Ref
	}

	if operator.Name != ownerName { //todo check permission
		w.Forbidden()
		return
	}

	treeHash := hash.EmptyHash
	if params.Type == api.RefTypeWip {
		//todo maybe from tag reference
		ref, err := commitCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(refName))
		if err != nil {
			w.Error(err)
			return
		}
		wip, err := commitCtl.Repo.WipRepo().Get(ctx, models.NewGetWipParams().SetCreatorID(operator.ID).SetRepositoryID(repository.ID).SetRefID(ref.ID))
		if err != nil {
			w.Error(err)
			return
		}
		treeHash = wip.CurrentTree
	} else if params.Type == api.RefTypeBranch {
		ref, err := commitCtl.Repo.BranchRepo().Get(ctx, models.NewGetBranchParams().SetRepositoryID(repository.ID).SetName(refName))
		if err != nil {
			w.Error(err)
			return
		}
		if !ref.CommitHash.IsEmpty() {
			commit, err := commitCtl.Repo.CommitRepo(repository.ID).Commit(ctx, ref.CommitHash)
			if err != nil {
				w.Error(err)
				return
			}
			treeHash = commit.TreeHash
		}
	} else {
		//check in validate middleware, test cant cover here, keep this check
		w.BadRequest("not support")
		return
	}

	workTree, err := versionmgr.NewWorkTree(ctx, commitCtl.Repo.FileTreeRepo(repository.ID), models.NewRootTreeEntry(treeHash))
	if err != nil {
		w.Error(err)
		return
	}

	path := versionmgr.CleanPath(utils.StringValue(params.Path))
	treeEntry, err := workTree.Ls(ctx, path)
	if err != nil {
		if errors.Is(err, versionmgr.ErrPathNotFound) {
			w.NotFound()
			return
		}
		w.Error(err)
		return
	}
	w.JSON(treeEntry)
}

func (commitCtl CommitController) GetCommitDiff(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, basehead string, params api.GetCommitDiffParams) {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Error(err)
		return
	}

	owner, err := commitCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := commitCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}

	if operator.ID != owner.ID { //todo check permission
		w.Forbidden()
		return
	}

	baseHead := strings.Split(basehead, "...")
	if len(baseHead) != 2 {
		w.BadRequest("invalid basehead must be base...head")
		return
	}

	toCommitHash, err := hex.DecodeString(baseHead[1])
	if err != nil {
		w.Error(err)
		return
	}

	workRepo, err := versionmgr.NewWorkRepositoryFromConfig(ctx, operator, repository, commitCtl.Repo, commitCtl.PublicStorageConfig)
	if err != nil {
		w.Error(err)
		return
	}

	err = workRepo.CheckOut(ctx, versionmgr.InCommit, baseHead[0])
	if err != nil {
		w.Error(err)
		return
	}

	changes, err := workRepo.DiffCommit(ctx, toCommitHash)
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
