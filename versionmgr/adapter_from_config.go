package versionmgr

import (
	"context"
	"encoding/json"

	"github.com/GitDataAI/jiaozifs/block"
	"github.com/GitDataAI/jiaozifs/block/factory"
	"github.com/GitDataAI/jiaozifs/config"
)

func AdapterFromConfig(ctx context.Context, jsonParams string) (block.Adapter, error) {
	var cfg = config.BlockStoreConfig{}
	err := json.Unmarshal([]byte(jsonParams), &cfg)
	if err != nil {
		return nil, err
	}
	adapter, err := factory.BuildBlockAdapter(ctx, &cfg)
	if err != nil {
		return nil, err
	}
	return adapter, err
}
