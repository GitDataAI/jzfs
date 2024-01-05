package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type MergeStatus int

const (
	InitMergeStatus MergeStatus = 1
)

type MergeRequest struct {
	bun.BaseModel `bun:"table:merge_requests"`
	ID            uint64      `bun:"id,pk,autoincrement" json:"id"`
	TargetBranch  uuid.UUID   `bun:"target_branch,unique:ts_repo_ts_branch,type:bytea,notnull" json:"target_branch"`
	SourceBranch  uuid.UUID   `bun:"source_branch,unique:ts_repo_ts_branch,type:bytea,notnull" json:"source_branch"`
	SourceRepoID  uuid.UUID   `bun:"source_repo_id,unique:ts_repo_ts_branch,type:bytea,notnull" json:"source_repo_id"`
	TargetRepoID  uuid.UUID   `bun:"target_repo_id,unique:ts_repo_ts_branch,type:bytea,notnull" json:"target_repo_id"`
	Title         string      `bun:"title,notnull" json:"title"`
	MergeStatus   MergeStatus `bun:"merge_status,notnull" json:"merge_status"`
	Description   *string     `bun:"description" json:"description"`

	AuthorID uuid.UUID `bun:"author_id,type:bytea,notnull" json:"author_id"`

	CreatedAt time.Time `bun:"created_at,notnull" json:"created_at"`
	UpdatedAt time.Time `bun:"updated_at,notnull" json:"updated_at"`
}

type GetMergeRequestParams struct {
	ID *uint64
}

func NewGetMergeRequestParams() *GetMergeRequestParams {
	return &GetMergeRequestParams{}
}

func (gmr *GetMergeRequestParams) SetID(id uint64) *GetMergeRequestParams {
	gmr.ID = &id
	return gmr
}

type DeleteMergeRequestParams struct {
	id *uint64
}

func NewDeleteMergeRequestParams() *DeleteMergeRequestParams {
	return &DeleteMergeRequestParams{}
}

func (gmr *DeleteMergeRequestParams) SetID(id uint64) *DeleteMergeRequestParams {
	gmr.id = &id
	return gmr
}

type UpdateMergeRequestParams struct {
	id          *uint64
	updateTime  time.Time
	title       *string
	description *string
}

func NewUpdateMergeRequestParams(id uint64) *UpdateMergeRequestParams {
	return &UpdateMergeRequestParams{
		id:         &id,
		updateTime: time.Now(),
	}
}

func (u *UpdateMergeRequestParams) SetTitle(title string) *UpdateMergeRequestParams {
	u.title = &title
	return u
}

func (u *UpdateMergeRequestParams) SetDescription(description string) *UpdateMergeRequestParams {
	u.description = &description
	return u
}

type ListMergeRequestParams struct {
	after        *time.Time
	amount       int
	targetRepoID uuid.UUID
}

func NewListMergeRequestParams() *ListMergeRequestParams {
	return &ListMergeRequestParams{}
}

func (lmr *ListMergeRequestParams) SetTargetRepoID(targetRepoID uuid.UUID) *ListMergeRequestParams {
	lmr.targetRepoID = targetRepoID
	return lmr
}

func (lmr *ListMergeRequestParams) SetAfter(after time.Time) *ListMergeRequestParams {
	lmr.after = &after
	return lmr
}

func (lmr *ListMergeRequestParams) SetAmount(amount int) *ListMergeRequestParams {
	lmr.amount = amount
	return lmr
}

type IMergeRequestRepo interface {
	Insert(ctx context.Context, ref *MergeRequest) (*MergeRequest, error)
	Get(ctx context.Context, params *GetMergeRequestParams) (*MergeRequest, error)
	List(ctx context.Context, params *ListMergeRequestParams) ([]MergeRequest, bool, error)
	UpdateByID(ctx context.Context, params *UpdateMergeRequestParams) error
	Delete(ctx context.Context, params *DeleteMergeRequestParams) (int64, error)
}

var _ IMergeRequestRepo = (*MergeRequestRepo)(nil)

type MergeRequestRepo struct {
	db bun.IDB
}

func NewMergeRequestRepo(db bun.IDB) IMergeRequestRepo {
	return &MergeRequestRepo{db: db}
}

func (m MergeRequestRepo) Insert(ctx context.Context, mr *MergeRequest) (*MergeRequest, error) {
	_, err := m.db.NewInsert().Model(mr).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return mr, nil
}

func (m MergeRequestRepo) Get(ctx context.Context, params *GetMergeRequestParams) (*MergeRequest, error) {
	mergeRequest := &MergeRequest{}
	query := m.db.NewSelect().Model(mergeRequest)

	if params.ID != nil {
		query = query.Where("id = ?", params.ID)
	}

	return mergeRequest, query.Limit(1).Scan(ctx)
}

func (m MergeRequestRepo) List(ctx context.Context, params *ListMergeRequestParams) ([]MergeRequest, bool, error) {
	mergeRequest := make([]MergeRequest, 0)
	query := m.db.NewSelect().Model(&mergeRequest)
	if params.targetRepoID != uuid.Nil {
		query = query.Where("target_repo_id = ?", params.targetRepoID)
	}

	query = query.Order("updated_at DESC")
	if params.after != nil {
		query = query.Where("updated_at > ?", *params.after)
	}

	err := query.Limit(params.amount).Scan(ctx)
	return mergeRequest, len(mergeRequest) == params.amount, err
}

func (m MergeRequestRepo) Delete(ctx context.Context, params *DeleteMergeRequestParams) (int64, error) {
	query := m.db.NewDelete().Model((*MergeRequest)(nil))
	if params.id != nil {
		query = query.Where("id = ?", params.id)
	}

	sqlResult, err := query.Exec(ctx)
	if err != nil {
		return 0, err
	}
	affectedRows, err := sqlResult.RowsAffected()
	if err != nil {
		return 0, err
	}
	return affectedRows, err
}

func (m MergeRequestRepo) UpdateByID(ctx context.Context, updateModel *UpdateMergeRequestParams) error {
	updateQuery := m.db.NewUpdate().Model((*MergeRequest)(nil)).Where("id = ?", updateModel.id)
	if updateModel.description != nil {
		updateQuery.Set("description = ?", *updateModel.description)
	}
	if updateModel.title != nil {
		updateQuery.Set("title = ?", *updateModel.title)
	}
	_, err := updateQuery.Exec(ctx)
	return err
}
