package models

import (
	"context"
	"time"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Ref struct {
	bun.BaseModel `bun:"table:ref"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	// RepositoryId which repository this branch belong
	RepositoryID uuid.UUID `bun:"repository_id,type:uuid,notnull"`
	CommitHash   hash.Hash `bun:"commit_hash,type:bytea,notnull"`
	// Path name/path of branch
	Name string `bun:"name,notnull"`
	// Description
	Description string `bun:"description"`
	// CreateId who create this branch
	CreateID uuid.UUID `bun:"create_id,type:uuid,notnull"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type GetRefParams struct {
	ID           uuid.UUID
	RepositoryID uuid.UUID
	Name         *string
}

type IRefRepo interface {
	Insert(ctx context.Context, repo *Ref) (*Ref, error)
	Get(ctx context.Context, id *GetRefParams) (*Ref, error)
}

var _ IRefRepo = (*RefRepo)(nil)

type RefRepo struct {
	db bun.IDB
}

func NewRefRepo(db bun.IDB) IRefRepo {
	return &RefRepo{db: db}
}

func (r RefRepo) Insert(ctx context.Context, ref *Ref) (*Ref, error) {
	_, err := r.db.NewInsert().Model(ref).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return ref, nil
}

func (r RefRepo) Get(ctx context.Context, params *GetRefParams) (*Ref, error) {
	repo := &Ref{}
	query := r.db.NewSelect().Model(repo)

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("create_id = ?", params.RepositoryID)
	}

	if params.Name != nil {
		query = query.Where("name = ?", *params.Name)
	}

	return repo, query.Scan(ctx, repo)

}
