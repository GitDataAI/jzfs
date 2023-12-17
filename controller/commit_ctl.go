package controller

import (
	"context"
	"encoding/hex"
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

func (commitCtl CommitController) GetEntriesInCommit(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, _ string, _ string, params api.GetEntriesInCommitParams) {
	refName := "main"
	if params.Path != nil {
		refName = *params.Ref
	}

	ref, err := commitCtl.Repo.RefRepo().Get(ctx, models.NewGetRefParams().SetName(refName))
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := commitCtl.Repo.CommitRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	workTree, err := versionmgr.NewWorkTree(ctx, commitCtl.Repo.FileTreeRepo(), models.NewRootTreeEntry(commit.TreeHash))
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
		w.Error(err)
		return
	}
	w.JSON(treeEntry)
}

func (commitCtl CommitController) GetCommitDiff(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, _ string, _ string, basehead string, params api.GetCommitDiffParams) {
	baseHead := strings.Split(basehead, "...")
	if len(baseHead) != 2 {
		w.BadRequest("invalid basehead must be base...head")
		return
	}

	bashCommitHash, err := hex.DecodeString(baseHead[0])
	if err != nil {
		w.Error(err)
		return
	}
	toCommitHash, err := hex.DecodeString(baseHead[1])
	if err != nil {
		w.Error(err)
		return
	}

	bashCommit, err := commitCtl.Repo.CommitRepo().Commit(ctx, bashCommitHash)
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
