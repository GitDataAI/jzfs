package ipfs

import (
	"context"
	"crypto/md5" //nolint:gosec
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"path"
	"strconv"
	"strings"
	"time"

	"github.com/modern-go/reflect2"

	"github.com/ipfs/kubo/core/coreiface/options"

	"github.com/ipfs/boxo/files"

	"github.com/google/uuid"
	"github.com/ipfs/kubo/client/rpc"
	"github.com/GitDataAI/jiaozifs/block"
	"github.com/GitDataAI/jiaozifs/utils/hash"
	ma "github.com/multiformats/go-multiaddr"
)

const DefaultNamespacePrefix = block.BlockstoreIPFS + "://"

type Adapter struct {
	url            string
	client         *rpc.HttpApi
	removeEmptyDir bool
}

var (
	ErrPathNotWritable         = errors.New("path provided is not writable")
	ErrInvalidUploadIDFormat   = errors.New("invalid upload id format")
	ErrBadPath                 = errors.New("bad path traversal blocked")
	ErrInvalidStorageNamespace = errors.New("invalid storageNamespace")
)

type QualifiedKey struct {
	block.CommonQualifiedKey
	url string
}

func (qk QualifiedKey) Format() string {
	p := path.Join(qk.url, qk.GetStorageNamespace(), qk.GetKey())
	return qk.GetStorageType().Scheme() + "://" + p
}

func (qk QualifiedKey) GetStorageType() block.StorageType {
	return qk.CommonQualifiedKey.GetStorageType()
}

func (qk QualifiedKey) GetStorageNamespace() string {
	return qk.CommonQualifiedKey.GetStorageNamespace()
}

func (qk QualifiedKey) GetKey() string {
	return qk.CommonQualifiedKey.GetKey()
}

func NewAdapter(url string, opts ...func(a *Adapter)) (*Adapter, error) {
	addr, err := ma.NewMultiaddr(strings.TrimSpace(url))
	if err != nil {
		return nil, err
	}

	client, err := rpc.NewApi(addr)
	if err != nil {
		return nil, err
	}

	localAdapter := &Adapter{
		url:            url,
		client:         client,
		removeEmptyDir: true,
	}
	for _, opt := range opts {
		opt(localAdapter)
	}
	return localAdapter, nil
}

func (l *Adapter) GetPreSignedURL(_ context.Context, _ block.ObjectPointer, _ block.PreSignMode) (string, time.Time, error) {
	return "", time.Time{}, fmt.Errorf("ipfs adapter presigned URL: %w", block.ErrOperationNotSupported)
}

func (l *Adapter) extractParamsFromObj(ptr block.ObjectPointer) (string, string, error) {
	if strings.HasPrefix(ptr.Identifier, DefaultNamespacePrefix) {
		// check abs path
		p := ptr.Identifier[len(DefaultNamespacePrefix):]
		seqs := strings.SplitN(p, "/", 2)
		if len(seqs) != 2 {
			return "", "", fmt.Errorf("seqs must start with namespace")
		}
		return seqs[0], seqs[1], nil
	}

	if !strings.HasPrefix(ptr.StorageNamespace, DefaultNamespacePrefix) {
		return "", "", fmt.Errorf("%w: storage namespace", ErrBadPath)
	}
	return ptr.StorageNamespace[len(DefaultNamespacePrefix):], ptr.Identifier, nil
}

func (l *Adapter) extractNamespace(storageNamespace string) (string, error) {
	if !strings.HasPrefix(storageNamespace, DefaultNamespacePrefix) {
		return "", fmt.Errorf("%w: storage namespace", ErrBadPath)
	}
	return storageNamespace[len(DefaultNamespacePrefix):], nil
}

func (l *Adapter) ensureNamespace(ctx context.Context, namespace string) error {
	err := l.client.Unixfs().Mkdir(ctx, "/"+namespace)
	if !reflect2.IsNil(err) {
		if !strings.Contains(err.Error(), "file already exists") {
			return err
		}
	}
	return nil
}

func (l *Adapter) Put(ctx context.Context, obj block.ObjectPointer, _ int64, reader io.Reader, _ block.PutOpts) error {
	namespace, identify, err := l.extractParamsFromObj(obj)
	if err != nil {
		return err
	}

	err = l.ensureNamespace(ctx, namespace)
	if err != nil {
		return err
	}

	ipfsPath, err := l.client.Unixfs().Add(ctx, files.NewReaderFile(reader))
	if !reflect2.IsNil(err) {
		return err
	}
	//copy to mfs
	err = l.client.Unixfs().Cp(ctx, ipfsPath.String(), fullPath(namespace, identify), options.Unixfs.CpParents(true))
	if !reflect2.IsNil(err) {
		if strings.Contains(err.Error(), "already has entry") { // already exists
			return nil
		}
		return err
	}
	return nil
}

