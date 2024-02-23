package aksk

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"net/http"
	"net/url"
	"sort"
	"strings"
	"time"
)

const (
	AccessKeykey        = "JiaozifsAccessKeyId"
	SignatureVersionKey = "SignatureVersion"
	SignatureMethodKey  = "SignatureMethod"
	TimestampKey        = "Timestamp"
	SignatureKey        = "Signature"

	signatureVersion = "0"
	signatureMethod  = "HmacSHA256"
	timeFormat       = "2006-01-02T15:04:05Z"
)

type Signer interface {
	Sign(req *http.Request) error
}

var _ Signer = (*V0Signer)(nil)

type V0Signer struct {
	accessKey, secretKey string
}

func NewV0Signer(accessKey string, secretKey string) *V0Signer {
	return &V0Signer{accessKey: accessKey, secretKey: secretKey}
}

func (voSigner V0Signer) Sign(req *http.Request) error {
	// http verb

	curTime := time.Now()
	// set query parameter
	query := req.URL.Query()
	query.Set(AccessKeykey, voSigner.accessKey)
	query.Set(SignatureVersionKey, signatureVersion)
	query.Set(SignatureVersionKey, signatureMethod)
	query.Set(TimestampKey, curTime.UTC().Format(timeFormat))

	req.Header.Del("Signature")

	method := req.Method
	host := req.URL.Host
	path := req.URL.Path
	if path == "" {
		path = "/"
	}

	// obtain all of the query keys and sort them
	queryKeys := make([]string, 0, len(query))
	for key := range query {
		queryKeys = append(queryKeys, key)
	}
	sort.Strings(queryKeys)

	// build URL-encoded query keys and values
	queryKeysAndValues := make([]string, len(queryKeys))
	for i, key := range queryKeys {
		k := strings.Replace(url.QueryEscape(key), "+", "%20", -1)
		v := strings.Replace(url.QueryEscape(query.Get(key)), "+", "%20", -1)
		queryKeysAndValues[i] = k + "=" + v
	}

	// join into one query string
	queryString := strings.Join(queryKeysAndValues, "&")

	// build the canonical string for the V2 signature
	stringToSign := strings.Join([]string{
		method,
		host,
		path,
		queryString,
	}, "\n")

	hash := hmac.New(sha256.New, []byte(voSigner.secretKey))
	hash.Write([]byte(stringToSign))
	signature := base64.StdEncoding.EncodeToString(hash.Sum(nil))
	query.Set(SignatureKey, signature)

	req.URL.RawQuery = query.Encode()
	return nil
}
