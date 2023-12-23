package versionmgr

import (
	"context"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestAdapterFromConfig(t *testing.T) {
	ctx := context.Background()
	t.Run("success", func(t *testing.T) {
		data := `{"Type":"local","DefaultNamespacePrefix":null,"Local":{"Path":"~/.jiaozifs/blockstore","ImportEnabled":false,"ImportHidden":false,"AllowedExternalPrefixes":null},"S3":null,"Azure":null,"GS":null}`
		adapter, err := AdapterFromConfig(ctx, data)
		require.NoError(t, err)
		require.Equal(t, "local", adapter.BlockstoreType())
	})

	t.Run("marshal fail", func(t *testing.T) {
		data := `{"Type":"local",DefaultNamespacePrefix":null,"Local":{"Path":"~/.jiaozifs/blockstore","ImportEnabled":false,"ImportHidden":false,"AllowedExternalPrefixes":null},"S3":null,"Azure":null,"GS":null}`
		_, err := AdapterFromConfig(ctx, data)
		require.Error(t, err)
	})

	t.Run("unsupport type", func(t *testing.T) {
		data := `{"Type":"mock"}`
		_, err := AdapterFromConfig(ctx, data)
		require.Error(t, err)
	})
}
