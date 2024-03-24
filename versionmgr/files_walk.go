package versionmgr

import (
	"container/list"
	"context"
	"errors"
	"path"

	"github.com/GitDataAI/jiaozifs/models"
)

var ErrHalt = errors.New("halt walk")

type IWalk interface {
	Walk(ctx context.Context, fn func(entry *models.TreeEntry, blob *models.Blob, path string) error) error
}

type FileWalk struct {
	object  models.IFileTreeRepo
	curNode *TreeNode
}

func NewFileWalk(object models.IFileTreeRepo, curNode *TreeNode) *FileWalk {
	return &FileWalk{object: object, curNode: curNode}
}

type nodeWithPath struct {
	curNode *TreeNode
	path    string
}

func (wk FileWalk) Walk(ctx context.Context, fn func(entry *models.TreeEntry, blob *models.Blob, path string) error) error {
	cache := list.New()
	cache.PushFront(nodeWithPath{wk.curNode, ""})
	for {
		if cache.Len() == 0 {
			break
		}
		curNode := cache.Front().Value.(nodeWithPath)
		cache.Remove(cache.Front())
		subNodes := curNode.curNode.SubObjects()
		for i := len(subNodes); i > 0; i-- {
			if !subNodes[i-1].IsDir {
				continue
			}
			treeNode, err := NewTreeNode(ctx, subNodes[i-1], wk.object)
			if err != nil {
				return err
			}

			cache.PushFront(nodeWithPath{treeNode, path.Join(curNode.path, treeNode.Name())})
		}
		for i := 0; i < len(subNodes); i++ {
			if subNodes[i].IsDir {
				err := fn(&subNodes[i], nil, path.Join(curNode.path, subNodes[i].Name))
				if err != nil {
					return err
				}
				continue
			}

			blob, err := wk.object.Blob(ctx, subNodes[i].Hash)
			if err != nil {
				return err
			}

			err = fn(&subNodes[i], blob, path.Join(curNode.path, subNodes[i].Name))
			if err != nil {
				return err
			}
		}
	}

	return nil
}
