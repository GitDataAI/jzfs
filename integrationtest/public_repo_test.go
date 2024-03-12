package integrationtest

import (
	"context"
	"fmt"
	"net/http"
	"strconv"

	"github.com/GitDataAI/jiaozifs/utils"

	"github.com/GitDataAI/jiaozifs/api"
	apiimpl "github.com/GitDataAI/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func PublicRepoSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	user1Name := "pubuser"
	testRepoName := "test_repo"

	user2Name := "otheruser"
	testRepo2Name := "test_repo2"

	var user1Token, user2Token []api.RequestEditorFn
	return func(c convey.C) {

		c.Convey("init", func(_ convey.C) {
			_ = createUser(ctx, client, user1Name)
			_ = createUser(ctx, client, user2Name)
			user1Token = getToken(ctx, client, user1Name)
			user2Token = getToken(ctx, client, user2Name)

			client.RequestEditors = user1Token
			_ = createRepo(ctx, client, testRepoName, false)

			client.RequestEditors = user2Token
			_ = createRepo(ctx, client, testRepo2Name, false)

			client.RequestEditors = user1Token
		})

		c.Convey("change visiable", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.ChangeVisible(ctx, user1Name, testRepoName, &api.ChangeVisibleParams{Visible: true})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("non exit user", func() {
				resp, err := client.ChangeVisible(ctx, "mockUser", testRepoName, &api.ChangeVisibleParams{Visible: true})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("non exit repo", func() {
				resp, err := client.ChangeVisible(ctx, user1Name, "mockRepo", &api.ChangeVisibleParams{Visible: true})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("change repo visible in  others repo", func() {
				resp, err := client.ChangeVisible(ctx, user2Name, testRepo2Name, &api.ChangeVisibleParams{Visible: true})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success change visible", func() {
				repoBeforeUpdated := getRepo(ctx, client, user1Name, testRepoName)
				convey.ShouldBeFalse(repoBeforeUpdated.Head)

				resp, err := client.ChangeVisible(ctx, user1Name, testRepoName, &api.ChangeVisibleParams{Visible: true})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				repoAfterUpdated := getRepo(ctx, client, user1Name, testRepoName)
				convey.ShouldBeTrue(repoAfterUpdated.Head)
			})
		})

		c.Convey("check permission", func(c convey.C) {
			c.Convey("init", func() {
				resp, err := client.ChangeVisible(ctx, user1Name, testRepoName, &api.ChangeVisibleParams{Visible: false})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				client.RequestEditors = user2Token
			})

			c.Convey("cannot read branch", func() {
				resp, err := client.GetBranch(ctx, user1Name, testRepoName, &api.GetBranchParams{RefName: "main"})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("cannot create branch", func() {
				client.RequestEditors = user1Token
				resp, err := client.ChangeVisible(ctx, user1Name, testRepoName, &api.ChangeVisibleParams{Visible: true})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
				client.RequestEditors = user2Token
			})

			c.Convey("can read branch", func() {
				resp, err := client.GetBranch(ctx, user1Name, testRepoName, &api.GetBranchParams{RefName: "main"})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})
		})

		c.Convey("list public repo", func() {
			resp, err := client.ListPublicRepository(ctx, &api.ListPublicRepositoryParams{})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

			result, err := api.ParseListPublicRepositoryResponse(resp)
			convey.So(err, convey.ShouldBeNil)
			convey.ShouldHaveLength(1, result.JSON200.Results)
		})

		c.Convey("list many repo", func() {
			for i := 0; i < 20; i++ {
				_ = createRepo(ctx, client, fmt.Sprintf("aa%d", i), true)
			}
			repo := createRepo(ctx, client, "aa21", true)
			_ = createRepo(ctx, client, "aa22", true)

			after := repo.UpdatedAt
			count := 0
			for {
				resp, err := client.ListPublicRepository(ctx, &api.ListPublicRepositoryParams{
					Prefix: utils.String("aa"),
					After:  utils.Int64(after),
					Amount: utils.Int(2),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseListPublicRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldHaveLength(1, result.JSON200.Results)

				count = count + len(result.JSON200.Results)
				if !result.JSON200.Pagination.HasMore {
					break
				}
				convey.ShouldHaveLength(2, result.JSON200.Results)
				after, err = strconv.ParseInt(result.JSON200.Pagination.NextOffset, 10, 64)
				convey.So(err, convey.ShouldBeNil)
			}
			convey.ShouldEqual(21, count)
		})
	}
}
