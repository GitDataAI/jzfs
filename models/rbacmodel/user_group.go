package rbacmodel

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type UserGroup struct {
	bun.BaseModel `bun:"table:usergroup"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	UserID        uuid.UUID `bun:"user_id,type:uuid,unique:user_group_pk,notnull" json:"user_id"`
	GroupID       uuid.UUID `bun:"group_id,type:uuid,unique:user_group_pk,notnull" json:"group_id"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}

type GetUserGroupParams struct {
	userID  uuid.UUID
	groupID uuid.UUID
}

func NewGetUserGroupParams() *GetUserGroupParams {
	return &GetUserGroupParams{}
}

func (gup *GetUserGroupParams) SetUserID(userID uuid.UUID) *GetUserGroupParams {
	gup.userID = userID
	return gup
}

func (gup *GetUserGroupParams) SetGroupID(groupID uuid.UUID) *GetUserGroupParams {
	gup.groupID = groupID
	return gup
}

type IUserGroupRepo interface {
	Get(ctx context.Context, params *GetUserGroupParams) (*UserGroup, error)
	Insert(ctx context.Context, asSk *UserGroup) (*UserGroup, error)
}

var _ IUserGroupRepo = (*UserGroupRepo)(nil)

type UserGroupRepo struct {
	db bun.IDB
}

func NewUserGroupRepo(db bun.IDB) IUserGroupRepo {
	return &UserGroupRepo{db: db}
}

func (a UserGroupRepo) Insert(ctx context.Context, group *UserGroup) (*UserGroup, error) {
	_, err := a.db.NewInsert().Model(group).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return group, nil
}

func (a UserGroupRepo) Get(ctx context.Context, params *GetUserGroupParams) (*UserGroup, error) {
	ug := &UserGroup{}
	query := a.db.NewSelect().Model(ug)

	if uuid.Nil != params.userID {
		query = query.Where("user_id = ?", params.userID)
	}

	if uuid.Nil != params.groupID {
		query = query.Where("group_id = ?", params.groupID)
	}

	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return ug, nil
}
