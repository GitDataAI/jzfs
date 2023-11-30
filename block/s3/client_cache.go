package s3

import (
	"context"
	"sync"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/feature/s3/manager"
	"github.com/aws/aws-sdk-go-v2/service/s3"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/block/params"
)

var log = logging.Logger("s3")

type (
	clientFactory  func(region string) *s3.Client
	s3RegionGetter func(ctx context.Context, bucket string) (string, error)
)

type ClientCache struct {
	mu             sync.Mutex
	regionClient   map[string]*s3.Client
	bucketRegion   map[string]string
	awsConfig      aws.Config
	defaultClient  *s3.Client
	clientFactory  clientFactory
	s3RegionGetter s3RegionGetter
}

func NewClientCache(awsConfig aws.Config, params params.S3) *ClientCache {
	clientFactory := newClientFactory(awsConfig, WithClientParams(params))
	defaultClient := clientFactory(awsConfig.Region)
	clientCache := &ClientCache{
		regionClient:  make(map[string]*s3.Client),
		bucketRegion:  make(map[string]string),
		awsConfig:     awsConfig,
		defaultClient: defaultClient,
		clientFactory: clientFactory,
	}
	clientCache.DiscoverBucketRegion(true)
	return clientCache
}

// newClientFactory returns a function that creates a new S3 client with the given region.
// accepts aws configuration and list of s3 options functions to apply with the s3 client.
// the factory function is used to create a new client for a region when it is not cached.
func newClientFactory(awsConfig aws.Config, s3OptFns ...func(options *s3.Options)) clientFactory {
	return func(region string) *s3.Client {
		return s3.NewFromConfig(awsConfig, func(options *s3.Options) {
			for _, opts := range s3OptFns {
				opts(options)
			}
			options.Region = region
		})
	}
}

func (c *ClientCache) SetClientFactory(clientFactory clientFactory) {
	c.clientFactory = clientFactory
}

func (c *ClientCache) SetS3RegionGetter(s3RegionGetter s3RegionGetter) {
	c.s3RegionGetter = s3RegionGetter
}

func (c *ClientCache) DiscoverBucketRegion(b bool) {
	if b {
		c.s3RegionGetter = c.getBucketRegionFromAWS
	} else {
		c.s3RegionGetter = c.getBucketRegionDefault
	}
}

func (c *ClientCache) getBucketRegionFromAWS(ctx context.Context, bucket string) (string, error) {
	return manager.GetBucketRegion(ctx, c.defaultClient, bucket)
}

func (c *ClientCache) getBucketRegionDefault(_ context.Context, _ string) (string, error) {
	return c.awsConfig.Region, nil
}

func (c *ClientCache) Get(ctx context.Context, bucket string) *s3.Client {
	client, region := c.cachedClientByBucket(bucket)
	if client != nil {
		return client
	}

	// lookup region if needed
	if region == "" {
		region = c.refreshBucketRegion(ctx, bucket)
		if client, ok := c.cachedClientByRegion(region); ok {
			return client
		}
	}

	// create client and update cache
	log.With("region", region).Debug("creating client for region")
	client = c.clientFactory(region)

	// re-check if a client was created by another goroutine
	// keep using the existing client and discard the new one
	c.mu.Lock()
	existingClient, existingFound := c.regionClient[region]
	if existingFound {
		client = existingClient
	} else {
		c.regionClient[region] = client
	}
	c.mu.Unlock()
	return client
}

func (c *ClientCache) cachedClientByBucket(bucket string) (*s3.Client, string) {
	c.mu.Lock()
	defer c.mu.Unlock()
	if region, ok := c.bucketRegion[bucket]; ok {
		return c.regionClient[region], region
	}
	return nil, ""
}

func (c *ClientCache) cachedClientByRegion(region string) (*s3.Client, bool) {
	c.mu.Lock()
	defer c.mu.Unlock()
	client, ok := c.regionClient[region]
	return client, ok
}

func (c *ClientCache) refreshBucketRegion(ctx context.Context, bucket string) string {
	region, err := c.s3RegionGetter(ctx, bucket)
	if err != nil {
		// fallback to default region
		region = c.awsConfig.Region
		log.With("default_region", region).
			Error("Failed to get region for bucket, falling back to default region")
	}
	// update bucket to region cache
	c.mu.Lock()
	c.bucketRegion[bucket] = region
	c.mu.Unlock()
	return region
}

func (c *ClientCache) GetDefault() *s3.Client {
	return c.defaultClient
}
