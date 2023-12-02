package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models/filemode"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/uptrace/bun"
)

// ObjectType internal object type
// Integer values from 0 to 7 map to those exposed by git.
// AnyObject is used to represent any from 0 to 7.
type ObjectType int8

const (
	InvalidObject ObjectType = 0
	CommitObject  ObjectType = 1
	TreeObject    ObjectType = 2
	BlobObject    ObjectType = 3
	TagObject     ObjectType = 4
)

// Signature is used to identify who and when created a commit or tag.
type Signature struct {
	// Name represents a person name. It is an arbitrary string.
	Name string `bun:"name"`
	// Email is an email, but it cannot be assumed to be well-formed.
	Email string `bun:"email"`
	// When is the timestamp of the signature.
	When time.Time `bun:"when"`
}

type TreeEntry struct {
	Name string            `bun:"name"`
	Mode filemode.FileMode `bun:"mode"`
	ID   uuid.UUID         `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Hash hash.Hash         `bun:"hash"`
}

type Blob struct {
	bun.BaseModel   `bun:"table:object"`
	ID              uuid.UUID  `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Hash            hash.Hash  `bun:"hash,type:bytea"`
	Type            ObjectType `bun:"type"`
	PhysicalAddress string     `bun:"physical_address"`
	RelativePath    bool       `bun:"relative_path"`
	Size            int64      `bun:"size"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type TreeNode struct {
	bun.BaseModel `bun:"table:object"`
	ID            uuid.UUID   `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Hash          hash.Hash   `bun:"hash,type:bytea"`
	Type          ObjectType  `bun:"type"`
	SubObject     []TreeEntry `bun:"subObj,type:jsonb"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type Commit struct {
	bun.BaseModel `bun:"table:object"`
	ID            uuid.UUID  `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Hash          hash.Hash  `bun:"hash,type:bytea"`
	Type          ObjectType `bun:"type"`
	//////********commit********////////
	// Author is the original author of the commit.
	Author Signature `bun:"author,type:jsonb"`
	// Committer is the one performing the commit, might be different from
	// Author.
	Committer Signature `bun:"committer,type:jsonb"`
	// MergeTag is the embedded tag object when a merge commit is created by
	// merging a signed tag.
	MergeTag string `bun:"merge_tag"` //todo
	// Message is the commit/tag message, contains arbitrary text.
	Message string `bun:"message"`
	// TreeHash is the hash of the root tree of the commit.
	TreeId uuid.UUID `bun:"tree_id,type:uuid,notnull"`
	// ParentHashes are the hashes of the parent commits of the commit.
	ParentHashes []hash.Hash `bun:"parent_hashes,type:bytea[]"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type Tag struct {
	bun.BaseModel `bun:"table:object"`
	ID            uuid.UUID  `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Hash          hash.Hash  `bun:"hash,type:bytea"`
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

type Object struct {
	bun.BaseModel `bun:"table:object"`
	ID            uuid.UUID  `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Hash          hash.Hash  `bun:"hash,type:bytea"`
	Type          ObjectType `bun:"type"`
	Size          int64      `bun:"size"`
	//tree
	SubObject []TreeEntry `bun:"subObj,type:jsonb"`

	//////********commit********////////
	// Author is the original author of the commit.
	Author Signature `bun:"author,type:jsonb"`
	// Committer is the one performing the commit, might be different from
	// Author.
	Committer Signature `bun:"committer,type:jsonb"`
	// MergeTag is the embedded tag object when a merge commit is created by
	// merging a signed tag.
	MergeTag string `bun:"merge_tag"` //todo
	// Message is the commit/tag message, contains arbitrary text.
	Message string `bun:"message"`
	// TreeHash is the hash of the root tree of the commit.
	TreeHash hash.Hash `bun:"tree_hash,type:bytea"`
	// ParentHashes are the hashes of the parent commits of the commit.
	ParentHashes []hash.Hash `bun:"parent_hashes,type:bytea[]"`

	//////********commit********////////
	// Name of the tag.
	Name string `bun:"name"`
	// Tagger is the one who created the tag.
	Tagger Signature `bun:"tagger,type:jsonb"`
	// TargetType is the object type of the target.
	TargetType ObjectType `bun:"target_type"`
	// Target is the hash of the target object.
	Target hash.Hash `bun:"target,type:bytea"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

type GetObjParams struct {
	ID uuid.UUID
}

type IObjectRepo interface {
	Insert(ctx context.Context, repo *Object) (*Object, error)
	Get(ctx context.Context, params *GetObjParams) (*Object, error)
	Count(ctx context.Context) (int, error)
	List(ctx context.Context) ([]Object, error)
	Blob(ctx context.Context, id uuid.UUID) (*Blob, error)
	TreeNode(ctx context.Context, id uuid.UUID) (*TreeNode, error)
	Commit(ctx context.Context, id uuid.UUID) (*Commit, error)
	Tag(ctx context.Context, id uuid.UUID) (*Tag, error)
}

var _ IObjectRepo = (*ObjectRepo)(nil)

type ObjectRepo struct {
	db *bun.DB
}

func NewObjectRepo(db *bun.DB) IObjectRepo {
	return &ObjectRepo{db}
}

func (o ObjectRepo) Insert(ctx context.Context, obj *Object) (*Object, error) {
	_, err := o.db.NewInsert().Model(obj).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return obj, nil
}

func (o ObjectRepo) Get(ctx context.Context, params *GetObjParams) (*Object, error) {
	repo := &Object{}
	query := o.db.NewSelect().Model(repo)

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	return repo, query.Scan(ctx, repo)
}

func (o ObjectRepo) Blob(ctx context.Context, id uuid.UUID) (*Blob, error) {
	blob := &Blob{}
	return blob, o.db.NewSelect().Model(blob).Where("id = ?", id).Scan(ctx)
}

func (o ObjectRepo) TreeNode(ctx context.Context, id uuid.UUID) (*TreeNode, error) {
	tree := &TreeNode{}
	return tree, o.db.NewSelect().Model(tree).Where("id = ?", id).Scan(ctx)
}

func (o ObjectRepo) Commit(ctx context.Context, id uuid.UUID) (*Commit, error) {
	commit := &Commit{}
	return commit, o.db.NewSelect().Model(commit).Where("id = ?", id).Scan(ctx)
}

func (o ObjectRepo) Tag(ctx context.Context, id uuid.UUID) (*Tag, error) {
	tag := &Tag{}
	return tag, o.db.NewSelect().Model(tag).Where("id = ?", id).Scan(ctx)
}

func (o ObjectRepo) Count(ctx context.Context) (int, error) {
	return o.db.NewSelect().Model((*Object)(nil)).Count(ctx)
}

func (o ObjectRepo) List(ctx context.Context) ([]Object, error) {
	obj := []Object{}
	return obj, o.db.NewSelect().Model(&obj).Scan(ctx)
}
