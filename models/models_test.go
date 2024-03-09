package models_test

import (
	"context"
	"fmt"
	"testing"

	embeddedpostgres "github.com/fergusstrange/embedded-postgres"
	"github.com/GitDataAI/jiaozifs/config"
	"github.com/GitDataAI/jiaozifs/models"
	"github.com/phayes/freeport"
	"github.com/stretchr/testify/require"
	"go.uber.org/fx/fxtest"
)

var testConnTmpl = "postgres://postgres:postgres@localhost:%d/jiaozifs?sslmode=disable"

func TestSetupDatabase(t *testing.T) {
	ctx := context.Background()
	port, err := freeport.GetFreePort()
	require.NoError(t, err)

	postgres := embeddedpostgres.NewDatabase(embeddedpostgres.DefaultConfig().Port(uint32(port)).Database("jiaozifs"))
	err = postgres.Start()
	require.NoError(t, err)
	defer postgres.Stop() //nolint

	lc := fxtest.NewLifecycle(t)
	db, err := models.SetupDatabase(ctx, lc, &config.DatabaseConfig{Connection: fmt.Sprintf(testConnTmpl, port)})
	require.NoError(t, err)
	lc.RequireStop()
	require.NoError(t, db.PingContext(ctx))
}
