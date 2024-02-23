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
			Model((*models.Branch)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		//wip
		_, err = db.NewCreateTable().
			Model((*models.WorkingInProcess)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		_, err = db.NewCreateTable().
			Model((*models.MergeRequest)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		//commits
		_, err = db.NewCreateTable().
			Model((*models.Commit)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		//tags
		_, err = db.NewCreateTable().
			Model((*models.Tag)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		//filetree
		_, err = db.NewCreateTable().
			Model((*models.FileTree)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		//aksk
		_, err = db.NewCreateTable().
			Model((*models.AkSk)(nil)).
			Exec(ctx)
		if err != nil {
			return err
		}
		return err
	}, nil)
}
