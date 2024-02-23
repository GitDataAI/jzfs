package aksk

import (
	crand "crypto/rand"
	"encoding/hex"
	"io"
	"math/rand"
	"net/http"
	"testing"
	"time"

	"github.com/stretchr/testify/require"
)

func TestFull(t *testing.T) {
	ak, sk, err := GenerateAksk()
	require.NoError(t, err)

	t.Run("correct", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		actualAk, err := verifer.Verify(req)
		require.NoError(t, err)
		require.Equal(t, ak, actualAk)
	})

	t.Run("fail verify", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Add("a", "b")
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})
	t.Run("no access id", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Del("AWSAccessKeyId")
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})
	t.Run("sig method fail", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Set("SignatureMethod", "2")
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})
	t.Run("sig method fail", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Set("SignatureMethod", "md5")
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})
	t.Run("sig version fail", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Set("SignatureVersion", "2")
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})
	t.Run("no timestamp", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Del("Timestamp")
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})

	t.Run("invalid timestamp format", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Set("Timestamp", time.Now().String())
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})
	t.Run("request out of date", func(t *testing.T) {
		req := mockHttpRequest()
		signer := NewV0Signer(ak, sk)
		verifer := NewV0Verier(skGetter{sk})

		err = signer.Sign(req)
		require.NoError(t, err)

		query := req.URL.Query()
		query.Set("Timestamp", time.Now().Add(-time.Minute*10).UTC().Format(timeFormat))
		req.URL.RawQuery = query.Encode()
		_, err = verifer.Verify(req)
		require.Error(t, err)
	})
}

type skGetter struct {
	sk string
}

func (getter skGetter) Get(_ string) (string, error) {
	return getter.sk, nil
}

func mockHttpRequest() *http.Request { //nolint
	verbs := []string{"GET", "POST", "PUT", "Delete"}
	req, _ := http.NewRequest(verbs[rand.Intn(3)], "http://www.xx.com/index.html", closerWraper{io.LimitReader(crand.Reader, 100)})

	query := req.URL.Query()
	for i := 0; i < 3; i++ {
		query.Set(randString(), randString())
	}
	req.URL.RawQuery = query.Encode()
	return req
}

var _ io.ReadCloser = (*closerWraper)(nil)

type closerWraper struct {
	reader io.Reader
}

func (c closerWraper) Read(p []byte) (int, error) {
	return c.reader.Read(p)
}

func (c closerWraper) Close() error {
	return nil
}

func randString() string {
	akBytes, _ := io.ReadAll(io.LimitReader(crand.Reader, 16))
	return hex.EncodeToString(akBytes)
}
