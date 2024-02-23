package aksk

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"fmt"
	"net/http"
	"net/url"
	"sort"
	"strings"
	"time"
)

type Verifier interface {
	// IsAkskCredential check the requess is aksk credential, just check request have AccessKeykey
	IsAkskCredential(req *http.Request) bool
	// Verify verify the request and return access key
	Verify(req *http.Request) (string, error)
}

type SkGetter interface {
	Get(ak string) (string, error)
}

var _ Verifier = (*V0Verier)(nil)

type V0Verier struct {
	skGetter SkGetter
}

func NewV0Verier(skGetter SkGetter) *V0Verier {
	return &V0Verier{skGetter: skGetter}
}

func (v *V0Verier) IsAkskCredential(req *http.Request) bool {
	accessKey := req.URL.Query().Get(AccessKeykey)
	return len(accessKey) > 0
}

func (v *V0Verier) Verify(req *http.Request) (string, error) {
	query := req.URL.Query()
	accessKey := query.Get(AccessKeykey)
	if len(accessKey) == 0 {
		return "", fmt.Errorf("ak not found")
	}

	secretKey, err := v.skGetter.Get(accessKey)
	if err != nil {
		return "", fmt.Errorf("access key not correct")
	}

	sigMethod := query.Get(SignatureMethodKey)
	if sigMethod != signatureMethod {
		return "", fmt.Errorf("invalid signature method %s", sigMethod)
	}

	sigVersion := query.Get(SignatureVersionKey)
	if sigVersion != signatureVersion {
		return "", fmt.Errorf("invalid signature method %s", sigMethod)
	}

	reqTime := query.Get(TimestampKey)
	t, err := time.Parse(timeFormat, reqTime)
	if err != nil {
		return "", fmt.Errorf("invalid timestamp %s", reqTime)
	}
	if t.Before(time.Now().Add(-5 * time.Minute)) {
		return "", fmt.Errorf("request is out of data")
	}
	expectSignature := query.Get(SignatureKey)
	query.Del(SignatureKey)

	method := req.Method
	host := req.Host
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

	hash := hmac.New(sha256.New, []byte(secretKey))
	hash.Write([]byte(stringToSign))
	actualSig := base64.StdEncoding.EncodeToString(hash.Sum(nil))
	if actualSig != expectSignature {
		return "", fmt.Errorf("signature not correct")
	}
	return accessKey, nil
}
