package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Repository struct {
	bun.BaseModel    `bun:"table:repository"`
	ID               uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Name             string    `bun:"name,notnull"`
	StorageNamespace string    `bun:"storage_namespace,notnull"`
	Description      string    `bun:"description"`
	HEAD             uuid.UUID `bun:"head,type:uuid,notnull"`
	CreateID         uuid.UUID `bun:"create_id,type:uuid,notnull"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type GetRepoParams struct {
	Id       uuid.UUID
	CreateID uuid.UUID
	Name     *string
}

type IRepositoryRepo interface {
	Insert(ctx context.Context, repo *Repository) (*Repository, error)
	Get(ctx context.Context, params *GetRepoParams) (*Repository, error)
}

var _ IRepositoryRepo = (*RepositoryRepo)(nil)

type RepositoryRepo struct {
	db *bun.DB
}

func NewRepositoryRepo(db *bun.DB) IRepositoryRepo {
	return &RepositoryRepo{db}
}

func (r *RepositoryRepo) Insert(ctx context.Context, repo *Repository) (*Repository, error) {
	_, err := r.db.NewInsert().Model(repo).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return repo, nil
}

func (r *RepositoryRepo) Get(ctx context.Context, params *GetRepoParams) (*Repository, error) {
	repo := &Repository{}
	query := r.db.NewSelect().Model(repo)

	if uuid.Nil != params.Id {
		query = query.Where("id = ?", params.Id)
	}

	if uuid.Nil != params.CreateID {
		query = query.Where("create_id = ?", params.CreateID)
	}

	if params.Name != nil {
		query = query.Where("name = ?", *params.Name)
	}

	return repo, query.Scan(ctx, repo)
}
