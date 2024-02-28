package rbacModel

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Group struct {
	bun.BaseModel `bun:"table:groups"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	// Name policy name
	Name string `bun:"name,unique,notnull" json:"secret_key"`
	// Policies
	Policies []uuid.UUID `bun:"policies,type:jsonb,notnull" json:"policies"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}

type GetGroupParams struct {
	id   uuid.UUID
	name *string
}

func NewGetGroupParams() *GetGroupParams {
	return &GetGroupParams{}
}

func (gup *GetGroupParams) SetID(id uuid.UUID) *GetGroupParams {
	gup.id = id
	return gup
}

func (gup *GetGroupParams) SetName(name string) *GetGroupParams {
	gup.name = &name
	return gup
}

type IGroupRepo interface {
	GetGroupByUserID(ctx context.Context, userID uuid.UUID) (*Group, error)
	Get(ctx context.Context, params *GetGroupParams) (*Group, error)
	Insert(ctx context.Context, asSk *Group) (*Group, error)
}

var _ IGroupRepo = (*GroupRepo)(nil)

type GroupRepo struct {
	db bun.IDB
}

func NewGroupRepo(db bun.IDB) IGroupRepo {
	return &GroupRepo{db: db}
}

func (a GroupRepo) GetGroupByUserID(ctx context.Context, userID uuid.UUID) (*Group, error) {
	ug := &Group{}
	query := a.db.NewSelect().Model(ug)

	query = query.Join(`RIGHT JOIN usergroup ON usergroup.group_id = "group".id`).Where("usergroup.user_id = ?", userID)
	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return ug, nil
}

func (a GroupRepo) Get(ctx context.Context, params *GetGroupParams) (*Group, error) {
	ug := &Group{}
	query := a.db.NewSelect().Model(ug)

	if uuid.Nil != params.id {
		query = query.Where("id = ?", params.id)
	}
	if params.name != nil {
		query = query.Where("name = ?", params.name)
	}
	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return ug, nil
}

func (a GroupRepo) Insert(ctx context.Context, group *Group) (*Group, error) {
	_, err := a.db.NewInsert().Model(group).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return group, nil
}
