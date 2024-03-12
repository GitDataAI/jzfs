package integrationtest

import (
	"context"
	"net/http"
	"net/url"

	"github.com/smartystreets/goconvey/convey"
)

func StatusSpec(_ context.Context, urlStr string) func(c convey.C) {
	return func(_ convey.C) {
		url, err := url.Parse(urlStr)
		convey.ShouldBeNil(err)
		url.Path = "/status"
		resp, err := http.Get(url.String())
		convey.ShouldBeNil(err)
		convey.ShouldEqual(resp.StatusCode, http.StatusOK)
	}
}
