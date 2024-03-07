package config

import (
	"fmt"
	"time"

	"github.com/jiaozifs/jiaozifs/block/params"
	"github.com/mitchellh/go-homedir"
)

type BlockStoreConfig struct {
	Type                   string  `mapstructure:"type" validate:"required" json:"type"`
	DefaultNamespacePrefix *string `mapstructure:"default_namespace_prefix" json:"default_namespace_prefix"`

	Local *struct {
		Path                    string   `mapstructure:"path" json:"path"`
		ImportEnabled           bool     `mapstructure:"import_enabled" json:"import_enabled"`
		ImportHidden            bool     `mapstructure:"import_hidden" json:"import_hidden"`
		AllowedExternalPrefixes []string `mapstructure:"allowed_external_prefixes" json:"allowed_external_prefixes"`
	} `mapstructure:"local" json:"local"`
	Ipfs *struct {
		URL string `mapstructure:"url" json:"url"`
	} `mapstructure:"ipfs" json:"ipfs"`
	S3 *struct {
		S3AuthInfo                    `mapstructure:",squash"`
		Region                        string        `mapstructure:"region" json:"region"`
		Endpoint                      string        `mapstructure:"endpoint" json:"endpoint"`
		MaxRetries                    int           `mapstructure:"max_retries" json:"max_retries"`
		ForcePathStyle                bool          `mapstructure:"force_path_style" json:"force_path_style"`
		DiscoverBucketRegion          bool          `mapstructure:"discover_bucket_region" json:"discover_bucket_region"`
		SkipVerifyCertificateTestOnly bool          `mapstructure:"skip_verify_certificate_test_only" json:"skip_verify_certificate_test_only"`
		ServerSideEncryption          string        `mapstructure:"server_side_encryption" json:"server_side_encryption"`
		ServerSideEncryptionKmsKeyID  string        `mapstructure:"server_side_encryption_kms_key_id" json:"server_side_encryption_kms_key_id"`
		PreSignedExpiry               time.Duration `mapstructure:"pre_signed_expiry" json:"pre_signed_expiry"`
		DisablePreSigned              bool          `mapstructure:"disable_pre_signed" json:"disable_pre_signed"`
		DisablePreSignedUI            bool          `mapstructure:"disable_pre_signed_ui" json:"disable_pre_signed_ui"`
		ClientLogRetries              bool          `mapstructure:"client_log_retries" json:"client_log_retries"`
		ClientLogRequest              bool          `mapstructure:"client_log_request" json:"client_log_request"`
		WebIdentity                   *struct {
			SessionDuration     time.Duration `mapstructure:"session_duration" json:"session_duration"`
			SessionExpiryWindow time.Duration `mapstructure:"session_expiry_window" json:"session_expiry_window"`
		} `mapstructure:"web_identity"`
	} `mapstructure:"s3" json:"s3"`
	Azure *struct {
		TryTimeout         time.Duration `mapstructure:"try_timeout" json:"try_timeout"`
		StorageAccount     string        `mapstructure:"storage_account" json:"storage_account"`
		StorageAccessKey   string        `mapstructure:"storage_access_key" json:"storage_access_key"`
		PreSignedExpiry    time.Duration `mapstructure:"pre_signed_expiry" json:"pre_signed_expiry"`
		DisablePreSigned   bool          `mapstructure:"disable_pre_signed" json:"disable_pre_signed"`
		DisablePreSignedUI bool          `mapstructure:"disable_pre_signed_ui" json:"disable_pre_signed_ui"`
		// TestEndpointURL for testing purposes
		TestEndpointURL string `mapstructure:"test_endpoint_url" json:"test_endpoint_url"`
	} `mapstructure:"azure" json:"azure"`
	GS *struct {
		S3Endpoint         string        `mapstructure:"s3_endpoint" json:"s3_endpoint"`
		CredentialsFile    string        `mapstructure:"credentials_file" json:"credentials_file"`
		CredentialsJSON    string        `mapstructure:"credentials_json" json:"credentials_json"`
		PreSignedExpiry    time.Duration `mapstructure:"pre_signed_expiry" json:"pre_signed_expiry"`
		DisablePreSigned   bool          `mapstructure:"disable_pre_signed" json:"disable_pre_signed"`
		DisablePreSignedUI bool          `mapstructure:"disable_pre_signed_ui" json:"disable_pre_signed_ui"`
	} `mapstructure:"gs" json:"gs"`
}

func (c *BlockStoreConfig) BlockstoreType() string {
	return c.Type
}

