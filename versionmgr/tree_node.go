package versionmgr

import (
	"context"

	"github.com/go-git/go-git/v5/utils/merkletrie/noder"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/filemode"
)

var _ noder.Noder = (*TreeNode)(nil)

type TreeNode struct {
	Ctx context.Context
	models.TreeEntry
	Object models.IObjectRepo
}

func (n TreeNode) Hash() []byte {
	return n.TreeEntry.Hash
}

func (n TreeNode) String() string {
	return n.TreeEntry.Name + " " + n.TreeEntry.Hash.Hex()
}

func (n TreeNode) Name() string {
	return n.TreeEntry.Name
}

func (n TreeNode) IsDir() bool {
	return n.TreeEntry.Mode == filemode.Dir
}

func (n TreeNode) Children() ([]noder.Noder, error) {
	treeNode, err := n.Object.TreeNode(n.Ctx, n.Hash())
	if err != nil {
		return nil, err
	}
	children := make([]noder.Noder, len(treeNode.SubObjects))
	for i, sub := range treeNode.SubObjects {
		children[i] = TreeNode{
			Ctx:       n.Ctx,
			TreeEntry: sub,
			Object:    n.Object,
		}
	}
	return children, nil
}

func (n TreeNode) NumChildren() (int, error) {
	treeNode, err := n.Object.TreeNode(n.Ctx, n.Hash())
	if err != nil {
		return 0, err
	}
	return len(treeNode.SubObjects), nil
}

func (n TreeNode) Skip() bool {
	return false
}
