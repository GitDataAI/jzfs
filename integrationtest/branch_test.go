package integrationtest

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func BranchSpec(ctx context.Context, urlStr string) func(c convey.C) {
	return func(c convey.C) {
		client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
		userName := "mike"
		repoName := "mlops"
		createUser(ctx, c, client, userName)
		loginAndSwitch(ctx, c, client, userName)
		createRepo(ctx, c, client, repoName)
		c.Convey("create branch", func() {
			c.Convey("success create branch", func() {
				resp, err := client.CreateBranch(ctx, userName, repoName, api.CreateBranchJSONRequestBody{
					Name: "feat/test",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})
		})
	}
}
