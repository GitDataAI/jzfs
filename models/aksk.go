package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type AkSk struct {
	bun.BaseModel `bun:"table:aksks"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	// UserID ak/sk belong to user id
	UserID uuid.UUID `bun:"user_id,type:uuid,notnull" json:"user_id"`
	// AccessKey
	AccessKey string `bun:"access_key,unique,notnull" json:"access_key"`
	// SecretKey
	SecretKey string `bun:"secret_key,unique,notnull" json:"secret_key"`
	// Description
	Description *string `bun:"description" json:"description,omitempty"`

	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}

type GetAkSkParams struct {
	id        uuid.UUID
	accessKey *string
}

func NewGetAkSkParams() *GetAkSkParams {
	return &GetAkSkParams{}
}

func (gap *GetAkSkParams) SetID(id uuid.UUID) *GetAkSkParams {
	gap.id = id
	return gap
}

func (gap *GetAkSkParams) SetAccessKey(ak string) *GetAkSkParams {
	gap.accessKey = &ak
	return gap
}

type ListAkSkParams struct {
	userID uuid.UUID
	after  *time.Time
	amount int
}

func NewListAkSkParams() *ListAkSkParams {
	return &ListAkSkParams{}
}

func (lap *ListAkSkParams) SetUserID(userID uuid.UUID) *ListAkSkParams {
	lap.userID = userID
	return lap
}

func (lap *ListAkSkParams) SetAfter(after time.Time) *ListAkSkParams {
	lap.after = &after
	return lap
}

func (lap *ListAkSkParams) SetAmount(amount int) *ListAkSkParams {
	lap.amount = amount
	return lap
}

type DeleteAkSkParams struct {
	id        uuid.UUID
	accessKey *string
}

func NewDeleteAkSkParams() *DeleteAkSkParams {
	return &DeleteAkSkParams{}
}

func (dap *DeleteAkSkParams) SetID(id uuid.UUID) *DeleteAkSkParams {
	dap.id = id
	return dap
}

func (dap *DeleteAkSkParams) SetAccessKey(accessKey string) *DeleteAkSkParams {
	dap.accessKey = &accessKey
	return dap
}

type IAkskRepo interface {
	Insert(ctx context.Context, asSk *AkSk) (*AkSk, error)
	Get(ctx context.Context, params *GetAkSkParams) (*AkSk, error)

	List(ctx context.Context, params *ListAkSkParams) ([]*AkSk, bool, error)
	Delete(ctx context.Context, params *DeleteAkSkParams) (int64, error)
}

var _ IAkskRepo = (*AkskRepo)(nil)

type AkskRepo struct {
	db bun.IDB
}

func NewAkskRepo(db bun.IDB) IAkskRepo {
	return &AkskRepo{db: db}
}

func (a AkskRepo) Insert(ctx context.Context, akSk *AkSk) (*AkSk, error) {
	_, err := a.db.NewInsert().Model(akSk).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return akSk, nil
}

func (a AkskRepo) Get(ctx context.Context, params *GetAkSkParams) (*AkSk, error) {
	repo := &AkSk{}
	query := a.db.NewSelect().Model(repo)

	if uuid.Nil != params.id {
		query = query.Where("id = ?", params.id)
	}

	if params.accessKey != nil {
		query = query.Where("access_key = ?", *params.accessKey)
	}

	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return repo, nil
}

func (a AkskRepo) List(ctx context.Context, params *ListAkSkParams) ([]*AkSk, bool, error) {
	var branches []*AkSk
	query := a.db.NewSelect().Model(&branches)

	if uuid.Nil != params.userID {
		query = query.Where("user_Id = ?", params.userID)
	}

	query = query.Order("created_at DESC")
	if params.after != nil {
		query = query.Where("created_at < ?", *params.after)
	}

	err := query.Limit(params.amount).Scan(ctx)
	return branches, len(branches) == params.amount, err
}

func (a AkskRepo) Delete(ctx context.Context, params *DeleteAkSkParams) (int64, error) {
	query := a.db.NewDelete().Model((*AkSk)(nil))

	if uuid.Nil != params.id {
		query = query.Where("id = ?", params.id)
	}

	if params.accessKey != nil {
		query = query.Where("access_key = ?", *params.accessKey)
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
