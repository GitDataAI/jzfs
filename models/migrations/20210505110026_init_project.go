package migrations

import (
	"context"

	"github.com/jiaozifs/jiaozifs/models"
	"github.com/uptrace/bun"
)

func init() {
	Migrations.MustRegister(func(ctx context.Context, db *bun.DB) error {
		_, err := db.Exec(`CREATE EXTENSION IF NOT EXISTS "uuid-ossp";`)
		if err != nil {
			return err
		}

		_, err = db.NewCreateTable().
			Model((*models.User)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}

		_, err = db.NewCreateIndex().
			Model((*models.User)(nil)).
			Index("name_idx").
			Column("name").
			Exec(ctx)
		return err
	}, nil)
}
