package versionmgr

import (
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/jiaozifs/jiaozifs/block"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/jiaozifs/jiaozifs/utils/pathutil"

	"golang.org/x/exp/slices"

	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/filemode"
)

var EmptyRoot = &models.TreeNode{
	Hash: hash.Hash([]byte{}),
	Type: models.TreeObject,
}

var (
	ErrPathNotFound   = fmt.Errorf("path not found")
	ErrEntryExit      = fmt.Errorf("entry exit")
	ErrBlobMustBeLeaf = fmt.Errorf("blob must be leaf")
	ErrNotDiretory    = fmt.Errorf("path must be a directory")
)

type TreeNodeWithNode struct {
	Node *models.Object
	Name string
}

type TreeOp struct {
	Object models.IObjectRepo
}

func NewTreeOp(object models.IObjectRepo) *TreeOp {
	return &TreeOp{
		Object: object,
	}
}

func (treeOp *TreeOp) WriteBlob(ctx context.Context, adapter block.Adapter, body io.Reader, contentLength int64, opts block.PutOpts) (*models.Blob, error) {
	// handle the upload itself
	hashReader := hash.NewHashingReader(body, hash.Md5)
	tempf, err := os.CreateTemp("", "*")
	if err != nil {
		return nil, err
	}
	_, err = io.Copy(tempf, hashReader)
	if err != nil {
		return nil, err
	}

	hash := hash.Hash(hashReader.Md5.Sum(nil))
	_, err = tempf.Seek(0, io.SeekStart)
	if err != nil {
		return nil, err
	}

	defer func() {
		name := tempf.Name()
		_ = tempf.Close()
		_ = os.RemoveAll(name)
	}()

	address := pathutil.PathOfHash(hash)
	err = adapter.Put(ctx, block.ObjectPointer{
		StorageNamespace: adapter.BlockstoreType() + "://",
		IdentifierType:   block.IdentifierTypeRelative,
		Identifier:       address,
	}, contentLength, tempf, opts)
	if err != nil {
		return nil, err
	}

	return &models.Blob{
		Hash: hash,
		Size: hashReader.CopiedSize,
	}, nil
}

func (treeOp *TreeOp) SubDir(ctx context.Context, tn *models.TreeNode, name string) (*models.TreeNode, error) {
	for _, node := range tn.SubObjects {
		if node.Name == name {
			if node.Mode == filemode.Dir {
				return treeOp.Object.TreeNode(ctx, node.Hash)
			}
			return nil, fmt.Errorf("node is not directory")
		}
	}
	return nil, ErrPathNotFound
}

func (treeOp *TreeOp) SubFile(ctx context.Context, tn *models.TreeNode, name string) (*models.Blob, error) {
	for _, node := range tn.SubObjects {
		if node.Name == name {
			if node.Mode == filemode.Regular || node.Mode == filemode.Executable {
				return treeOp.Object.Blob(ctx, node.Hash)
			}
			return nil, fmt.Errorf("node is not blob")
		}
	}
	return nil, ErrPathNotFound
}

func (treeOp *TreeOp) SubEntry(_ context.Context, tn *models.TreeNode, name string) (models.TreeEntry, error) {
	for _, node := range tn.SubObjects {
		if node.Name == name {
			return node, nil
		}
	}
	return models.TreeEntry{}, ErrPathNotFound
}

func (treeOp *TreeOp) AppendTreeEntry(ctx context.Context, tn *models.TreeNode, treeEntry models.TreeEntry) (*models.TreeNode, error) {
	for _, node := range tn.SubObjects {
		if node.Name == treeEntry.Name {
			return nil, ErrEntryExit
		}
	}

	newTree := &models.TreeNode{
		Type:       tn.Type,
		SubObjects: tn.SubObjects,
		CreatedAt:  time.Now(),
		UpdatedAt:  time.Now(),
	}
	newTree.SubObjects = append(newTree.SubObjects, treeEntry)
	hash, err := newTree.GetHash()
	if err != nil {
		return nil, err
	}
	newTree.Hash = hash

	obj, err := treeOp.Object.Insert(ctx, newTree.Object())
	if err != nil {
		return nil, err
	}
	return obj.TreeNode(), nil
}

func (treeOp *TreeOp) DeleteDirectObject(ctx context.Context, tn *models.TreeNode, name string) (*models.TreeNode, bool, error) {
	newTree := &models.TreeNode{
		Type:      tn.Type,
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	}
	for _, sub := range tn.SubObjects {
		if sub.Name != name { //filter tree entry by name
			newTree.SubObjects = append(newTree.SubObjects, sub)
		}
	}

	if len(newTree.SubObjects) == 0 {
		//this node has no element return nothing
		return nil, true, nil
	}

	hash, err := newTree.GetHash()
	if err != nil {
		return nil, false, err
	}
	newTree.Hash = hash

	obj, err := treeOp.Object.Insert(ctx, newTree.Object())
	if err != nil {
		return nil, false, err
	}
	return obj.TreeNode(), false, nil
}

