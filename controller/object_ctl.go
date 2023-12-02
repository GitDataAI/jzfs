package controller

import (
	"context"
	"fmt"
	"io"
	"mime"
	"mime/multipart"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/models/filemode"

	"github.com/go-openapi/swag"

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

	Core       *CoreMgr
	StashRepo  models.StashRepo
	UserRepo   models.IUserRepo
	Repository models.RepositoryRepo
	Object     models.ObjectRepo
	Ref        models.RefRepo
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
	user, err := oct.UserRepo.GetByName(ctx, userName)
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := oct.Repository.Get(ctx, &models.GetRepoParams{
		CreateID: user.ID,
		Name:     utils.String(repository),
	})
	if err != nil {
		w.Error(err)
		return
	}

	ref, err := oct.Ref.Get(ctx, &models.GetRefParams{
		RepositoryID: repo.ID,
		Name:         utils.String(params.Branch),
	})
	if err != nil {
		w.Error(err)
		return
	}

	commit, err := oct.Object.Commit(ctx, ref.CommitHash)
	if err != nil {
		w.Error(err)
		return
	}

	blob, treeEntry, err := oct.Core.GetBlobByPath(ctx, commit.ID, params.Path)
	if err != nil {
		w.Error(err)
		return
	}

	//lookup files
	etag := httputil.ETag(blob.Hash.Hex())
	w.Header().Set("ETag", etag)
	lastModified := httputil.HeaderTimestamp(blob.CreatedAt)
	w.Header().Set("Last-Modified", lastModified)
	w.Header().Set("Accept-Ranges", "bytes")
	w.Header().Set("Content-Type", httputil.ExtensionsByType(treeEntry.Name))
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
	user, err := oct.UserRepo.GetByName(ctx, userName)
	if err != nil {
		w.Error(err)
		return
	}

	repo, err := oct.Repository.Get(ctx, &models.GetRepoParams{
		CreateID: user.ID,
		Name:     utils.String(repository),
	})
	if err != nil {
		w.Error(err)
		return
	}

	// read request body parse multipart for "content" and upload the data
	contentType := r.Header.Get("Content-Type")
	mediaType, p, err := mime.ParseMediaType(contentType)
	if err != nil {
		w.Error(err)
		return
	}

	var blob *models.Blob
	if mediaType != "multipart/form-data" {
		// handle non-multipart, direct content upload
		blob, err = oct.Core.WriteBlob(ctx, oct.BlockAdapter, repo.StorageNamespace, r.Body, r.ContentLength, block.PutOpts{})
		if err != nil {
			w.Error(err)
			return
		}
	} else {
		// handle multipart upload
		boundary, ok := p["boundary"]
		if !ok {
			w.Error(err)
			return
		}

		contentUploaded := false
		reader := multipart.NewReader(r.Body, boundary)
		for !contentUploaded {
			part, err := reader.NextPart()
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
				// upload the first "content" and exit the loop
				blob, err = oct.Core.WriteBlob(ctx, oct.BlockAdapter, repo.StorageNamespace, part, -1, block.PutOpts{})
				if err != nil {
					_ = part.Close()
					w.Error(err)
					return
				}
				contentUploaded = true
			}
			_ = part.Close()
		}
		if !contentUploaded {
			err := fmt.Errorf("multipart upload missing key 'content': %w", http.ErrMissingFile)
			w.Error(err)
			return
		}
	}

	stash, err := oct.StashRepo.Get(ctx, &models.GetStashParam{
		RepositoryID: repo.ID,
		CreateID:     user.ID,
	})
	if err != nil {
		w.Error(err)
		return
	}

	//apply change to stash
	//todo write block to stash
	identifierType := block.IdentifierTypeFull
	if blob.RelativePath {
		identifierType = block.IdentifierTypeRelative
	}

	qk, err := oct.BlockAdapter.ResolveNamespace(repo.StorageNamespace, blob.PhysicalAddress, identifierType)
	if err != nil {
		w.Error(err)
		return
	}

	response := api.ObjectStats{
		Checksum:        blob.Hash.Hex(),
		Mtime:           time.Now().Unix(),
		Path:            params.Path,
		PathMode:        utils.Uint32(uint32(filemode.Regular)),
		PhysicalAddress: qk.Format(),
		SizeBytes:       swag.Int64(blob.Size),
		ContentType:     &contentType,
		Metadata:        &api.ObjectUserMetadata{},
	}
	w.JSON(response)
}
