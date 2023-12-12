package controller

import (
	"context"
	"encoding/hex"
	"errors"
	"net/http"
	"strings"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/versionmgr"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type CommitController struct {
	fx.In

	Repo models.IRepo
}

func (commitCtl CommitController) GetEntriesInCommit(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, _ string, _ string, commitHashStr string, params api.GetEntriesInCommitParams) {
	commitHash, err := hex.DecodeString(commitHashStr)
	if err != nil {
		w.Error(err)
		return
	}
	commit, err := commitCtl.Repo.ObjectRepo().Get(ctx, &models.GetObjParams{
		Hash: commitHash,
	})
	if err != nil {
		w.Error(err)
		return
	}

	workTree, err := versionmgr.NewWorkTree(ctx, commitCtl.Repo.ObjectRepo(), models.NewRootTreeEntry(commit.TreeHash))
	if err != nil {
		w.Error(err)
		return
	}

	path := ""
	if params.Path != nil {
		path = *params.Path
	}
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

func (commitCtl CommitController) GetCommitDiff(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, _ string, _ string, baseCommitStr string, toCommitStr string, params api.GetCommitDiffParams) {
	bashCommitHash, err := hex.DecodeString(baseCommitStr)
	if err != nil {
		w.Error(err)
		return
	}
	toCommitHash, err := hex.DecodeString(toCommitStr)
	if err != nil {
		w.Error(err)
		return
	}

	bashCommit, err := commitCtl.Repo.ObjectRepo().Commit(ctx, bashCommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	path := ""
	if params.Path != nil {
		path = *params.Path
	}

	commitOp := versionmgr.NewCommitOp(commitCtl.Repo, bashCommit)
	changes, err := commitOp.DiffCommit(ctx, toCommitHash)
	if err != nil {
		w.Error(err)
		return
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
		}
		return nil
	})
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(changesResp)
}
