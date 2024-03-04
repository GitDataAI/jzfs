package integrationtest

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
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

		c.Convey("init", func(c convey.C) {
			_ = createUser(ctx, client, user1Name)
			_ = createUser(ctx, client, user2Name)
			user1Token = getToken(ctx, client, user1Name)
			user2Token = getToken(ctx, client, user2Name)

			client.RequestEditors = user1Token
			_ = createRepo(ctx, client, testRepoName)

			client.RequestEditors = user2Token
			_ = createRepo(ctx, client, testRepo2Name)

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

		c.Convey("", func(c convey.C) {
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
	}
}
