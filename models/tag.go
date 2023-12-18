package models

import (
	"context"
	"time"

	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/uptrace/bun"
)

type Tag struct {
	bun.BaseModel `bun:"table:tags"`
	Hash          hash.Hash  `bun:"hash,pk,type:bytea"`
	Type          ObjectType `bun:"type"`
	//////********commit********////////
	// Name of the tag.
	Name string `bun:"name"`
	// Tagger is the one who created the tag.
	Tagger Signature `bun:"tagger,type:jsonb"`
	// TargetType is the object type of the target.
	TargetType ObjectType `bun:"target_type"`
	// Target is the hash of the target object.
	Target hash.Hash `bun:"target,type:bytea"`
	// Message is the tag message, contains arbitrary text.
	Message string `bun:"message"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type ITagRepo interface {
	Insert(ctx context.Context, tag *Tag) (*Tag, error)
	Tag(ctx context.Context, hash hash.Hash) (*Tag, error)
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

func (t *TagRepo) Tag(ctx context.Context, hash hash.Hash) (*Tag, error) {
	tag := &Tag{}
	err := t.db.NewSelect().
		Model(tag).
		Where("hash = ?", hash).
		Scan(ctx)
	if err != nil {
		return nil, err
	}
	return tag, nil
}
