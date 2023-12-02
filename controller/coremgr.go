package controller

import (
	"context"
	"fmt"
	"io"
	"os"
	"strings"

	"github.com/jiaozifs/jiaozifs/utils/pathutil"

	hash2 "github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/block"

	"github.com/jiaozifs/jiaozifs/models/filemode"

	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/models"
)

var (
	ErrPathNotFound = fmt.Errorf("path not found")
)

type CoreMgr struct {
	UserRepo   models.IUserRepo
	Repository models.RepositoryRepo
	Object     models.ObjectRepo
	Ref        models.RefRepo
}

func (core *CoreMgr) GetBlobByPath(ctx context.Context, treeId uuid.UUID, path string) (*models.Blob, *models.TreeEntry, error) {
	rootNode, err := core.Object.TreeNode(ctx, treeId)
	if err != nil {
		return nil, nil, err
	}

	fileSegs := strings.Split(path, "/")
	pathLen := len(fileSegs) - 1
	for index, seg := range fileSegs {
		for _, node := range rootNode.SubObject {
			if node.Name == seg {
				if index == pathLen {
					//last one
					if node.Mode == filemode.Regular || node.Mode == filemode.Executable {
						blob, err := core.Object.Blob(ctx, node.ID)
						if err != nil {
							return nil, nil, err
						}
						return blob, &node, nil
					}
					return nil, nil, ErrPathNotFound
				}
				if node.Mode != filemode.Dir {
					return nil, nil, ErrPathNotFound
				}
				rootNode, err = core.Object.TreeNode(ctx, treeId)
				if err != nil {
					return nil, nil, err
				}
				break
			}
			continue
		}
		return nil, nil, ErrPathNotFound
	}
	return nil, nil, ErrPathNotFound
}

func (core *CoreMgr) WriteBlob(ctx context.Context, adapter block.Adapter, bucketName string, body io.Reader, contentLength int64, opts block.PutOpts) (*models.Blob, error) {
	// handle the upload itself
	hashReader := hash2.NewHashingReader(body, hash2.HashFunctionMD5)
	hash := hash2.Hash(hashReader.Md5.Sum(nil))
	tempf, err := os.CreateTemp("", "*")
	if err != nil {
		return nil, err
	}
	_, err = io.Copy(tempf, body)
	if err != nil {
		return nil, err
	}

	_, err = tempf.Seek(io.SeekStart, 0)
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
		StorageNamespace: bucketName,
		IdentifierType:   block.IdentifierTypeRelative,
		Identifier:       address,
	}, contentLength, tempf, opts)
	if err != nil {
		return nil, err
	}

	return &models.Blob{
		PhysicalAddress: address,
		RelativePath:    true,
		Hash:            hash,
		Size:            hashReader.CopiedSize,
	}, nil
}

func (core *CoreMgr) ApplyChange(ctx context.Context, treeID uuid.UUID, changeMode models.Action, path string) (uuid.UUID, error) {
	treeNode, err := core.Object.TreeNode(ctx, treeID)
	if err != nil {
		return uuid.Nil, err
	}

	subDir := func(ctx context.Context, tn *models.TreeNode, name string) (*models.TreeNode, error) {
		for _, node := range tn.SubObject {
			if node.Name == name && node.Mode == filemode.Dir {
				return core.Object.TreeNode(ctx, node.ID)
			}
		}
		return nil, ErrPathNotFound
	}
	subFile := func(ctx context.Context, tn *models.TreeNode, name string) (*models.Blob, error) {
		for _, node := range tn.SubObject {
			if node.Name == name && (node.Mode == filemode.Regular || node.Mode == filemode.Executable) {
				return core.Object.Blob(ctx, node.ID)
			}
		}
		return nil, ErrPathNotFound
	}

	fileSegs := strings.Split(path, "/")
	pathLen := len(fileSegs) - 1

	for index, seg := range fileSegs {
		if index == pathLen {
			blob, err := subFile(ctx, treeNode, seg)
			if err != nil {
				return uuid.Nil, err
			}

		} else {
			treeNode, err = subDir(ctx, treeNode, seg)
			if err != nil {
				return uuid.Nil, err
			}
		}
		return uuid.Nil, ErrPathNotFound
	}
	return uuid.Nil, ErrPathNotFound
}
