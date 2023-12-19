package models

import (
	"context"
	"time"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Ref struct {
	bun.BaseModel `bun:"table:refs"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	// RepositoryId which repository this branch belong
	RepositoryID uuid.UUID `bun:"repository_id,type:uuid,notnull"`
	CommitHash   hash.Hash `bun:"commit_hash,type:bytea,notnull"`
	// Path name/path of branch
	Name string `bun:"name,notnull"`
	// Description
	Description *string `bun:"description"`
	// CreatorID who create this branch
	CreatorID uuid.UUID `bun:"creator_id,type:uuid,notnull"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type GetRefParams struct {
	ID           uuid.UUID
	RepositoryID uuid.UUID
	Name         *string
}

func NewGetRefParams() *GetRefParams {
	return &GetRefParams{}
}

func (gup *GetRefParams) SetID(id uuid.UUID) *GetRefParams {
	gup.ID = id
	return gup
}

func (gup *GetRefParams) SetRepositoryID(repositoryID uuid.UUID) *GetRefParams {
	gup.RepositoryID = repositoryID
	return gup
}

func (gup *GetRefParams) SetName(name string) *GetRefParams {
	gup.Name = &name
	return gup
}

type DeleteRefParams struct {
	ID           uuid.UUID
	RepositoryID uuid.UUID
	Name         *string
}

func NewDeleteRefParams() *DeleteRefParams {
	return &DeleteRefParams{}
}

func (gup *DeleteRefParams) SetRepositoryID(repositoryID uuid.UUID) *DeleteRefParams {
	gup.RepositoryID = repositoryID
	return gup
}
func (gup *DeleteRefParams) SetID(id uuid.UUID) *DeleteRefParams {
	gup.ID = id
	return gup
}

func (gup *DeleteRefParams) SetName(name string) *DeleteRefParams {
	gup.Name = &name
	return gup
}

type UpdateRefParams struct {
	bun.BaseModel `bun:"table:refs"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	CommitHash    hash.Hash `bun:"commit_hash,type:bytea,notnull"`
}

func NewUpdateRefParams(id uuid.UUID) *UpdateRefParams {
	return &UpdateRefParams{ID: id}
}

func (up *UpdateRefParams) SetCommitHash(commitHash hash.Hash) *UpdateRefParams {
	up.CommitHash = commitHash
	return up
}

type ListRefParams struct {
	RepositoryID uuid.UUID
}

func NewListRefParams() *ListRefParams {
	return &ListRefParams{}
}

func (gup *ListRefParams) SetRepositoryID(repositoryID uuid.UUID) *ListRefParams {
	gup.RepositoryID = repositoryID
	return gup
}

type IRefRepo interface {
	Insert(ctx context.Context, repo *Ref) (*Ref, error)
	UpdateByID(ctx context.Context, params *UpdateRefParams) error
	Get(ctx context.Context, id *GetRefParams) (*Ref, error)

	List(ctx context.Context, params *ListRefParams) ([]*Ref, error)
	Delete(ctx context.Context, params *DeleteRefParams) error
}

var _ IRefRepo = (*RefRepo)(nil)

type RefRepo struct {
	db bun.IDB
}

func NewRefRepo(db bun.IDB) IRefRepo {
	return &RefRepo{db: db}
}

func (r RefRepo) Insert(ctx context.Context, ref *Ref) (*Ref, error) {
	_, err := r.db.NewInsert().Model(ref).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return ref, nil
}

func (r RefRepo) Get(ctx context.Context, params *GetRefParams) (*Ref, error) {
	repo := &Ref{}
	query := r.db.NewSelect().Model(repo)

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	if params.Name != nil {
		query = query.Where("name = ?", *params.Name)
	}

	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return repo, nil
}

func (r RefRepo) List(ctx context.Context, params *ListRefParams) ([]*Ref, error) {
	var refs []*Ref
	query := r.db.NewSelect().Model(&refs)

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	err := query.Scan(ctx)
	if err != nil {
		return nil, err
	}
	return refs, nil
}

func (r RefRepo) Delete(ctx context.Context, params *DeleteRefParams) error {
	query := r.db.NewDelete().Model((*Ref)(nil))

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	if params.Name != nil {
		query = query.Where("name = ?", *params.Name)
	}

	_, err := query.Exec(ctx)
	return err
}

func (r RefRepo) UpdateByID(ctx context.Context, updateModel *UpdateRefParams) error {
	_, err := r.db.NewUpdate().Model(updateModel).WherePK().Exec(ctx)
	return err
}