func (l *Adapter) Remove(ctx context.Context, obj block.ObjectPointer) error {
	namespace, identify, err := l.extractParamsFromObj(obj)
	if err != nil {
		return err
	}

	err = l.client.Unixfs().Rm(ctx, fullPath(namespace, identify))
	if !reflect2.IsNil(err) {
		return err
	}
	return nil
}

func (l *Adapter) RemoveNameSpace(ctx context.Context, storageNamespace string) error {
	namespace, err := l.extractNamespace(storageNamespace)
	if err != nil {
		return err
	}

	err = l.client.Unixfs().Rm(ctx, fmt.Sprintf("/%s", namespace), options.Unixfs.Recursive(true), options.Unixfs.Force(true))
	if !reflect2.IsNil(err) {
		return err
	}
	return nil
}

func (l *Adapter) Copy(ctx context.Context, sourceObj, destinationObj block.ObjectPointer) error {
	namespace, _, err := l.extractParamsFromObj(sourceObj)
	if err != nil {
		return err
	}

	err = l.client.Unixfs().Cp(ctx,
		fmt.Sprintf("/%s/%s", namespace, sourceObj.Identifier),
		fmt.Sprintf("/%s/%s", namespace, destinationObj.Identifier),
		options.Unixfs.CpParents(true),
	)
	if !reflect2.IsNil(err) {
		return err
	}
	return nil
}

func (l *Adapter) UploadCopyPart(ctx context.Context, sourceObj, destinationObj block.ObjectPointer, uploadID string, partNumber int) (*block.UploadPartResponse, error) {
	if err := isValidUploadID(uploadID); err != nil {
		return nil, err
	}
	r, err := l.Get(ctx, sourceObj, 0)
	if err != nil {
		return nil, err
	}
	md5Read := hash.NewHashingReader(r, hash.Md5)
	fName := uploadID + fmt.Sprintf("-%05d", partNumber)
	err = l.Put(ctx, block.ObjectPointer{StorageNamespace: destinationObj.StorageNamespace, Identifier: fName}, -1, md5Read, block.PutOpts{})
	if err != nil {
		return nil, err
	}
	etag := hex.EncodeToString(md5Read.Md5.Sum(nil))
	return &block.UploadPartResponse{
		ETag: etag,
	}, nil
}

func (l *Adapter) UploadCopyPartRange(ctx context.Context, sourceObj, destinationObj block.ObjectPointer, uploadID string, partNumber int, startPosition, endPosition int64) (*block.UploadPartResponse, error) {
	if err := isValidUploadID(uploadID); err != nil {
		return nil, err
	}
	r, err := l.GetRange(ctx, sourceObj, startPosition, endPosition)
	if err != nil {
		return nil, err
	}
	md5Read := hash.NewHashingReader(r, hash.Md5)
	fName := uploadID + fmt.Sprintf("-%05d", partNumber)
	err = l.Put(ctx, block.ObjectPointer{StorageNamespace: destinationObj.StorageNamespace, Identifier: fName}, -1, md5Read, block.PutOpts{})
	if err != nil {
		return nil, err
	}
	etag := hex.EncodeToString(md5Read.Md5.Sum(nil))
	return &block.UploadPartResponse{
		ETag: etag,
	}, err
}

func (l *Adapter) Get(ctx context.Context, obj block.ObjectPointer, _ int64) (io.ReadCloser, error) {
	namespace, identify, err := l.extractParamsFromObj(obj)
	if err != nil {
		return nil, err
	}

	rc, err := l.client.Unixfs().Read(ctx, fmt.Sprintf("/%s/%s", namespace, identify))
	if !reflect2.IsNil(err) {
		return nil, err
	}
	return rc, nil
}

func (l *Adapter) GetWalker(uri *url.URL) (block.Walker, error) {
	if err := block.ValidateStorageType(uri, block.StorageTypeLocal); err != nil {
		return nil, err
	}

	return NewIPFSWalker(l.client), nil
}

func (l *Adapter) Exists(ctx context.Context, obj block.ObjectPointer) (bool, error) {
	namespace, identify, err := l.extractParamsFromObj(obj)
	if err != nil {
		return false, err
	}

	_, err = l.client.Unixfs().Stat(ctx, fmt.Sprintf("/%s/%s", namespace, identify))
	if !reflect2.IsNil(err) {
		return false, err
	}
	return true, nil
}

func (l *Adapter) GetRange(ctx context.Context, obj block.ObjectPointer, start int64, end int64) (io.ReadCloser, error) {
	if start < 0 || end < start {
		return nil, block.ErrBadIndex
	}
	namespace, identify, err := l.extractParamsFromObj(obj)
	if err != nil {
		return nil, err
	}
	rc, err := l.client.Unixfs().Read(ctx, fmt.Sprintf("/%s/%s", namespace, identify), options.Unixfs.Offset(start), options.Unixfs.Count(end-start))
	if !reflect2.IsNil(err) {
		return nil, err
	}
	return rc, nil
}

