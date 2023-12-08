package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/utils/hash"
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

type WorkingInProcess struct {
	bun.BaseModel `bun:"table:wip"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	CurrentTree   hash.Hash `bun:"current_tree,type:bytea,notnull"`
	ParentTree    hash.Hash `bun:"parent_tree,type:bytea,notnull"`
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

type IWipRepo interface {
	Insert(ctx context.Context, repo *WorkingInProcess) (*WorkingInProcess, error)
	Get(ctx context.Context, params *GetStashParam) (*WorkingInProcess, error)
	UpdateCurrentHash(ctx context.Context, id uuid.UUID, newTreeHash hash.Hash) error
}

var _ IWipRepo = (*WipRepo)(nil)

type WipRepo struct {
	db *bun.DB
}

func NewWipRepo(db *bun.DB) IWipRepo {
	return &WipRepo{db}
}

func (s *WipRepo) Insert(ctx context.Context, repo *WorkingInProcess) (*WorkingInProcess, error) {
	_, err := s.db.NewInsert().Model(repo).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return repo, nil
}

func (s *WipRepo) Get(ctx context.Context, params *GetStashParam) (*WorkingInProcess, error) {
	repo := &WorkingInProcess{}
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

func (s *WipRepo) UpdateCurrentHash(ctx context.Context, id uuid.UUID, newTreeHash hash.Hash) error {
	repo := &WorkingInProcess{
		CurrentTree: newTreeHash,
	}
	_, err := s.db.NewUpdate().Model(repo).OmitZero().Column("current_tree").
		Where("id = ?", id).
		Exec(ctx)
	return err
}
