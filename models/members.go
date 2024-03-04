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
	UserID        uuid.UUID `bun:"user_id,type:uuid,unique:user_repo_pk,notnull" json:"user_id"`
	RepoID        uuid.UUID `bun:"repo_id,type:uuid,unique:user_repo_pk,notnull" json:"repo_id"`
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

type DeleteMemberParams struct {
	repoID uuid.UUID
	userID uuid.UUID
}

func NewDeleteMemberParams() *DeleteMemberParams {
	return &DeleteMemberParams{}
}

func (p *DeleteMemberParams) SetRepoID(repo uuid.UUID) *DeleteMemberParams {
	p.repoID = repo
	return p
}

func (p *DeleteMemberParams) SetUserID(userID uuid.UUID) *DeleteMemberParams {
	p.userID = userID
	return p
}

type UpdateMemberParams struct {
	filter struct {
		repoID uuid.UUID
		userID uuid.UUID
	}

	update struct {
		GroupID uuid.UUID
	}

	updateTime time.Time
}

func NewUpdateMemberParams() *UpdateMemberParams {
	return &UpdateMemberParams{
		updateTime: time.Now(),
	}
}

func (p *UpdateMemberParams) SetFilterRepoID(repo uuid.UUID) *UpdateMemberParams {
	p.filter.repoID = repo
	return p
}

func (p *UpdateMemberParams) SetFilterUserID(userID uuid.UUID) *UpdateMemberParams {
	p.filter.userID = userID
	return p
}

func (p *UpdateMemberParams) SetUpdateGroupID(groupID uuid.UUID) *UpdateMemberParams {
	p.update.GroupID = groupID
	return p
}

type ListMembersParams struct {
	repoID uuid.UUID
}

func NewListMembersParams() *ListMembersParams {
	return &ListMembersParams{}
}

func (p *ListMembersParams) SetRepoID(repo uuid.UUID) *ListMembersParams {
	p.repoID = repo
	return p
}

type IMemberRepo interface {
	Insert(ctx context.Context, member *Member) (*Member, error)
	GetMember(ctx context.Context, params *GetMemberParams) (*Member, error)
	ListMember(ctx context.Context, params *ListMembersParams) ([]*Member, error)
	DeleteMember(ctx context.Context, params *DeleteMemberParams) (int64, error)
	UpdateMember(ctx context.Context, params *UpdateMemberParams) error
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
func (a MemberRepo) ListMember(ctx context.Context, params *ListMembersParams) ([]*Member, error) {
	var members []*Member
	query := a.db.NewSelect().Model(&members)

	if uuid.Nil != params.repoID {
		query = query.Where("repo_id = ?", params.repoID)
	}

	query = query.Order("created_at DESC")

	err := query.Scan(ctx)
	if err != nil {
		return nil, err
	}
	return members, nil
}

func (a MemberRepo) UpdateMember(ctx context.Context, params *UpdateMemberParams) error {
	updateQuery := a.db.NewUpdate().Model((*Member)(nil))
	if params.filter.repoID != uuid.Nil {
		updateQuery.Where("repo_id = ?", params.filter.repoID)
	}
	if params.filter.userID != uuid.Nil {
		updateQuery.Where("user_id = ?", params.filter.userID)
	}

	if params.update.GroupID != uuid.Nil {
		updateQuery.Set("group_id = ?", params.update.GroupID)
	}

	updateQuery.Set("updated_at = ?", params.updateTime)

	_, err := updateQuery.Exec(ctx)
	return err
}

func (a MemberRepo) DeleteMember(ctx context.Context, params *DeleteMemberParams) (int64, error) {
	query := a.db.NewDelete().Model((*Member)(nil))

	if uuid.Nil != params.repoID {
		query = query.Where("repo_id = ?", params.repoID)
	}

	if uuid.Nil != params.userID {
		query = query.Where("user_id = ?", params.userID)
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
