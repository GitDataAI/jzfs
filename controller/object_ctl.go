package controller

import (
	"context"
	"fmt"
	"io"
	"mime"
	"mime/multipart"
	"net/http"
	"time"

	"github.com/go-openapi/swag"
	"github.com/jiaozifs/jiaozifs/models/filemode"

	"github.com/jiaozifs/jiaozifs/versionmgr"

	"github.com/jiaozifs/jiaozifs/block"

	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/utils/httputil"

	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/api"
	"go.uber.org/fx"
)

type ObjectController struct {
	fx.In

	BlockAdapter block.Adapter

	Repo models.IRepo
}

func (oct ObjectController) DeleteObject(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, user string, repository string, params api.DeleteObjectParams) { //nolint
	//TODO implement me
	panic("implement me")
}

func (oct ObjectController) GetObject(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, user string, repository string, params api.GetObjectParams) { //nolint
	//TODO implement me
	panic("implement me")
}

func (oct ObjectController) HeadObject(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, userName string, repository string, params api.HeadObjectParams) { //nolint
	user, err := oct.Repo.UserRepo().Get(ctx, &models.GetUserParam{Name: utils.String(userName)})
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := oct.Repo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
		CreateID: user.ID,
		Name:     utils.String(repository),
	})
	if err != nil {
		w.Error(err)
		return
	}

	ref, err := oct.Repo.RefRepo().Get(ctx, &models.GetRefParams{
		RepositoryID: repo.ID,
		Name:         utils.String(params.Branch),
	})
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := oct.Repo.ObjectRepo().Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	objRepo := oct.Repo.ObjectRepo()
	treeOp, err := versionmgr.NewWorkTree(ctx, oct.Repo.ObjectRepo(), models.NewRootTreeEntry(commit.TreeHash))
	if err != nil {
		w.Error(err)
		return
	}

	existNodes, missingPath, err := treeOp.MatchPath(ctx, params.Path)
	if err != nil {
		w.Error(err)
		return
	}
	if len(missingPath) == 0 {
		w.Error(versionmgr.ErrPathNotFound)
		return
	}

	objectWithName := existNodes[len(existNodes)-1]

	blob, err := objRepo.Blob(ctx, objectWithName.Node().Hash)
	if err != nil {
		w.Error(err)
		return
	}

	//lookup files
	etag := httputil.ETag(objectWithName.Node().Hash.Hex())
	w.Header().Set("ETag", etag)
	lastModified := httputil.HeaderTimestamp(objectWithName.Node().CreatedAt)
	w.Header().Set("Last-Modified", lastModified)
	w.Header().Set("Accept-Ranges", "bytes")
	w.Header().Set("Content-Type", httputil.ExtensionsByType(objectWithName.Entry().Name))
	// for security, make sure the browser and any proxies en route don't cache the response
	w.Header().Set("Cache-Control", "no-store, must-revalidate")
	w.Header().Set("Expires", "0")

	// calculate possible byte range, if any.
	if params.Range != nil {
		rng, err := httputil.ParseRange(*params.Range, blob.Size)
		if err != nil {
			w.CodeMsg(http.StatusRequestedRangeNotSatisfiable, "")
			return
		}
		w.Header().Set("Content-Range", fmt.Sprintf("bytes %d-%d/%d", rng.StartOffset, rng.EndOffset, blob.Size))
		w.Header().Set("Content-Length", fmt.Sprintf("%d", rng.EndOffset-rng.StartOffset+1))
		w.CodeMsg(http.StatusPartialContent, "")
	} else {
		w.Header().Set("Content-Length", fmt.Sprint(blob.Size))
	}
}

func (oct ObjectController) UploadObject(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, userName string, repository string, params api.UploadObjectParams) { //nolint
	// read request body parse multipart for "content" and upload the data
	contentType := r.Header.Get("Content-Type")
	mediaType, p, err := mime.ParseMediaType(contentType)
	if err != nil {
		w.Error(err)
		return
	}

	reader := r.Body
	if mediaType == "multipart/form-data" {
		// handle multipart upload
		boundary, ok := p["boundary"]
		if !ok {
			w.Error(err)
			return
		}

		contentUploaded := false
		partReader := multipart.NewReader(r.Body, boundary)
		for !contentUploaded {
			part, err := partReader.NextPart()
			if err == io.EOF {
				break
			}
			if err != nil {
				w.Error(err)
				return
			}
			contentType = part.Header.Get("Content-Type")
			partName := part.FormName()
			if partName == "content" {
				reader = part
				contentUploaded = true
			} else { //close not target part
				_ = part.Close()
			}

		}
		if !contentUploaded {
			w.Error(fmt.Errorf("multipart upload missing key 'content': %w", http.ErrMissingFile))
			return
		}
	}
	defer reader.Close() //nolint

	var response api.ObjectStats
	err = oct.Repo.Transaction(ctx, func(dRepo models.IRepo) error {
		user, err := dRepo.UserRepo().Get(ctx, &models.GetUserParam{Name: utils.String(userName)})
		if err != nil {
			return err
		}

		repo, err := dRepo.RepositoryRepo().Get(ctx, &models.GetRepoParams{
			CreateID: user.ID,
			Name:     utils.String(repository),
		})
		if err != nil {
			return err
		}

		stash, err := dRepo.WipRepo().Get(ctx, &models.GetWipParam{
			RepositoryID: repo.ID,
			CreateID:     user.ID,
		})
		if err != nil {
			return err
		}

		workingTree, err := versionmgr.NewWorkTree(ctx, dRepo.ObjectRepo(), models.NewRootTreeEntry(stash.CurrentTree))
		if err != nil {
			return err
		}

		blob, err := workingTree.WriteBlob(ctx, oct.BlockAdapter, reader, r.ContentLength, block.PutOpts{})
		if err != nil {
			return err
		}

		err = workingTree.AddLeaf(ctx, params.Path, blob)
		if err != nil {
			return err
		}
		response = api.ObjectStats{
			Checksum:    blob.Hash.Hex(),
			Mtime:       time.Now().Unix(),
			Path:        params.Path,
			PathMode:    utils.Uint32(uint32(filemode.Regular)),
			SizeBytes:   swag.Int64(blob.Size),
			ContentType: &contentType,
			Metadata:    &api.ObjectUserMetadata{},
		}
		return dRepo.WipRepo().UpdateCurrentHash(ctx, stash.ID, workingTree.Root().Hash())
	})

	if err != nil {
		w.Error(err)
		return
	}

	w.JSON(response)
}
