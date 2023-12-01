package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Ref struct {
	bun.BaseModel `bun:"table:ref"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	// RepositoryId which repository this branch belong
	RepositoryID uuid.UUID `bun:"repository_id,type:uuid,notnull"`
	CommitHash   uuid.UUID `bun:"commit_hash,type:uuid,notnull"`
	// Path name/path of branch
	Path string `bun:"path,notnull"`
	// Description
	Description string `bun:"description"`
	// CreateId who create this branch
	CreateID uuid.UUID `bun:"create_id,type:uuid,notnull"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type IRefRepo interface {
	Insert(ctx context.Context, repo *Ref) (*Ref, error)
	Get(ctx context.Context, id uuid.UUID) (*Ref, error)
}

var _ IRefRepo = (*RefRepo)(nil)

type RefRepo struct {
	*bun.DB
}

func NewRefRepo(db *bun.DB) IRefRepo {
	return &RefRepo{db}
}

func (r RefRepo) Insert(ctx context.Context, ref *Ref) (*Ref, error) {
	_, err := r.DB.NewInsert().Model(ref).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return ref, nil
}

func (r RefRepo) Get(ctx context.Context, id uuid.UUID) (*Ref, error) {
	ref := &Ref{}
	return ref, r.DB.NewSelect().Model(ref).Where("id = ?", id).Scan(ctx)
}
