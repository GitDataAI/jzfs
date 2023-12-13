package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Repository struct {
	bun.BaseModel `bun:"table:repositories"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Name          string    `bun:"name,notnull"`
	Description   *string   `bun:"description"`
	HEAD          string    `bun:"head,notnull"`
	CreatorID     uuid.UUID `bun:"creator_id,type:uuid,notnull"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type GetRepoParams struct {
	ID        uuid.UUID
	CreatorID uuid.UUID
	Name      *string
}

func NewGetRepoParams() *GetRepoParams {
	return &GetRepoParams{}
}

func (gup *GetRepoParams) SetID(id uuid.UUID) *GetRepoParams {
	gup.ID = id
	return gup
}

func (gup *GetRepoParams) SetCreatorID(creatorID uuid.UUID) *GetRepoParams {
	gup.CreatorID = creatorID
	return gup
}

func (gup *GetRepoParams) SetName(name string) *GetRepoParams {
	gup.Name = &name
	return gup
}

type ListRepoParams struct {
	ID        uuid.UUID
	CreatorID uuid.UUID
}

func NewListRepoParam() *ListRepoParams {
	return &ListRepoParams{}
}

func (lrp *ListRepoParams) SetID(id uuid.UUID) *ListRepoParams {
	lrp.ID = id
	return lrp
}

func (lrp *ListRepoParams) SetCreatorID(creatorID uuid.UUID) *ListRepoParams {
	lrp.CreatorID = creatorID
	return lrp
}

type DeleteRepoParams struct {
	ID uuid.UUID
}

func NewDeleteRepoParams() *DeleteRepoParams {
	return &DeleteRepoParams{}
}

func (drp *DeleteRepoParams) SetID(id uuid.UUID) *DeleteRepoParams {
	drp.ID = id
	return drp
}

type UpdateRepoParams struct {
	bun.BaseModel `bun:"table:repositories"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Description   *string   `bun:"description"`
}

func NewUpdateRepoParams(id uuid.UUID) *UpdateRepoParams {
	return &UpdateRepoParams{
		ID: id,
	}
}

func (up *UpdateRepoParams) SetDescription(description string) *UpdateRepoParams {
	up.Description = &description
	return up
}

type IRepositoryRepo interface {
	Insert(ctx context.Context, repo *Repository) (*Repository, error)
	Get(ctx context.Context, params *GetRepoParams) (*Repository, error)
	List(ctx context.Context, params *ListRepoParams) ([]*Repository, error)
	Delete(ctx context.Context, params *DeleteRepoParams) error
	UpdateByID(ctx context.Context, updateModel *UpdateRepoParams) error
}

var _ IRepositoryRepo = (*RepositoryRepo)(nil)

type RepositoryRepo struct {
	db bun.IDB
}

func NewRepositoryRepo(db bun.IDB) IRepositoryRepo {
	return &RepositoryRepo{db: db}
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

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if uuid.Nil != params.CreatorID {
		query = query.Where("creator_id = ?", params.CreatorID)
	}

	if params.Name != nil {
		query = query.Where("name = ?", *params.Name)
	}

	return repo, query.Limit(1).Scan(ctx, repo)
}

func (r *RepositoryRepo) List(ctx context.Context, params *ListRepoParams) ([]*Repository, error) {
	repos := []*Repository{}
	query := r.db.NewSelect().Model((*Repository)(nil))

	if uuid.Nil != params.CreatorID {
		query = query.Where("creator_id = ?", params.CreatorID)
	}

	return repos, query.Scan(ctx, &repos)
}

func (r *RepositoryRepo) Delete(ctx context.Context, params *DeleteRepoParams) error {
	query := r.db.NewDelete().Model((*Repository)(nil))
	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	_, err := query.Exec(ctx)
	return err
}

func (r *RepositoryRepo) UpdateByID(ctx context.Context, updateModel *UpdateRepoParams) error {
	_, err := r.db.NewUpdate().Model(updateModel).WherePK().Exec(ctx)
	return err
}