func (treeOp *TreeOp) ReplaceTreeEntry(ctx context.Context, tn *models.TreeNode, treeEntry models.TreeEntry) (*models.TreeNode, error) {
	index := -1
	var sub models.TreeEntry
	for index, sub = range tn.SubObjects {
		if sub.Name == treeEntry.Name {
			break
		}
	}
	if index == -1 {
		return nil, ErrPathNotFound
	}

	newTree := &models.TreeNode{
		Type:       tn.Type,
		SubObjects: make([]models.TreeEntry, len(tn.SubObjects)),
		CreatedAt:  time.Now(),
		UpdatedAt:  time.Now(),
	}
	copy(newTree.SubObjects, tn.SubObjects)
	newTree.SubObjects[index] = treeEntry

	hash, err := newTree.GetHash()
	if err != nil {
		return nil, err
	}
	newTree.Hash = hash

	obj, err := treeOp.Object.Insert(ctx, newTree.Object())
	if err != nil {
		return nil, err
	}
	return obj.TreeNode(), nil
}

func (treeOp *TreeOp) MatchPath(ctx context.Context, tn *models.TreeNode, path string) ([]TreeNodeWithNode, []string, error) {
	pathSegs := strings.Split(filepath.Clean(path), fmt.Sprintf("%c", os.PathSeparator))
	var existNodes []TreeNodeWithNode
	var missingPath []string
	//a/b/c/d/e
	//a/b/c
	//a/b/c/d/e/f/g
	for index, seg := range pathSegs {
		entry, err := treeOp.SubEntry(ctx, tn, seg)
		if errors.Is(err, ErrPathNotFound) {
			missingPath = pathSegs[index:]
			return existNodes, missingPath, nil
		}

		if entry.Mode == filemode.Dir {
			tn, err = treeOp.SubDir(ctx, tn, entry.Name)
			if err != nil {
				return nil, nil, err
			}
			existNodes = append(existNodes, TreeNodeWithNode{
				Node: tn.Object(),
				Name: entry.Name,
			})
		} else {
			//must be file
			blob, err := treeOp.SubFile(ctx, tn, entry.Name)
			if err != nil {
				return nil, nil, err
			}
			existNodes = append(existNodes, TreeNodeWithNode{
				Node: blob.Object(),
				Name: entry.Name,
			})

			if index != len(pathSegs)-1 {
				//blob must be leaf
				return nil, nil, ErrBlobMustBeLeaf
			}
		}

	}
	return existNodes, nil, nil
}

// AddLeaf insert new leaf in entry, if path not exit, create new
func (treeOp *TreeOp) AddLeaf(ctx context.Context, root *models.TreeNode, fullPath string, blob *models.Blob) (*models.TreeNode, error) {
	existNode, missingPath, err := treeOp.MatchPath(ctx, root, fullPath)
	if err != nil {
		return nil, err
	}

	if len(missingPath) == 0 {
		return nil, ErrEntryExit
	}

	_, err = treeOp.Object.Insert(ctx, blob.Object())
	if err != nil {
		return nil, err
	}

	slices.Reverse(missingPath)
	var lastEntry models.TreeEntry
	for index, path := range missingPath {
		if index == 0 {
			_, err = treeOp.Object.Insert(ctx, blob.Object())
			if err != nil {
				return nil, err
			}
			lastEntry = models.TreeEntry{
				Name: path,
				Mode: filemode.Regular,
				Hash: blob.Hash,
			}
			continue
		}

		newTree, err := models.NewTreeNode(lastEntry)
		if err != nil {
			return nil, err
		}
		_, err = treeOp.Object.Insert(ctx, newTree.Object())
		if err != nil {
			return nil, err
		}
		lastEntry = models.TreeEntry{
			Name: path,
			Mode: filemode.Dir,
			Hash: newTree.Hash,
		}
	}

	slices.Reverse(existNode)
	existNode = append(existNode, TreeNodeWithNode{
		Node: root.Object(),
		Name: "", //root node have no name
	})

	var newNode *models.TreeNode
	for index, node := range existNode {
		if index == 0 { //insert new node
			newNode, err = treeOp.AppendTreeEntry(ctx, node.Node.TreeNode(), lastEntry)
		} else { //replace node
			newNode, err = treeOp.ReplaceTreeEntry(ctx, node.Node.TreeNode(), lastEntry)
		}
		if err != nil {
			return nil, err
		}
		lastEntry = models.TreeEntry{
			Name: node.Name,
			Mode: filemode.Dir,
			Hash: newNode.Hash,
		}
	}
	return newNode, nil
}

