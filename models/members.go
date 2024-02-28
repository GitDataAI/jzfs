package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Member struct {
	bun.BaseModel `bun:"table:members"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	UserID        uuid.UUID `bun:"user_id,type:uuid,notnull" json:"user_id"`
	RepoID        uuid.UUID `bun:"repo_id,type:uuid,notnull" json:"repo_id"`
	GroupID       uuid.UUID `bun:"group_id,type:uuid,notnull" json:"group_id"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}

type GetMemberParams struct {
	id     uuid.UUID
	repoID uuid.UUID
	userID uuid.UUID
}

func NewGetMemberParams() *GetMemberParams {
	return &GetMemberParams{}
}

func (p *GetMemberParams) SetID(id uuid.UUID) *GetMemberParams {
	p.id = id
	return p
}

func (p *GetMemberParams) SetRepoID(repo uuid.UUID) *GetMemberParams {
	p.repoID = repo
	return p
}

func (p *GetMemberParams) SetUserID(userID uuid.UUID) *GetMemberParams {
	p.userID = userID
	return p
}

type IMemberRepo interface {
	Insert(ctx context.Context, member *Member) (*Member, error)
	GetMember(ctx context.Context, params *GetMemberParams) (*Member, error)
}

var _ IMemberRepo = (*MemberRepo)(nil)

type MemberRepo struct {
	db bun.IDB
}

func NewMemberRepo(db bun.IDB) IMemberRepo {
	return &MemberRepo{db: db}
}

func (a MemberRepo) Insert(ctx context.Context, member *Member) (*Member, error) {
	_, err := a.db.NewInsert().Model(member).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return member, nil
}

func (a MemberRepo) GetMember(ctx context.Context, params *GetMemberParams) (*Member, error) {
	member := &Member{}
	query := a.db.NewSelect().Model(member)

	if params.id != uuid.Nil {
		query = query.Where("id = ?", params.id)
	}

	if params.repoID != uuid.Nil {
		query = query.Where("repo_id = ?", params.repoID)
	}
	if params.userID != uuid.Nil {
		query = query.Where("user_id = ?", params.userID)
	}
	return member, query.Limit(1).Scan(ctx)
}