func (c *BlockStoreConfig) BlockstoreIpfsParams() (params.Ipfs, error) {
	return params.Ipfs{
		URL: c.Ipfs.URL,
	}, nil
}

func (c *BlockStoreConfig) BlockstoreS3Params() (params.S3, error) {
	var webIdentity *params.S3WebIdentity
	if c.S3.WebIdentity != nil {
		webIdentity = &params.S3WebIdentity{
			SessionDuration:     c.S3.WebIdentity.SessionDuration,
			SessionExpiryWindow: c.S3.WebIdentity.SessionExpiryWindow,
		}
	}

	var creds params.S3Credentials
	if c.S3.Credentials != nil {
		creds.AccessKeyID = c.S3.Credentials.AccessKeyID.SecureValue()
		creds.SecretAccessKey = c.S3.Credentials.SecretAccessKey.SecureValue()
		creds.SessionToken = c.S3.Credentials.SessionToken.SecureValue()
	}

	return params.S3{
		Region:                        c.S3.Region,
		Profile:                       c.S3.Profile,
		CredentialsFile:               c.S3.CredentialsFile,
		Credentials:                   creds,
		MaxRetries:                    c.S3.MaxRetries,
		Endpoint:                      c.S3.Endpoint,
		ForcePathStyle:                c.S3.ForcePathStyle,
		DiscoverBucketRegion:          c.S3.DiscoverBucketRegion,
		SkipVerifyCertificateTestOnly: c.S3.SkipVerifyCertificateTestOnly,
		ServerSideEncryption:          c.S3.ServerSideEncryption,
		ServerSideEncryptionKmsKeyID:  c.S3.ServerSideEncryptionKmsKeyID,
		PreSignedExpiry:               c.S3.PreSignedExpiry,
		DisablePreSigned:              c.S3.DisablePreSigned,
		DisablePreSignedUI:            c.S3.DisablePreSignedUI,
		ClientLogRetries:              c.S3.ClientLogRetries,
		ClientLogRequest:              c.S3.ClientLogRequest,
		WebIdentity:                   webIdentity,
	}, nil
}

func (c *BlockStoreConfig) BlockstoreLocalParams() (params.Local, error) {
	localPath := c.Local.Path
	path, err := homedir.Expand(localPath)
	if err != nil {
		return params.Local{}, fmt.Errorf("parse blockstore location URI %s: %w", localPath, err)
	}

	params := params.Local(*c.Local)
	params.Path = path
	return params, nil
}

func (c *BlockStoreConfig) BlockstoreGSParams() (params.GS, error) {
	credPath, err := homedir.Expand(c.GS.CredentialsFile)
	if err != nil {
		return params.GS{}, fmt.Errorf("parse GS credentials path '%s': %w", c.GS.CredentialsFile, err)
	}
	return params.GS{
		CredentialsFile:    credPath,
		CredentialsJSON:    c.GS.CredentialsJSON,
		PreSignedExpiry:    c.GS.PreSignedExpiry,
		DisablePreSigned:   c.GS.DisablePreSigned,
		DisablePreSignedUI: c.GS.DisablePreSignedUI,
	}, nil
}

func (c *BlockStoreConfig) BlockstoreAzureParams() (params.Azure, error) {
	return params.Azure{
		StorageAccount:     c.Azure.StorageAccount,
		StorageAccessKey:   c.Azure.StorageAccessKey,
		TryTimeout:         c.Azure.TryTimeout,
		PreSignedExpiry:    c.Azure.PreSignedExpiry,
		TestEndpointURL:    c.Azure.TestEndpointURL,
		DisablePreSigned:   c.Azure.DisablePreSigned,
		DisablePreSignedUI: c.Azure.DisablePreSignedUI,
	}, nil
}

type SecureString string

// String returns an elided version.  It is safe to call for logging.
func (SecureString) String() string {
	return "[SECRET]"
}

// SecureValue returns the actual value of s as a string.
func (s SecureString) SecureValue() string {
	return string(s)
}

// S3AuthInfo holds S3-style authentication.
type S3AuthInfo struct {
	CredentialsFile string `mapstructure:"credentials_file" json:"credentials_file"`
	Profile         string
	Credentials     *struct {
		AccessKeyID     SecureString `mapstructure:"access_key_id" json:"access_key_id"`
		SecretAccessKey SecureString `mapstructure:"secret_access_key" json:"secret_access_key"`
		SessionToken    SecureString `mapstructure:"session_token" json:"session_token"`
	}
}