// ReplaceLeaf replace leaf with a new blob, all parent directory updated
func (treeOp *TreeOp) ReplaceLeaf(ctx context.Context, root *models.TreeNode, fullPath string, blob *models.Blob) (*models.TreeNode, error) {
	existNode, missingPath, err := treeOp.MatchPath(ctx, root, fullPath)
	if err != nil {
		return nil, err
	}

	if len(missingPath) > 0 {
		return nil, ErrPathNotFound
	}

	_, err = treeOp.Object.Insert(ctx, blob.Object())
	if err != nil {
		return nil, err
	}

	slices.Reverse(existNode)
	existNode = append(existNode, TreeNodeWithNode{
		Node: root.Object(),
		Name: "", //root node have no name
	})

	var lastEntry models.TreeEntry
	var newNode *models.TreeNode
	for index, node := range existNode {
		if index == 0 {
			lastEntry = models.TreeEntry{
				Name: node.Name,
				Mode: filemode.Regular,
				Hash: blob.Hash,
			}
			continue
		}

		newNode, err = treeOp.ReplaceTreeEntry(ctx, node.Node.TreeNode(), lastEntry)
		if err != nil {
			return nil, err
		}
		lastEntry = models.TreeEntry{
			Name: node.Name,
			Mode: filemode.Dir,
			Hash: newNode.Hash,
		}
	}
	return newNode, nil
}

// RemoveEntry remove tree entry from specific tree, if directory have only one entry, this directory was remove too
// examples:  a -> b
// a
// └── b
//
//	├── c.txt
//	└── d.txt
//
// RemoveEntry(ctx, root, "a") return empty root
// RemoveEntry(ctx, root, "a/b/c.txt") return new root of(a/b/c.txt)
// RemoveEntry(ctx, root, "a/b") return empty root. a b c.txt d.txt all removed
func (treeOp *TreeOp) RemoveEntry(ctx context.Context, root *models.TreeNode, fullPath string) (*models.TreeNode, error) {
	existNode, missingPath, err := treeOp.MatchPath(ctx, root, fullPath)
	if err != nil {
		return nil, err
	}

	if len(missingPath) > 0 {
		return nil, ErrPathNotFound
	}

	slices.Reverse(existNode)
	existNode = append(existNode, TreeNodeWithNode{
		Node: root.Object(),
		Name: "", //root node have no name
	})

	lastEntry := models.TreeEntry{
		Name: existNode[0].Name,
		Mode: filemode.Dir,
		Hash: existNode[0].Node.Hash,
	}
	existNode = existNode[1:]

	var newNode *models.TreeNode
	for index, node := range existNode {
		if index == 0 || lastEntry.Hash.IsEmpty() {
			var isEmpty bool
			newNode, isEmpty, err = treeOp.DeleteDirectObject(ctx, node.Node.TreeNode(), lastEntry.Name)
			if err != nil {
				return nil, err
			}

			lastEntry = models.TreeEntry{
				Name: node.Name,
				Mode: filemode.Dir,
			}
			if !isEmpty {
				lastEntry.Hash = newNode.Hash
			}
		} else {
			newNode, err = treeOp.ReplaceTreeEntry(ctx, node.Node.TreeNode(), lastEntry)
			if err != nil {
				return nil, err
			}
			lastEntry = models.TreeEntry{
				Name: node.Name,
				Mode: filemode.Dir,
				Hash: newNode.Hash,
			}
		}
	}
	if newNode == nil {
		return EmptyRoot, nil
	}
	return newNode, nil
}

// Ls list tree entry of specific path of specific root
// examples:  a -> b
// a
// └── b
//
//	├── c.txt
//	└── d.txt
//
// Ls(ctx, root, "a") return b
// Ls(ctx, root, "a/b" return c.txt and d.txt
func (treeOp *TreeOp) Ls(ctx context.Context, root *models.TreeNode, fullPath string) ([]models.TreeEntry, error) {
	if len(fullPath) == 0 {
		return root.SubObjects, nil
	}

	existNode, missingPath, err := treeOp.MatchPath(ctx, root, fullPath)
	if err != nil {
		return nil, err
	}

	if len(missingPath) > 0 {
		return nil, ErrPathNotFound
	}

	lastNode := existNode[len(existNode)-1]
	if lastNode.Node.Type != models.TreeObject {
		return nil, ErrNotDiretory
	}

	return lastNode.Node.SubObjects, nil
}