func (l *Adapter) GetProperties(_ context.Context, _ block.ObjectPointer) (block.Properties, error) {
	return block.Properties{}, nil
}

func (l *Adapter) CreateMultiPartUpload(ctx context.Context, obj block.ObjectPointer, _ *http.Request, _ block.CreateMultiPartUploadOpts) (*block.CreateMultiPartUploadResponse, error) {
	if strings.Contains(obj.Identifier, "/") {
		namespace, _, err := l.extractParamsFromObj(obj)
		if err != nil {
			return nil, err
		}
		err = l.ensureNamespace(ctx, namespace)
		if err != nil {
			return nil, err
		}
	}
	uidBytes := uuid.New()
	uploadID := hex.EncodeToString(uidBytes[:])
	return &block.CreateMultiPartUploadResponse{
		UploadID: uploadID,
	}, nil
}

func (l *Adapter) UploadPart(ctx context.Context, obj block.ObjectPointer, _ int64, reader io.Reader, uploadID string, partNumber int) (*block.UploadPartResponse, error) {
	if err := isValidUploadID(uploadID); err != nil {
		return nil, err
	}
	md5Read := hash.NewHashingReader(reader, hash.Md5)
	fName := uploadID + fmt.Sprintf("-%05d", partNumber)
	err := l.Put(ctx, block.ObjectPointer{StorageNamespace: obj.StorageNamespace, Identifier: fName}, -1, md5Read, block.PutOpts{})
	etag := hex.EncodeToString(md5Read.Md5.Sum(nil))
	return &block.UploadPartResponse{
		ETag: etag,
	}, err
}

func (l *Adapter) AbortMultiPartUpload(_ context.Context, _ block.ObjectPointer, uploadID string) error {
	if err := isValidUploadID(uploadID); err != nil {
		return err
	}

	panic("todo")
}

func (l *Adapter) CompleteMultiPartUpload(_ context.Context, obj block.ObjectPointer, uploadID string, multipartList *block.MultipartUploadCompletion) (*block.CompleteMultiPartUploadResponse, error) {
	if err := isValidUploadID(uploadID); err != nil {
		return nil, err
	}

	size, err := l.unitePartFiles(obj, uploadID)
	if err != nil {
		return nil, fmt.Errorf("multipart upload unite for %s: %w", uploadID, err)
	}
	return &block.CompleteMultiPartUploadResponse{
		ETag:          computeETag(multipartList.Part) + "-" + strconv.Itoa(len(multipartList.Part)),
		ContentLength: size,
	}, nil
}

func computeETag(parts []block.MultipartPart) string {
	var etagHex []string
	for _, p := range parts {
		e := strings.Trim(p.ETag, `"`)
		etagHex = append(etagHex, e)
	}
	s := strings.Join(etagHex, "")
	b, _ := hex.DecodeString(s)
	md5res := md5.Sum(b) //nolint:gosec
	csm := hex.EncodeToString(md5res[:])
	return csm
}

func (l *Adapter) unitePartFiles(_ block.ObjectPointer, _ string) (int64, error) {
	panic("not impl")
}

func (l *Adapter) removePartFiles(ctx context.Context, files []string) error { //nolint
	var firstErr error
	for _, name := range files {
		// If removal fails prefer to skip the error: "only" wasted space.
		_ = l.client.Unixfs().Rm(ctx, name, options.Unixfs.Force(true))
	}
	return firstErr
}

func (l *Adapter) BlockstoreType() string {
	return block.BlockstoreIPFS
}

func (l *Adapter) GetStorageNamespaceInfo() block.StorageNamespaceInfo {
	info := block.DefaultStorageNamespaceInfo(block.BlockstoreIPFS)
	info.PreSignSupport = false
	info.DefaultNamespacePrefix = DefaultNamespacePrefix
	info.ImportSupport = true
	return info
}

func (l *Adapter) ResolveNamespace(storageNamespace, key string, identifierType block.IdentifierType) (block.QualifiedKey, error) {
	qk, err := block.DefaultResolveNamespace(storageNamespace, key, identifierType)
	if err != nil {
		return nil, err
	}

	return QualifiedKey{
		CommonQualifiedKey: qk,
		url:                l.url,
	}, nil
}

func (l *Adapter) RuntimeStats() map[string]string {
	return nil
}

func isValidUploadID(uploadID string) error {
	_, err := hex.DecodeString(uploadID)
	if err != nil {
		return fmt.Errorf("%w: %s", ErrInvalidUploadIDFormat, err)
	}
	return nil
}

func fullPath(namespace, identify string) string {
	return fmt.Sprintf("/%s/%s", namespace, identify)
}
