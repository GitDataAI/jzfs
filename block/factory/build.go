package factory

import (
	"context"
	"fmt"

	"cloud.google.com/go/storage"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/block"
	"github.com/jiaozifs/jiaozifs/block/azure"
	"github.com/jiaozifs/jiaozifs/block/gs"
	"github.com/jiaozifs/jiaozifs/block/local"
	"github.com/jiaozifs/jiaozifs/block/mem"
	"github.com/jiaozifs/jiaozifs/block/params"
	s3a "github.com/jiaozifs/jiaozifs/block/s3"
	"github.com/jiaozifs/jiaozifs/block/transient"
	"golang.org/x/oauth2/google"
	"google.golang.org/api/option"
)

var log = logging.Logger("block_factory")

const (
	// googleAuthCloudPlatform - Cloud Storage authentication https://cloud.google.com/storage/docs/authentication
	googleAuthCloudPlatform = "https://www.googleapis.com/auth/cloud-platform"
)

type BlockAdapterBuilder = func(context.Context, params.AdapterConfig) (block.Adapter, error)

func BuildBlockAdapter(ctx context.Context, c params.AdapterConfig) (block.Adapter, error) {
	blockstore := c.BlockstoreType()
	log.With("type", blockstore).
		Info("initialize blockstore adapter")
	switch blockstore {
	case block.BlockstoreTypeLocal:
		p, err := c.BlockstoreLocalParams()
		if err != nil {
			return nil, err
		}
		return buildLocalAdapter(ctx, p)
	case block.BlockstoreTypeS3:
		p, err := c.BlockstoreS3Params()
		if err != nil {
			return nil, err
		}
		return buildS3Adapter(ctx, p)
	case block.BlockstoreTypeMem, "memory":
		return mem.New(ctx), nil
	case block.BlockstoreTypeTransient:
		return transient.New(ctx), nil
	case block.BlockstoreTypeGS:
		p, err := c.BlockstoreGSParams()
		if err != nil {
			return nil, err
		}
		return buildGSAdapter(ctx, p)
	case block.BlockstoreTypeAzure:
		p, err := c.BlockstoreAzureParams()
		if err != nil {
			return nil, err
		}
		return azure.NewAdapter(ctx, p)
	default:
		return nil, fmt.Errorf("%w '%s' please choose one of %s",
			block.ErrInvalidAddress, blockstore, []string{block.BlockstoreTypeLocal, block.BlockstoreTypeS3, block.BlockstoreTypeAzure, block.BlockstoreTypeMem, block.BlockstoreTypeTransient, block.BlockstoreTypeGS})
	}
}

func buildLocalAdapter(_ context.Context, params params.Local) (*local.Adapter, error) {
	adapter, err := local.NewAdapter(params.Path,
		local.WithAllowedExternalPrefixes(params.AllowedExternalPrefixes),
		local.WithImportEnabled(params.ImportEnabled),
	)
	if err != nil {
		return nil, fmt.Errorf("got error opening a local block adapter with path %s: %w", params.Path, err)
	}
	log.With(
		"type", "local",
		"path", params.Path,
	).Info("initialized blockstore adapter")
	return adapter, nil
}

func BuildS3Client(ctx context.Context, params params.S3) (*s3.Client, error) {
	cfg, err := s3a.LoadConfig(ctx, params)
	if err != nil {
		return nil, err
	}

	client := s3.NewFromConfig(cfg, s3a.WithClientParams(params))
	return client, nil
}

func buildS3Adapter(ctx context.Context, params params.S3) (*s3a.Adapter, error) {
	opts := []s3a.AdapterOption{
		s3a.WithDiscoverBucketRegion(params.DiscoverBucketRegion),
		s3a.WithPreSignedExpiry(params.PreSignedExpiry),
		s3a.WithDisablePreSigned(params.DisablePreSigned),
		s3a.WithDisablePreSignedUI(params.DisablePreSignedUI),
	}
	if params.ServerSideEncryption != "" {
		opts = append(opts, s3a.WithServerSideEncryption(params.ServerSideEncryption))
	}
	if params.ServerSideEncryptionKmsKeyID != "" {
		opts = append(opts, s3a.WithServerSideEncryptionKmsKeyID(params.ServerSideEncryptionKmsKeyID))
	}
	adapter, err := s3a.NewAdapter(ctx, params, opts...)
	if err != nil {
		return nil, err
	}
	log.With("type", "s3").Info("initialized blockstore adapter")
	return adapter, nil
}

func BuildGSClient(ctx context.Context, params params.GS) (*storage.Client, error) {
	var opts []option.ClientOption
	if params.CredentialsFile != "" {
		opts = append(opts, option.WithCredentialsFile(params.CredentialsFile))
	} else if params.CredentialsJSON != "" {
		cred, err := google.CredentialsFromJSON(ctx, []byte(params.CredentialsJSON), googleAuthCloudPlatform)
		if err != nil {
			return nil, err
		}
		opts = append(opts, option.WithCredentials(cred))
	}
	return storage.NewClient(ctx, opts...)
}

func buildGSAdapter(ctx context.Context, params params.GS) (*gs.Adapter, error) {
	client, err := BuildGSClient(ctx, params)
	if err != nil {
		return nil, err
	}
	adapter := gs.NewAdapter(client,
		gs.WithPreSignedExpiry(params.PreSignedExpiry),
		gs.WithDisablePreSigned(params.DisablePreSigned),
		gs.WithDisablePreSignedUI(params.DisablePreSignedUI),
	)
	log.With("type", "gs").Info("initialized blockstore adapter")
	return adapter, nil
}
