package integrationtest

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/jiaozifs/jiaozifs/testhelper"

	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/cmd"
	"github.com/phayes/freeport"
)

func InitCmd(ctx context.Context, jzHome string, listen string, db string) error {
	buf := new(bytes.Buffer)
	cmd.RootCmd().SetOut(buf)
	cmd.RootCmd().SetErr(buf)
	cmd.RootCmd().SetArgs([]string{"init", "--listen", listen, "--db_debug", "true", "--db", db,
		"--config", fmt.Sprintf("%s/config.toml", jzHome), "--bs_path", fmt.Sprintf("%s/blockstore", jzHome)})

	return cmd.RootCmd().ExecuteContext(ctx)
}

func Daemon(ctx context.Context, writer io.Writer, jzHome string, listen string) error {
	cmd.RootCmd().SetOut(writer)
	cmd.RootCmd().SetErr(writer)
	cmd.RootCmd().SetArgs([]string{"daemon", "--listen", listen, "--config", fmt.Sprintf("%s/config.toml", jzHome)})
	return cmd.RootCmd().ExecuteContext(ctx)
}

type Closer func()

func SetupDaemon(t *testing.T, ctx context.Context) (string, Closer) { //nolint
	pg, connectString, _ := testhelper.SetupDatabase(ctx, t)

	port, err := freeport.GetFreePort()
	require.NoError(t, err)
	url := fmt.Sprintf("http://127.0.0.1:%d", port)
	ctx, cancel := context.WithCancel(ctx)
	tmpDir, err := os.MkdirTemp(os.TempDir(), "*")
	require.NoError(t, err)
	require.NoError(t, InitCmd(ctx, tmpDir, url, connectString))
	buf := new(bytes.Buffer)

	closer := func() {
		cancel()
		require.NoError(t, pg.Stop())
	}
	go func() {
		err := Daemon(ctx, buf, tmpDir, url)
		if err != nil && !errors.Is(err, context.Canceled) {
			require.NoError(t, err)
		}
	}()
	fmt.Println(connectString)

	//wai for api ready
	ticker := time.NewTicker(time.Second)
	tryCount := 0
	for {
		select {
		case <-ticker.C:
			readAll, err := io.ReadAll(buf)
			require.NoError(t, err)
			if strings.Contains(string(readAll), "") {
				return url, closer
			}
			tryCount++
			if tryCount > 5 {
				require.NoError(t, errors.New("timeout to wait api not ready"))
				return "", nil
			}
		case <-ctx.Done():
			closer()
			require.NoError(t, errors.New("context canceled"))
			return "", nil
		}
	}
}
