package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type MergeStatus int

type MergeRequest struct {
	bun.BaseModel `bun:"table:merge_requests"`
	ID            uuid.UUID   `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	TargetBranch  string      `bun:"target_branch,notnull"`
	SourceBranch  string      `bun:"source_branch,notnull"`
	SourceRepoID  uuid.UUID   `bun:"source_repo_id,type:bytea,notnull"`
	TargetRepoID  uuid.UUID   `bun:"target_repo_id,type:bytea,notnull"`
	Title         string      `bun:"title,notnull"`
	MergeStatus   MergeStatus `bun:"merge_status,notnull"`
	Description   *string     `bun:"description"`

	AuthorID uuid.UUID `bun:"author_id,type:bytea,notnull"`

	AssigneeID           uuid.UUID `bun:"assignee_id,type:bytea"`
	MergeUserID          uuid.UUID `bun:"merge_user_id,type:bytea"`
	ApprovalsBeforeMerge int       `bun:"approvals_before_merge"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type GetMergeRequestParams struct {
	ID uuid.UUID
}

func NewGetMergeRequestParams() *GetMergeRequestParams {
	return &GetMergeRequestParams{}
}

func (gmr *GetMergeRequestParams) SetID(id uuid.UUID) *GetMergeRequestParams {
	gmr.ID = id
	return gmr
}

type IMergeRequestRepo interface {
	Insert(ctx context.Context, ref *MergeRequest) (*MergeRequest, error)
	Get(ctx context.Context, params *GetMergeRequestParams) (*MergeRequest, error)
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

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	return mergeRequest, query.Limit(1).Scan(ctx, mergeRequest)
}
