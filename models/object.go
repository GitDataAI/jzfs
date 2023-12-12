package models

import (
	"bytes"
	"context"
	"time"

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
	Hash hash.Hash         `bun:"hash"`
}

func NewRootTreeEntry(hash hash.Hash) TreeEntry {
	return TreeEntry{
		Name: "",
		Mode: filemode.Dir,
		Hash: hash,
	}
}
func (treeEntry TreeEntry) Equal(other TreeEntry) bool {
	return bytes.Equal(treeEntry.Hash, other.Hash) && treeEntry.Mode == other.Mode && treeEntry.Name == other.Name
}

type Blob struct {
	bun.BaseModel `bun:"table:object"`
	Hash          hash.Hash  `bun:"hash,pk,type:bytea"`
	Type          ObjectType `bun:"type"`
	Size          int64      `bun:"size"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

func (blob *Blob) Object() *Object {
	return &Object{
		Hash:      blob.Hash,
		Type:      blob.Type,
		Size:      blob.Size,
		CreatedAt: blob.CreatedAt,
		UpdatedAt: blob.UpdatedAt,
	}
}

type TreeNode struct {
	bun.BaseModel `bun:"table:object"`
	Hash          hash.Hash   `bun:"hash,pk,type:bytea"`
	Type          ObjectType  `bun:"type"`
	SubObjects    []TreeEntry `bun:"subObjs,type:jsonb"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

func NewTreeNode(subObjects ...TreeEntry) (*TreeNode, error) {
	newTree := &TreeNode{
		Type:       TreeObject,
		SubObjects: subObjects,
		CreatedAt:  time.Now(),
		UpdatedAt:  time.Now(),
	}
	hash, err := newTree.GetHash()
	if err != nil {
		return nil, err
	}
	newTree.Hash = hash
	return newTree, nil
}

func (tn *TreeNode) Object() *Object {
	return &Object{
		Hash:       tn.Hash,
		Type:       tn.Type,
		SubObjects: tn.SubObjects,
		CreatedAt:  tn.CreatedAt,
		UpdatedAt:  tn.UpdatedAt,
	}
}

func (tn *TreeNode) GetHash() (hash.Hash, error) {
	hasher := hash.NewHasher(hash.Md5)
	err := hasher.WriteInt8(int8(tn.Type))
	if err != nil {
		return nil, err
	}
	for _, obj := range tn.SubObjects {
		_, err = hasher.Write(obj.Hash)
		if err != nil {
			return nil, err
		}
		err = hasher.WriteString(obj.Name)
		if err != nil {
			return nil, err
		}
		err = hasher.WriteUint32(uint32(obj.Mode))
		if err != nil {
			return nil, err
		}
	}

	return hasher.Md5.Sum(nil), nil
}

type Commit struct {
	bun.BaseModel `bun:"table:object"`
	Hash          hash.Hash  `bun:"hash,pk,type:bytea"`
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
	TreeHash hash.Hash `bun:"tree_hash,type:bytea,notnull"`
	// ParentHashes are the hashes of the parent commits of the commit.
	ParentHashes []hash.Hash `bun:"parent_hashes,type:bytea[]"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

func (commit *Commit) GetHash() (hash.Hash, error) {
	hasher := hash.NewHasher(hash.Md5)
	err := hasher.WriteInt8(int8(commit.Type))
	if err != nil {
		return nil, err
	}
	err = hasher.WriteString(commit.Author.Name)
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Author.Email)
	if err != nil {
		return nil, err
	}

	err = hasher.WritInt64(commit.Author.When.Unix())
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Committer.Name)
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Committer.Email)
	if err != nil {
		return nil, err
	}

	err = hasher.WritInt64(commit.Committer.When.Unix())
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.MergeTag)
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Message)
	if err != nil {
		return nil, err
	}

	_, err = hasher.Write(commit.TreeHash)
	if err != nil {
		return nil, err
	}

	for _, h := range commit.ParentHashes {
		_, err = hasher.Write(h)
		if err != nil {
			return nil, err
		}
	}

	return hasher.Md5.Sum(nil), nil
}

func (commit *Commit) NumParents() int {
	return len(commit.ParentHashes)
}

func (commit *Commit) Object() *Object {
	return &Object{
		Hash:         commit.Hash,
		Type:         commit.Type,
		Author:       commit.Author,
		Committer:    commit.Committer,
		MergeTag:     commit.MergeTag,
		Message:      commit.Message,
		TreeHash:     commit.TreeHash,
		ParentHashes: commit.ParentHashes,
		CreatedAt:    commit.CreatedAt,
		UpdatedAt:    commit.UpdatedAt,
	}
}

type Tag struct {
	bun.BaseModel `bun:"table:object"`
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

func (tag *Tag) Object() *Object {
	return &Object{
		Hash:       tag.Hash,
		Type:       tag.Type,
		Name:       tag.Name,
		Tagger:     tag.Tagger,
		TargetType: tag.TargetType,
		Target:     tag.Target,
		Message:    tag.Message,
		CreatedAt:  tag.CreatedAt,
		UpdatedAt:  tag.UpdatedAt,
	}
}

type Object struct {
	bun.BaseModel `bun:"table:object"`
	Hash          hash.Hash  `bun:"hash,pk,type:bytea"`
	Type          ObjectType `bun:"type"`
	Size          int64      `bun:"size"`
	//tree
	SubObjects []TreeEntry `bun:"subObjs,type:jsonb"`

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

func (obj *Object) Blob() *Blob {
	return &Blob{
		Hash:      obj.Hash,
		Type:      obj.Type,
		Size:      obj.Size,
		CreatedAt: obj.CreatedAt,
		UpdatedAt: obj.UpdatedAt,
	}
}

func (obj *Object) TreeNode() *TreeNode {
	return &TreeNode{
		Hash:       obj.Hash,
		Type:       obj.Type,
		SubObjects: obj.SubObjects,
		CreatedAt:  obj.CreatedAt,
		UpdatedAt:  obj.UpdatedAt,
	}
}

func (obj *Object) Commit() *Commit {
	return &Commit{
		Hash:         obj.Hash,
		Type:         obj.Type,
		Author:       obj.Author,
		Committer:    obj.Committer,
		MergeTag:     obj.MergeTag,
		Message:      obj.Message,
		TreeHash:     obj.TreeHash,
		ParentHashes: obj.ParentHashes,
		CreatedAt:    obj.CreatedAt,
		UpdatedAt:    obj.UpdatedAt,
	}
}

func (obj *Object) Tag() *Tag {
	return &Tag{
		Hash:       obj.Hash,
		Type:       obj.Type,
		Name:       obj.Name,
		Tagger:     obj.Tagger,
		TargetType: obj.TargetType,
		Target:     obj.Target,
		Message:    obj.Message,
		CreatedAt:  obj.CreatedAt,
		UpdatedAt:  obj.UpdatedAt,
	}
}

type GetObjParams struct {
	Hash hash.Hash
}

func NewGetObjParams() *GetObjParams {
	return &GetObjParams{}
}

func (gop *GetObjParams) SetHash(hash hash.Hash) *GetObjParams {
	gop.Hash = hash
	return gop
}

type IObjectRepo interface {
	Insert(ctx context.Context, repo *Object) (*Object, error)
	Get(ctx context.Context, params *GetObjParams) (*Object, error)
	Count(ctx context.Context) (int, error)
	List(ctx context.Context) ([]Object, error)
	Blob(ctx context.Context, hash hash.Hash) (*Blob, error)
	TreeNode(ctx context.Context, hash hash.Hash) (*TreeNode, error)
	Commit(ctx context.Context, hash hash.Hash) (*Commit, error)
	Tag(ctx context.Context, hash hash.Hash) (*Tag, error)
}

var _ IObjectRepo = (*ObjectRepo)(nil)

type ObjectRepo struct {
	db bun.IDB
}

func NewObjectRepo(db bun.IDB) IObjectRepo {
	return &ObjectRepo{db: db}
}

func (o ObjectRepo) Insert(ctx context.Context, obj *Object) (*Object, error) {
	_, err := o.db.NewInsert().Model(obj).Ignore().Exec(ctx)
	if err != nil {
		return nil, err
	}
	return obj, nil
}

func (o ObjectRepo) Get(ctx context.Context, params *GetObjParams) (*Object, error) {
	repo := &Object{}
	query := o.db.NewSelect().Model(repo)

	if params.Hash != nil {
		query = query.Where("hash = ?", params.Hash)
	}

	return repo, query.Limit(1).Scan(ctx, repo)
}

func (o ObjectRepo) Blob(ctx context.Context, hash hash.Hash) (*Blob, error) {
	blob := &Blob{}
	return blob, o.db.NewSelect().Model(blob).Limit(1).Where("hash = ?", hash).Scan(ctx)
}

func (o ObjectRepo) TreeNode(ctx context.Context, hash hash.Hash) (*TreeNode, error) {
	tree := &TreeNode{}
	return tree, o.db.NewSelect().Model(tree).Limit(1).Where("hash = ?", hash).Scan(ctx)
}

func (o ObjectRepo) Commit(ctx context.Context, hash hash.Hash) (*Commit, error) {
	commit := &Commit{}
	return commit, o.db.NewSelect().Model(commit).Limit(1).Where("hash = ?", hash).Scan(ctx)
}

func (o ObjectRepo) Tag(ctx context.Context, hash hash.Hash) (*Tag, error) {
	tag := &Tag{}
	return tag, o.db.NewSelect().Model(tag).Limit(1).Where("hash = ?", hash).Scan(ctx)
}

func (o ObjectRepo) Count(ctx context.Context) (int, error) {
	return o.db.NewSelect().Model((*Object)(nil)).Count(ctx)
}

func (o ObjectRepo) List(ctx context.Context) ([]Object, error) {
	var obj []Object
	return obj, o.db.NewSelect().Model(&obj).Scan(ctx)
}
