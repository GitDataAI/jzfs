package migrations

import (
	"context"
	"github.com/uptrace/bun"
	"github.com/uptrace/bun/migrate"
)

var Migrations = migrate.NewMigrations()

func init() {
	if err := Migrations.DiscoverCaller(); err != nil {
		panic(err)
	}
}

func MigrateDatabase(ctx context.Context, sqlDB *bun.DB) error {
	migrator := migrate.NewMigrator(sqlDB, Migrations)
	err := migrator.Init(ctx)
	if err != nil {
		return err
	}

	_, err = migrator.Migrate(ctx)
	return err
}
