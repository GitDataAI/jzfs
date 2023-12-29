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
	bun.BaseModel `bun:"table:wips"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	CurrentTree   hash.Hash `bun:"current_tree,type:bytea,notnull" json:"current_tree"`
	BaseCommit    hash.Hash `bun:"base_commit,type:bytea,notnull" json:"base_commit"`
	RepositoryID  uuid.UUID `bun:"repository_id,unique:creator_id_repository_id_ref_id_unique,type:uuid,notnull" json:"repository_id"`
	RefID         uuid.UUID `bun:"ref_id,unique:creator_id_repository_id_ref_id_unique,type:uuid,notnull" json:"ref_id"`
	State         WipState  `bun:"state,notnull" json:"state"`
	CreatorID     uuid.UUID `bun:"creator_id,unique:creator_id_repository_id_ref_id_unique,type:uuid,notnull" json:"creator_id"`
	CreatedAt     time.Time `bun:"created_at" json:"created_at"`
	UpdatedAt     time.Time `bun:"updated_at" json:"updated_at"`
}

type GetWipParams struct {
	ID           uuid.UUID
	CreatorID    uuid.UUID
	RepositoryID uuid.UUID
	RefID        uuid.UUID
}

func NewGetWipParams() *GetWipParams {
	return &GetWipParams{}
}

func (gwp *GetWipParams) SetID(id uuid.UUID) *GetWipParams {
	gwp.ID = id
	return gwp
}

func (gwp *GetWipParams) SetCreatorID(creatorID uuid.UUID) *GetWipParams {
	gwp.CreatorID = creatorID
	return gwp
}

func (gwp *GetWipParams) SetRepositoryID(repositoryID uuid.UUID) *GetWipParams {
	gwp.RepositoryID = repositoryID
	return gwp
}

func (gwp *GetWipParams) SetRefID(refID uuid.UUID) *GetWipParams {
	gwp.RefID = refID
	return gwp
}

type ListWipParams struct {
	CreatorID    uuid.UUID
	RepositoryID uuid.UUID
	RefID        uuid.UUID
}

func NewListWipParams() *ListWipParams {
	return &ListWipParams{}
}
func (lwp *ListWipParams) SetCreatorID(creatorID uuid.UUID) *ListWipParams {
	lwp.CreatorID = creatorID
	return lwp
}

func (lwp *ListWipParams) SetRepositoryID(repositoryID uuid.UUID) *ListWipParams {
	lwp.RepositoryID = repositoryID
	return lwp
}

func (lwp *ListWipParams) SetRefID(refID uuid.UUID) *ListWipParams {
	lwp.RefID = refID
	return lwp
}

type DeleteWipParams struct {
	ID           uuid.UUID
	CreatorID    uuid.UUID
	RepositoryID uuid.UUID
	RefID        uuid.UUID
}

func NewDeleteWipParams() *DeleteWipParams {
	return &DeleteWipParams{}
}

func (dwp *DeleteWipParams) SetID(id uuid.UUID) *DeleteWipParams {
	dwp.ID = id
	return dwp
}

func (dwp *DeleteWipParams) SetCreatorID(creatorID uuid.UUID) *DeleteWipParams {
	dwp.CreatorID = creatorID
	return dwp
}

func (dwp *DeleteWipParams) SetRepositoryID(repositoryID uuid.UUID) *DeleteWipParams {
	dwp.RepositoryID = repositoryID
	return dwp
}

func (dwp *DeleteWipParams) SetRefID(refID uuid.UUID) *DeleteWipParams {
	dwp.RefID = refID
	return dwp
}

type UpdateWipParams struct {
	bun.BaseModel `bun:"table:wips"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	CurrentTree   hash.Hash `bun:"current_tree,type:bytea,notnull"`
	BaseCommit    hash.Hash `bun:"base_commit,type:bytea,notnull"`
	State         WipState  `bun:"state,notnull"`
	UpdatedAt     time.Time `bun:"updated_at"`
}

func NewUpdateWipParams(id uuid.UUID) *UpdateWipParams {
	return &UpdateWipParams{ID: id, UpdatedAt: time.Now()}
}

func (up *UpdateWipParams) SetCurrentTree(currentTree hash.Hash) *UpdateWipParams {
	up.CurrentTree = currentTree
	return up
}

func (up *UpdateWipParams) SetBaseCommit(commitHash hash.Hash) *UpdateWipParams {
	up.BaseCommit = commitHash
	return up
}

func (up *UpdateWipParams) SetState(state WipState) *UpdateWipParams {
	up.State = state
	return up
}

type IWipRepo interface {
	Insert(ctx context.Context, repo *WorkingInProcess) (*WorkingInProcess, error)
	Get(ctx context.Context, params *GetWipParams) (*WorkingInProcess, error)
	List(ctx context.Context, params *ListWipParams) ([]*WorkingInProcess, error)
	Delete(ctx context.Context, params *DeleteWipParams) (int64, error)
	UpdateByID(ctx context.Context, params *UpdateWipParams) error
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
func (s *WipRepo) Get(ctx context.Context, params *GetWipParams) (*WorkingInProcess, error) {
	wips := &WorkingInProcess{}
	query := s.db.NewSelect().Model(wips)

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if uuid.Nil != params.CreatorID {
		query = query.Where("creator_id = ?", params.CreatorID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	if uuid.Nil != params.RefID {
		query = query.Where("ref_id = ?", params.RefID)
	}

	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return wips, nil
}

func (s *WipRepo) List(ctx context.Context, params *ListWipParams) ([]*WorkingInProcess, error) {
	var resp []*WorkingInProcess
	query := s.db.NewSelect().Model(&resp)

	if uuid.Nil != params.CreatorID {
		query = query.Where("creator_id = ?", params.CreatorID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	if uuid.Nil != params.RefID {
		query = query.Where("ref_id = ?", params.RefID)
	}

	err := query.Scan(ctx)
	if err != nil {
		return nil, err
	}
	return resp, nil
}

// Delete remove wip in table by id
func (s *WipRepo) Delete(ctx context.Context, params *DeleteWipParams) (int64, error) {
	query := s.db.NewDelete().Model((*WorkingInProcess)(nil))

	if uuid.Nil != params.CreatorID {
		query = query.Where("creator_id = ?", params.CreatorID)
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
	r, err := query.Exec(ctx)
	if err != nil {
		return 0, err
	}
	row, err := r.RowsAffected()
	if err != nil {
		return 0, err
	}
	return row, nil
}

func (s *WipRepo) UpdateByID(ctx context.Context, updateModel *UpdateWipParams) error {
	_, err := s.db.NewUpdate().Model(updateModel).WherePK().OmitZero().Exec(ctx)
	return err
}
