package versionmgr

import (
	"archive/zip"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	path2 "path"
	"path/filepath"
	"strings"

	chunker "github.com/ipfs/boxo/chunker"

	"github.com/ipfs/go-cid"

	"github.com/GitDataAI/jiaozifs/models"
	bserv "github.com/ipfs/boxo/blockservice"
	bstore "github.com/ipfs/boxo/blockstore"
	dag "github.com/ipfs/boxo/ipld/merkledag"
	ft "github.com/ipfs/boxo/ipld/unixfs"
	"github.com/ipfs/boxo/mfs"
	ds "github.com/ipfs/go-datastore"
	dssync "github.com/ipfs/go-datastore/sync"
	offline "github.com/ipfs/go-ipfs-exchange-offline"
	car "github.com/ipld/go-car"

	importer "github.com/ipfs/boxo/ipld/unixfs/importer"
)

type RepoArchiver struct {
	rootPath  string
	walker    IWalk
	getReader func(context.Context, *models.Blob, string) (io.ReadCloser, error)
}

func NewRepoArchiver(rootPath string, walker IWalk, getReader func(context.Context, *models.Blob, string) (io.ReadCloser, error)) *RepoArchiver {
	return &RepoArchiver{rootPath: rootPath, walker: walker, getReader: getReader}
}

func (repo *RepoArchiver) ArchiveZip(ctx context.Context, dest string) error {
	zipFile, err := os.Create(dest)
	if err != nil {
		return err
	}
	defer zipFile.Close() //nolint:errcheck

	zipWriter := zip.NewWriter(zipFile)
	defer zipWriter.Close() //nolint:errcheck

	_, err = zipWriter.CreateHeader(&zip.FileHeader{
		Name: repo.rootPath + "/",
	})
	if err != nil {
		return err
	}

	return repo.walker.Walk(ctx, func(entry *models.TreeEntry, blob *models.Blob, path string) error {
		if entry.IsDir {
			path = fmt.Sprintf("%s%c", path, os.PathSeparator)
			_, err = zipWriter.CreateHeader(&zip.FileHeader{
				Name: path2.Join(repo.rootPath, path) + "/",
			})
			return err
		}

		reader, err := repo.getReader(ctx, blob, path)
		if err != nil {
			return err
		}
		defer reader.Close() //nolint

		f, err := zipWriter.Create(path2.Join(repo.rootPath, path))
		if err != nil {
			return err
		}

		_, err = io.Copy(f, reader)
		return err
	})
}

func (repo *RepoArchiver) ArchiveCar(ctx context.Context, dest string) error {
	db := dssync.MutexWrap(ds.NewMapDatastore()) //todo use disk to cache data
	bs := bstore.NewBlockstore(db)
	blockSrv := bserv.New(bs, offline.Exchange(bs))
	dagSrv := dag.NewDAGService(blockSrv)
	rootNode := dag.NodeWithData(ft.FolderPBData())

	root, err := mfs.NewRoot(ctx, dagSrv, rootNode, nil)
	if err != nil {
		return err
	}
	defer root.Close() //nolint:errcheck

	rootDir := root.GetDirectory()
	err = repo.walker.Walk(ctx, func(entry *models.TreeEntry, blob *models.Blob, treePath string) error {
		path := path2.Join(repo.rootPath, treePath)
		if entry.IsDir {
			_, err = mkdirP(rootDir, path)
			return err
		}

		reader, err := repo.getReader(ctx, blob, treePath)
		if err != nil {
			return err
		}
		defer reader.Close() //nolint

		dir := filepath.Dir(path)
		base := filepath.Base(path)
		dirNd, err := mfs.Lookup(root, dir)
		if err != nil {
			return err
		}

		nd, err := importer.BuildDagFromReader(dagSrv, chunker.DefaultSplitter(reader))
		if err != nil {
			return err
		}
		return dirNd.(*mfs.Directory).AddChild(base, nd)
	})
	if err != nil {
		return err
	}
	err = root.Flush()
	if err != nil {
		return err
	}

	ipldNode, err := rootDir.GetNode()
	if err != nil {
		return err
	}

	carFile, err := os.Create(dest)
	if err != nil {
		return err
	}
	defer carFile.Close() //nolint:errcheck
	return car.WriteCar(ctx, dagSrv, []cid.Cid{ipldNode.Cid()}, carFile)
}

func mkdirP(root *mfs.Directory, pth string) (*mfs.Directory, error) {
	dirs := strings.Split(pth, "/")
	cur := root
	for _, d := range dirs {
		n, err := cur.Mkdir(d)
		if err != nil && !errors.Is(err, os.ErrExist) {
			return nil, err
		}
		if errors.Is(err, os.ErrExist) {
			fsn, err := cur.Child(d)
			if err != nil {
				return nil, err
			}
			switch fsn := fsn.(type) {
			case *mfs.Directory:
				n = fsn
			case *mfs.File:
				return nil, errors.New("find file")
			}
		}

		cur = n
	}
	return cur, nil
}
