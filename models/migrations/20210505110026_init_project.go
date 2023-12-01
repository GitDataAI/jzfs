package migrations

import (
	"context"

	"github.com/jiaozifs/jiaozifs/models"
	"github.com/uptrace/bun"
)

func init() {
	Migrations.MustRegister(func(ctx context.Context, db *bun.DB) error {
		//common
		_, err := db.Exec(`CREATE EXTENSION IF NOT EXISTS "uuid-ossp";`)
		if err != nil {
			return err
		}

		//user
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
		if err != nil {
			return err
		}
		//repository
		_, err = db.NewCreateTable().
			Model((*models.Repository)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}

		//ref
		_, err = db.NewCreateTable().
			Model((*models.Ref)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		//object
		_, err = db.NewCreateTable().
			Model((*models.Object)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		return err
	}, nil)
}
