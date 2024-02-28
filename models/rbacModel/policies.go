package rbacModel

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Statements []Statement

type Statement struct {
	Effect   string   `json:"effect"`
	Action   []string `json:"action"`
	Resource Resource `json:"resource"`
}

type Policy struct {
	bun.BaseModel `bun:"table:policies"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	// Name policy name
	Name string `bun:"name,unique,notnull" json:"name"`
	// Actions
	Statements []Statement `bun:"statements,type:jsonb,notnull" json:"statements"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}

type GetPolicyParams struct {
	id uuid.UUID
}

func NewGetPolicyParams() *GetPolicyParams {
	return &GetPolicyParams{}
}

func (gup *GetPolicyParams) SetID(id uuid.UUID) *GetPolicyParams {
	gup.id = id
	return gup
}

type ListPolicyParams struct {
	ids []uuid.UUID
}

func NewListPolicyParams() *ListPolicyParams {
	return &ListPolicyParams{}
}

func (gup *ListPolicyParams) SetIDs(ids ...uuid.UUID) *ListPolicyParams {
	gup.ids = ids
	return gup
}

type IPolicyRepo interface {
	Get(ctx context.Context, params *GetPolicyParams) (*Policy, error)
	List(ctx context.Context, params *ListPolicyParams) ([]*Policy, error)
	Insert(ctx context.Context, policy *Policy) (*Policy, error)
}

var _ IPolicyRepo = (*PolicyRepo)(nil)

type PolicyRepo struct {
	db bun.IDB
}

func NewPolicyRepo(db bun.IDB) IPolicyRepo {
	return &PolicyRepo{db: db}
}

func (p PolicyRepo) Insert(ctx context.Context, policy *Policy) (*Policy, error) {
	_, err := p.db.NewInsert().Model(policy).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return policy, nil
}

func (p PolicyRepo) Get(ctx context.Context, params *GetPolicyParams) (*Policy, error) {
	ug := &Policy{}
	query := p.db.NewSelect().Model(ug)

	if uuid.Nil != params.id {
		query = query.Where("id = ?", params.id)
	}

	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return ug, nil
}

func (p PolicyRepo) List(ctx context.Context, params *ListPolicyParams) ([]*Policy, error) {
	var policies []*Policy
	query := p.db.NewSelect().Model(&policies)

	if len(params.ids) > 0 {
		query = query.Where("id IN (?)", bun.In(params.ids))
	}

	query = query.Order("created_at desc")

	err := query.Scan(ctx)
	if err != nil {
		return nil, err
	}
	return policies, nil
}
