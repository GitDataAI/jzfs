package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/uptrace/bun"
)

type WipState int

const (
	Init WipState = iota
	Completed
)

type WorkingInProcess struct {
	bun.BaseModel `bun:"table:wip"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Name          string    `bun:"name,notnull"`
	CurrentTree   hash.Hash `bun:"current_tree,type:bytea,notnull"`
	BaseTree      hash.Hash `bun:"base_tree,type:bytea,notnull"`
	RepositoryID  uuid.UUID `bun:"repository_id,type:uuid,notnull"`
	RefID         uuid.UUID `bun:"ref_id,type:uuid,notnull"`
	State         WipState  `bun:"state"`
	CreateID      uuid.UUID `bun:"create_id,type:uuid,notnull"`
	CreatedAt     time.Time `bun:"created_at"`
	UpdatedAt     time.Time `bun:"updated_at"`
}

type GetWipParam struct {
	ID           uuid.UUID
	CreateID     uuid.UUID
	RepositoryID uuid.UUID
	RefID        uuid.UUID
}

type ListWipParam struct {
	CreateID     uuid.UUID
	RepositoryID uuid.UUID
	RefID        uuid.UUID
}

type DeleteWipParam struct {
	ID           uuid.UUID
	CreateID     uuid.UUID
	RepositoryID uuid.UUID
	RefID        uuid.UUID
}
type IWipRepo interface {
	Insert(ctx context.Context, repo *WorkingInProcess) (*WorkingInProcess, error)
	Get(ctx context.Context, params *GetWipParam) (*WorkingInProcess, error)
	List(ctx context.Context, params *ListWipParam) ([]*WorkingInProcess, error)
	Delete(ctx context.Context, params *DeleteWipParam) error
	UpdateCurrentHash(ctx context.Context, id uuid.UUID, newTreeHash hash.Hash) error
	UpdateBaseHash(ctx context.Context, id uuid.UUID, treeHash hash.Hash) error
	UpdateState(ctx context.Context, id uuid.UUID, state WipState) error
}

var _ IWipRepo = (*WipRepo)(nil)

type WipRepo struct {
	db bun.IDB
}

func NewWipRepo(db bun.IDB) IWipRepo {
	return &WipRepo{db: db}
}

func (s *WipRepo) Insert(ctx context.Context, repo *WorkingInProcess) (*WorkingInProcess, error) {
	_, err := s.db.NewInsert().Model(repo).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return repo, nil
}

// Get wip by a group of conditions
func (s *WipRepo) Get(ctx context.Context, params *GetWipParam) (*WorkingInProcess, error) {
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

	if uuid.Nil != params.RefID {
		query = query.Where("ref_id = ?", params.RefID)
	}

	return repo, query.Limit(1).Scan(ctx, repo)
}

func (s *WipRepo) List(ctx context.Context, params *ListWipParam) ([]*WorkingInProcess, error) {
	var resp []*WorkingInProcess
	query := s.db.NewSelect().Model((*WorkingInProcess)(nil))

	if uuid.Nil != params.CreateID {
		query = query.Where("create_id = ?", params.CreateID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	if uuid.Nil != params.RefID {
		query = query.Where("ref_id = ?", params.RefID)
	}

	return resp, query.Scan(ctx, &resp)
}

// UpdateCurrentHash update current hash
func (s *WipRepo) UpdateCurrentHash(ctx context.Context, id uuid.UUID, newTreeHash hash.Hash) error {
	wip := &WorkingInProcess{
		CurrentTree: newTreeHash,
	}
	_, err := s.db.NewUpdate().Model(wip).OmitZero().Column("current_tree").
		Where("id = ?", id).
		Exec(ctx)
	return err
}

// UpdateBaseHash update base hash
func (s *WipRepo) UpdateBaseHash(ctx context.Context, id uuid.UUID, treeHash hash.Hash) error {
	wip := &WorkingInProcess{
		BaseTree: treeHash,
	}
	_, err := s.db.NewUpdate().Model(wip).OmitZero().Column("base_tree").
		Where("id = ?", id).
		Exec(ctx)
	return err
}

// UpdateState update wip state
func (s *WipRepo) UpdateState(ctx context.Context, id uuid.UUID, state WipState) error {
	wip := &WorkingInProcess{
		State: state,
	}

	_, err := s.db.NewUpdate().Model(wip).OmitZero().Column("state").
		Where("id = ?", id).
		Exec(ctx)
	return err
}

// Delete remove wip in table by id
func (s *WipRepo) Delete(ctx context.Context, params *DeleteWipParam) error {
	query := s.db.NewDelete().Model((*WorkingInProcess)(nil))

	if uuid.Nil != params.CreateID {
		query = query.Where("create_id = ?", params.CreateID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	if uuid.Nil != params.RefID {
		query = query.Where("ref_id = ?", params.RefID)
	}

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}
	_, err := query.Exec(ctx)
	return err
}
