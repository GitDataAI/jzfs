package models

import (
	"context"
	"time"

	"github.com/GitDataAI/jiaozifs/utils/hash"
	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type Tag struct {
	bun.BaseModel `bun:"table:tags"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	RepositoryID  uuid.UUID `bun:"repository_id,pk,type:uuid,unique:repo_id_name,notnull" json:"repository_id"`
	//////********commit********////////
	// Name of the tag.
	Name string `bun:"name,unique:repo_id_name," json:"name"`
	// Creator is the one who created the tag.
	CreatorID uuid.UUID `bun:"tagger,type:uuid" json:"tagger"`
	// Target is the hash of the target object.
	Target hash.Hash `bun:"target,type:bytea" json:"target"`
	// Message is the tag message, contains arbitrary text.
	Message *string `bun:"message" json:"message"`

	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}

type GetTagParams struct {
	id           uuid.UUID
	repositoryID uuid.UUID
	name         *string
}

func NewGetTagParams() *GetTagParams {
	return &GetTagParams{}
}

func (gup *GetTagParams) SetID(id uuid.UUID) *GetTagParams {
	gup.id = id
	return gup
}

func (gup *GetTagParams) SetRepositoryID(repositoryID uuid.UUID) *GetTagParams {
	gup.repositoryID = repositoryID
	return gup
}

func (gup *GetTagParams) SetName(name string) *GetTagParams {
	gup.name = &name
	return gup
}

type DeleteTagParams struct {
	id           uuid.UUID
	repositoryID uuid.UUID
}

func NewDeleteTagParams() *DeleteTagParams {
	return &DeleteTagParams{}
}

func (gup *DeleteTagParams) SetRepositoryID(repositoryID uuid.UUID) *DeleteTagParams {
	gup.repositoryID = repositoryID
	return gup
}
func (gup *DeleteTagParams) SetID(id uuid.UUID) *DeleteTagParams {
	gup.id = id
	return gup
}

type ListTagParams struct {
	RepositoryID uuid.UUID
	Name         *string
	NameMatch    MatchMode
	After        *time.Time
	Amount       int
}

func NewListTagParams() *ListTagParams {
	return &ListTagParams{}
}

func (gup *ListTagParams) SetRepositoryID(repositoryID uuid.UUID) *ListTagParams {
	gup.RepositoryID = repositoryID
	return gup
}

func (gup *ListTagParams) SetName(name string, match MatchMode) *ListTagParams {
	gup.Name = &name
	gup.NameMatch = match
	return gup
}

func (gup *ListTagParams) SetAfter(after time.Time) *ListTagParams {
	gup.After = &after
	return gup
}

func (gup *ListTagParams) SetAmount(amount int) *ListTagParams {
	gup.Amount = amount
	return gup
}

type ITagRepo interface {
	Insert(ctx context.Context, tag *Tag) (*Tag, error)
	Get(ctx context.Context, params *GetTagParams) (*Tag, error)
	Delete(ctx context.Context, params *DeleteTagParams) (int64, error)
	List(ctx context.Context, params *ListTagParams) ([]*Tag, bool, error)
}

type TagRepo struct {
	db bun.IDB
}

func NewTagRepo(db bun.IDB) ITagRepo {
	return &TagRepo{db: db}
}
func (t *TagRepo) Insert(ctx context.Context, tag *Tag) (*Tag, error) {
	_, err := t.db.NewInsert().
		Model(tag).
		Exec(ctx)
	if err != nil {
		return nil, err
	}
	return tag, nil
}

func (t *TagRepo) Get(ctx context.Context, params *GetTagParams) (*Tag, error) {
	tag := &Tag{}
	query := t.db.NewSelect().Model(tag)

	if uuid.Nil != params.id {
		query = query.Where("id = ?", params.id)
	}

	if uuid.Nil != params.repositoryID {
		query = query.Where("repository_id = ?", params.repositoryID)
	}

	if params.name != nil {
		query = query.Where("name = ?", *params.name)
	}

	err := query.Limit(1).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return tag, nil
}

func (t *TagRepo) List(ctx context.Context, params *ListTagParams) ([]*Tag, bool, error) {
	var tags []*Tag
	query := t.db.NewSelect().Model(&tags)

	if uuid.Nil != params.RepositoryID {
		query = query.Where("repository_id = ?", params.RepositoryID)
	}

	if params.Name != nil {
		switch params.NameMatch {
		case ExactMatch:
			query = query.Where("name = ?", *params.Name)
		case PrefixMatch:
			query = query.Where("name LIKE ?", *params.Name+"%")
		case SuffixMatch:
			query = query.Where("name LIKE ?", "%"+*params.Name)
		case LikeMatch:
			query = query.Where("name LIKE ?", "%"+*params.Name+"%")
		}
	}

	query = query.Order("updated_at DESC")
	if params.After != nil {
		query = query.Where("updated_at > ?", *params.After)
	}

	err := query.Limit(params.Amount).Scan(ctx)
	return tags, len(tags) == params.Amount, err
}

func (t *TagRepo) Delete(ctx context.Context, params *DeleteTagParams) (int64, error) {
	query := t.db.NewDelete().Model((*Tag)(nil))

	if uuid.Nil != params.id {
		query = query.Where("id = ?", params.id)
	}

	if uuid.Nil != params.repositoryID {
		query = query.Where("repository_id = ?", params.repositoryID)
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
