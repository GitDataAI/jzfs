package models

import (
	"context"
	"database/sql"

	"github.com/uptrace/bun"
)

type TxOption func(*sql.TxOptions)

func IsolationLevelOption(level sql.IsolationLevel) TxOption {
	return func(opts *sql.TxOptions) {
		opts.Isolation = level
	}
}

type IRepo interface {
	Transaction(ctx context.Context, fn func(repo IRepo) error, opts ...TxOption) error
	UserRepo() IUserRepo
	ObjectRepo() IObjectRepo
	RefRepo() IRefRepo
	RepositoryRepo() IRepositoryRepo
	WipRepo() IWipRepo
}

type PgRepo struct {
	db bun.IDB
}

func NewRepo(db bun.IDB) IRepo {
	return &PgRepo{
		db: db,
	}
}

func (repo *PgRepo) Transaction(ctx context.Context, fn func(repo IRepo) error, opts ...TxOption) error {
	sqlOpt := &sql.TxOptions{}
	for _, opt := range opts {
		opt(sqlOpt)
	}
	return repo.db.RunInTx(ctx, sqlOpt, func(ctx context.Context, tx bun.Tx) error {
		return fn(NewRepo(tx))
	})
}

func (repo *PgRepo) UserRepo() IUserRepo {
	return NewUserRepo(repo.db)
}

func (repo *PgRepo) ObjectRepo() IObjectRepo {
	return NewObjectRepo(repo.db)
}

func (repo *PgRepo) RefRepo() IRefRepo {
	return NewRefRepo(repo.db)
}

func (repo *PgRepo) RepositoryRepo() IRepositoryRepo {
	return NewRepositoryRepo(repo.db)
}

func (repo *PgRepo) WipRepo() IWipRepo {
	return NewWipRepo(repo.db)
}
