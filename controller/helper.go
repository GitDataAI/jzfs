package controller

import (
	"encoding/hex"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/versionmgr"
)

func changesToDTO(changes *versionmgr.Changes) ([]api.Change, error) {
	changesResp := make([]api.Change, 0)
	err := changes.ForEach(func(change versionmgr.IChange) error {
		action, err := change.Action()
		if err != nil {
			return err
		}
		fullPath := change.Path()
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
		return nil
	})
	if err != nil {
		return nil, err
	}
	return changesResp, nil
}
