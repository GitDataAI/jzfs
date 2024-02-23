package api

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/auth/aksk"
)

func AkSkOption(ak, sk string) ClientOption {
	return func(client *Client) error {
		client.RequestEditors = append(client.RequestEditors, func(_ context.Context, req *http.Request) error {
			signer := aksk.NewV0Signer(ak, sk)
			return signer.Sign(req)
		})
		return nil
	}
}
