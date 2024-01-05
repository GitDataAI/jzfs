package integrationtest

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func MergeRequestSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "navi"
		repoName := "mr_test"
		branchName := "feat/obj_test"

		createUser(ctx, c, client, userName)
		loginAndSwitch(ctx, c, client, "molly login", userName, false)
		createRepo(ctx, c, client, repoName)
		createBranch(ctx, c, client, userName, repoName, "main", branchName)
		createWip(ctx, c, client, "feat get obj test", userName, repoName, branchName)
		uploadObject(ctx, c, client, "update f1 to test branch", userName, repoName, branchName, "a.bin")
		commitWip(ctx, c, client, "commit delete object", userName, repoName, branchName, "test")

		c.Convey("create merge request", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.CreateMergeRequest(ctx, userName, repoName, api.CreateMergeRequestJSONRequestBody{
					Description:      utils.String("create merge request test"),
					SourceBranchName: branchName,
					TargetBranchName: branchName,
					Title:            "Merge: test",
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
		})
	}
}
