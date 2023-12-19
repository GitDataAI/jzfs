package models

import (
	"bytes"
	"context"
	"sort"
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

type TreeEntry struct {
	Name  string    `bun:"name"`
	IsDir bool      `bun:"is_dir"`
	Hash  hash.Hash `bun:"hash"`
}

func SortSubObjects(subObjects []TreeEntry) []TreeEntry {
	sort.Slice(subObjects, func(i, j int) bool {
		return subObjects[i].Name < subObjects[j].Name
	})
	return subObjects
}

func NewRootTreeEntry(hash hash.Hash) TreeEntry {
	return TreeEntry{
		Name: "",
		Hash: hash,
	}
}
func (treeEntry TreeEntry) Equal(other TreeEntry) bool {
	return bytes.Equal(treeEntry.Hash, other.Hash) && treeEntry.Name == other.Name
}

type Property struct {
	Mode filemode.FileMode `json:"mode"`
}

func DefaultDirProperty() Property {
	return Property{
		Mode: filemode.Dir,
	}
}

func DefaultLeafProperty() Property {
	return Property{
		Mode: filemode.Regular,
	}
}

func (props Property) ToMap() map[string]string {
	return map[string]string{
		"mode": props.Mode.String(),
	}
}

type Blob struct {
	bun.BaseModel `bun:"table:trees"`
	Hash          hash.Hash  `bun:"hash,pk,type:bytea"`
	CheckSum      hash.Hash  `bun:"check_sum,type:bytea"`
	Type          ObjectType `bun:"type"`
	Size          int64      `bun:"size"`
	Properties    Property   `bun:"properties,type:jsonb"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

func NewBlob(props Property, checkSum hash.Hash, size int64) (*Blob, error) {
	blob := &Blob{
		CheckSum:   checkSum,
		Type:       BlobObject,
		Size:       size,
		Properties: props,
		CreatedAt:  time.Now(),
		UpdatedAt:  time.Now(),
	}
	hash, err := blob.calculateHash()
	if err != nil {
		return nil, err
	}
	blob.Hash = hash
	return blob, err
}

func (blob *Blob) calculateHash() (hash.Hash, error) {
	hasher := hash.NewHasher(hash.Md5)
	err := hasher.WriteInt8(int8(blob.Type))
	if err != nil {
		return nil, err
	}

	_, err = hasher.Write(blob.CheckSum)
	if err != nil {
		return nil, err
	}

	//write mode property  todo change reflect
	for k, v := range blob.Properties.ToMap() {
		err = hasher.WriteString(k)
		if err != nil {
			return nil, err
		}

		err = hasher.WriteString(v)
		if err != nil {
			return nil, err
		}
	}
	return hasher.Md5.Sum(nil), nil
}

func (blob *Blob) FileTree() *FileTree {
	return &FileTree{
		Hash:       blob.Hash,
		Type:       blob.Type,
		Size:       blob.Size,
		CheckSum:   blob.CheckSum,
		Properties: blob.Properties,
		CreatedAt:  blob.CreatedAt,
		UpdatedAt:  blob.UpdatedAt,
	}
}

type TreeNode struct {
	bun.BaseModel `bun:"table:trees"`
	Hash          hash.Hash   `bun:"hash,pk,type:bytea"`
	Type          ObjectType  `bun:"type"`
	SubObjects    []TreeEntry `bun:"subObjs,type:jsonb"`
	Properties    Property    `bun:"properties,type:jsonb"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

func NewTreeNode(props Property, subObjects ...TreeEntry) (*TreeNode, error) {
	newTree := &TreeNode{
		Type:       TreeObject,
		SubObjects: SortSubObjects(subObjects),
		Properties: props,
		CreatedAt:  time.Now(),
		UpdatedAt:  time.Now(),
	}
	hash, err := newTree.calculateHash()
	if err != nil {
		return nil, err
	}
	newTree.Hash = hash
	return newTree, nil
}

func (tn *TreeNode) FileTree() *FileTree {
	return &FileTree{
		Hash:       tn.Hash,
		Type:       tn.Type,
		SubObjects: tn.SubObjects,
		Properties: tn.Properties,
		CreatedAt:  tn.CreatedAt,
		UpdatedAt:  tn.UpdatedAt,
	}
}

func (tn *TreeNode) calculateHash() (hash.Hash, error) {
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
	}

	for name, value := range tn.Properties.ToMap() {
		err = hasher.WriteString(name)
		if err != nil {
			return nil, err
		}
		err = hasher.WriteString(value)
		if err != nil {
			return nil, err
		}
	}

	return hasher.Md5.Sum(nil), nil
}

type FileTree struct {
	bun.BaseModel `bun:"table:trees"`
	Hash          hash.Hash  `bun:"hash,pk,type:bytea"`
	Type          ObjectType `bun:"type"`
	Size          int64      `bun:"size"`
	CheckSum      hash.Hash  `bun:"check_sum,type:bytea"`
	Properties    Property   `bun:"properties,type:jsonb"`
	//tree
	SubObjects []TreeEntry `bun:"subObjs,type:jsonb"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

func (fileTree *FileTree) Blob() *Blob {
	return &Blob{
		Hash:       fileTree.Hash,
		Type:       fileTree.Type,
		Size:       fileTree.Size,
		Properties: fileTree.Properties,
		CheckSum:   fileTree.CheckSum,
		CreatedAt:  fileTree.CreatedAt,
		UpdatedAt:  fileTree.UpdatedAt,
	}
}

func (fileTree *FileTree) TreeNode() *TreeNode {
	return &TreeNode{
		Hash:       fileTree.Hash,
		Type:       fileTree.Type,
		Properties: fileTree.Properties,
		SubObjects: fileTree.SubObjects,
		CreatedAt:  fileTree.CreatedAt,
		UpdatedAt:  fileTree.UpdatedAt,
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

type IFileTreeRepo interface {
	Insert(ctx context.Context, repo *FileTree) (*FileTree, error)
	Get(ctx context.Context, params *GetObjParams) (*FileTree, error)
	Count(ctx context.Context) (int, error)
	List(ctx context.Context) ([]FileTree, error)
	Blob(ctx context.Context, hash hash.Hash) (*Blob, error)
	TreeNode(ctx context.Context, hash hash.Hash) (*TreeNode, error)
}

var _ IFileTreeRepo = (*FileTreeRepo)(nil)

type FileTreeRepo struct {
	db bun.IDB
}

func NewFileTree(db bun.IDB) IFileTreeRepo {
	return &FileTreeRepo{db: db}
}

func (o FileTreeRepo) Insert(ctx context.Context, obj *FileTree) (*FileTree, error) {
	_, err := o.db.NewInsert().Model(obj).Ignore().Exec(ctx)
	if err != nil {
		return nil, err
	}
	return obj, nil
}

func (o FileTreeRepo) Get(ctx context.Context, params *GetObjParams) (*FileTree, error) {
	repo := &FileTree{}
	query := o.db.NewSelect().Model(repo)

	if params.Hash != nil {
		query = query.Where("hash = ?", params.Hash)
	}

	err := query.Limit(1).Scan(ctx, repo)
	if err != nil {
		return nil, err
	}
	return repo, nil
}

func (o FileTreeRepo) Blob(ctx context.Context, hash hash.Hash) (*Blob, error) {
	blob := &Blob{}
	err := o.db.NewSelect().Model(blob).Limit(1).Where("hash = ?", hash).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return blob, nil
}

func (o FileTreeRepo) TreeNode(ctx context.Context, hash hash.Hash) (*TreeNode, error) {
	tree := &TreeNode{}
	err := o.db.NewSelect().Model(tree).Limit(1).Where("hash = ?", hash).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return tree, nil
}

func (o FileTreeRepo) Count(ctx context.Context) (int, error) {
	return o.db.NewSelect().Model((*FileTree)(nil)).Count(ctx)
}

func (o FileTreeRepo) List(ctx context.Context) ([]FileTree, error) {
	var obj []FileTree
	err := o.db.NewSelect().Model(&obj).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return obj, nil
}
