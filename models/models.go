package models

import (
	"context"
	"database/sql"

	"github.com/uptrace/bun/extra/bundebug"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/uptrace/bun"
	"github.com/uptrace/bun/dialect/pgdialect"
	"github.com/uptrace/bun/driver/pgdriver"
	"go.uber.org/fx"
)

func SetupDatabase(ctx context.Context, lc fx.Lifecycle, dbConfig *config.DatabaseConfig) (*bun.DB, error) {
	sqlDB := sql.OpenDB(pgdriver.NewConnector(pgdriver.WithDSN(dbConfig.Connection)))
	_, err := sqlDB.Conn(ctx)
	if err != nil {
		return nil, err
	}

	bunDB := bun.NewDB(sqlDB, pgdialect.New(), bun.WithDiscardUnknownColumns())

	if dbConfig.Debug {
		bunDB.AddQueryHook(bundebug.NewQueryHook(bundebug.WithVerbose(true)))
	}

	lc.Append(fx.Hook{
		OnStop: func(ctx context.Context) error {
			return bunDB.Close()
		},
	})

	return bunDB, nil
}
