package versionmgr

import (
	"bytes"
	"context"
	"fmt"

	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/versionmgr/merkletrie/noder"
)

var _ noder.Noder = (*TreeNode)(nil)

type TreeNode struct {
	ctx        context.Context
	entry      models.TreeEntry
	treeNode   *models.TreeNode
	objectRepo models.IFileTreeRepo
}

func NewTreeNode(ctx context.Context, entry models.TreeEntry, object models.IFileTreeRepo) (*TreeNode, error) {
	treeNode := EmptyRoot
	if !entry.Equal(EmptyDirEntry) {
		var err error
		treeNode, err = object.TreeNode(ctx, entry.Hash)
		if err != nil {
			return nil, err
		}
	}

	return &TreeNode{ctx: ctx, entry: entry, treeNode: treeNode, objectRepo: object}, nil
}

func (n TreeNode) Type() models.ObjectType {
	return n.treeNode.Type
}

func (n TreeNode) Properties() models.Property {
	return n.treeNode.Properties
}

func (n TreeNode) SubObjects() []models.TreeEntry {
	return n.treeNode.SubObjects
}

func (n TreeNode) TreeNode() *models.TreeNode {
	return n.treeNode
}

func (n TreeNode) Equal(other noder.Noder) bool {
	return bytes.Equal(n.Hash(), other.Hash())
}

func (n TreeNode) SubDir(ctx context.Context, name string) (*TreeNode, error) {
	for _, node := range n.treeNode.SubObjects {
		if node.Name == name {
			if node.IsDir {
				return NewTreeNode(ctx, node, n.objectRepo)
			}
			return nil, fmt.Errorf("node is not directory")
		}
	}
	return nil, ErrPathNotFound
}

func (n TreeNode) SubFile(ctx context.Context, name string) (*models.Blob, error) {
	for _, node := range n.treeNode.SubObjects {
		if node.Name == name {
			if !node.IsDir {
				return n.objectRepo.Blob(ctx, node.Hash)
			}
			return nil, fmt.Errorf("node is not blob")
		}
	}
	return nil, ErrPathNotFound
}

func (n TreeNode) SubEntry(_ context.Context, name string) (models.TreeEntry, error) {
	for _, node := range n.treeNode.SubObjects {
		if node.Name == name {
			return node, nil
		}
	}
	return models.TreeEntry{}, ErrPathNotFound
}

func (n TreeNode) Hash() []byte {
	return n.entry.Hash
}

func (n TreeNode) String() string {
	return n.entry.Name + " " + n.entry.Hash.Hex()
}

func (n TreeNode) Name() string {
	return n.entry.Name
}

func (n TreeNode) IsDir() bool {
	return n.entry.IsDir
}

func (n TreeNode) Children() ([]noder.Noder, error) {
	children := make([]noder.Noder, len(n.treeNode.SubObjects))
	for i, sub := range n.treeNode.SubObjects {
		var err error
		children[i], err = NewTreeNode(n.ctx, sub, n.objectRepo)
		if err != nil {
			return nil, err
		}
	}
	return children, nil
}

func (n TreeNode) NumChildren() (int, error) {
	treeNode, err := n.objectRepo.TreeNode(n.ctx, n.Hash())
	if err != nil {
		return 0, err
	}
	return len(treeNode.SubObjects), nil
}

func (n TreeNode) Skip() bool {
	return false
}
