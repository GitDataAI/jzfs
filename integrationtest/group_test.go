package integrationtest

import (
	"context"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func GroupSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "grouptest"
		createUser(ctx, client, userName)
		loginAndSwitch(ctx, client, userName, false)

		resp, err := client.ListRepoGroup(ctx)
		convey.ShouldBeNil(c, err)

		result, err := api.ParseListRepoGroupResponse(resp)
		convey.ShouldBeNil(c, err)
		convey.ShouldHaveLength(result, 3)
	}
}
