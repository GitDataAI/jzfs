package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

// Action values represent the kind of things a Change can represent:
// insertion, deletions or modifications of files.
type Action int

// The set of possible actions in a change.
const (
	_ Action = iota
	Insert
	Delete
	Modify
)

type Stash struct {
	bun.BaseModel `bun:"table:stash"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	CurrentTreeID uuid.UUID `bun:"current_tree_id,type:uuid,notnull"`
	ParentTreeID  uuid.UUID `bun:"parent_tree_id,type:uuid,notnull"`
	RepositoryID  uuid.UUID `bun:"repository_id,type:uuid,notnull"`
	CreateID      uuid.UUID `bun:"create_id,type:uuid,notnull"`
	CreatedAt     time.Time `bun:"created_at"`
	UpdatedAt     time.Time `bun:"updated_at"`
}

type GetStashParam struct {
	ID           uuid.UUID
	CreateID     uuid.UUID
	RepositoryID uuid.UUID
}

type IStashRepo interface {
	Insert(ctx context.Context, repo *Stash) (*Stash, error)
	Get(ctx context.Context, params *GetStashParam) (*Stash, error)
}

var _ IStashRepo = (*StashRepo)(nil)

type StashRepo struct {
	db *bun.DB
}

func NewStashRepo(db *bun.DB) IStashRepo {
	return &StashRepo{db}
}

func (s *StashRepo) Insert(ctx context.Context, repo *Stash) (*Stash, error) {
	_, err := s.db.NewInsert().Model(repo).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return repo, nil
}

func (s *StashRepo) Get(ctx context.Context, params *GetStashParam) (*Stash, error) {
	repo := &Stash{}
	query := s.db.NewSelect().Model(repo)

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if uuid.Nil != params.CreateID {
		query = query.Where("create_id = ?", params.CreateID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}
	return repo, query.Scan(ctx, repo)
}
