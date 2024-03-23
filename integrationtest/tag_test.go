package integrationtest

import (
	"context"
	"net/http"
	"strconv"
	"time"

	"github.com/GitDataAI/jiaozifs/api"
	apiimpl "github.com/GitDataAI/jiaozifs/api/api_impl"
	"github.com/GitDataAI/jiaozifs/utils"
	"github.com/smartystreets/goconvey/convey"
)

func TagSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "tagUser"
		repoName := "tagTest"
		tagName := "v00.00.01"

		c.Convey("init", func(_ convey.C) {
			_ = createUser(ctx, client, userName)
			loginAndSwitch(ctx, client, userName, false)
			_ = createRepo(ctx, client, repoName, false)
			_ = createWip(ctx, client, userName, repoName, "main")
			_ = uploadObject(ctx, client, userName, repoName, "main", "a.txt", true)
			_ = uploadObject(ctx, client, userName, repoName, "main", "b.txt", true)
			_ = uploadObject(ctx, client, userName, repoName, "main", "c.txt", true)
			_ = commitWip(ctx, client, userName, repoName, "main", "commit wip")
		})

		c.Convey("create tag", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.CreateTag(ctx, userName, repoName, api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "main",
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to create tag in non exit user", func() {
				resp, err := client.CreateTag(ctx, "fakeUser", repoName, api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to create tag in non exit repo", func() {
				resp, err := client.CreateTag(ctx, userName, "fakerepo", api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to create tag in non exit branch or wrong commit hash", func() {
				resp, err := client.CreateTag(ctx, userName, repoName, api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "main_not_exist",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusInternalServerError)
			})

			c.Convey("fail to create tag with not exit commit", func() {
				resp, err := client.CreateTag(ctx, userName, repoName, api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "963362e15e39cbb92203641b8eeb5e7b",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("too long name", func() {
				resp, err := client.CreateTag(ctx, userName, repoName, api.CreateTagJSONRequestBody{
					Name:    "v00.00.01111111111111111111111111111111111111111111111111111111111111111111111111111",
					Message: utils.String("testv"),
					Target:  "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("target not found", func() {
				resp, err := client.CreateTag(ctx, userName, repoName, api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "1111",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden create tag in others", func() {
				resp, err := client.CreateTag(ctx, "jimmy", "happygo", api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success create tag from branch", func() {
				resp, err := client.CreateTag(ctx, userName, repoName, api.CreateTagJSONRequestBody{
					Name:    tagName,
					Message: utils.String("testv"),
					Target:  "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
			})

			c.Convey("success create tag from commit", func() {
				branch := getBranch(ctx, client, userName, repoName, "main")
				resp, err := client.CreateTag(ctx, userName, repoName, api.CreateTagJSONRequestBody{
					Name:    "v00.00.02",
					Message: utils.String("testv"),
					Target:  branch.CommitHash,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
			})
		})

		c.Convey("get tag", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetTag(ctx, userName, repoName, &api.GetTagParams{
					RefName: tagName,
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success get tag", func() {
				resp, err := client.GetTag(ctx, userName, repoName, &api.GetTagParams{
					RefName: tagName,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				respResult, err := api.ParseGetTagResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(respResult.JSON200.Name, convey.ShouldEqual, tagName)
			})

			c.Convey("fail to get non exit ref", func() {
				resp, err := client.GetTag(ctx, userName, repoName, &api.GetTagParams{
					RefName: "mock_ref",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get ref from non exit user", func() {
				resp, err := client.GetTag(ctx, "mock_owner", repoName, &api.GetTagParams{
					RefName: "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get non exit branch", func() {
				resp, err := client.GetTag(ctx, userName, "mock_repo", &api.GetTagParams{
					RefName: "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to others branch", func() {
				resp, err := client.GetTag(ctx, "jimmy", "happygo", &api.GetTagParams{
					RefName: "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
		})

		c.Convey("delete tag", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.DeleteTag(ctx, userName, repoName, &api.DeleteTagParams{RefName: tagName})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("delete tag in not exit repo", func() {
				resp, err := client.DeleteTag(ctx, userName, "falerepo", &api.DeleteTagParams{RefName: tagName})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("delete tag in non exit user", func() {
				resp, err := client.DeleteTag(ctx, "fakeuser", repoName, &api.DeleteTagParams{RefName: tagName})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("delete tag in other's repo", func() {
				resp, err := client.DeleteTag(ctx, "jimmy", "happygo", &api.DeleteTagParams{RefName: tagName})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("delete tag successful", func() {
				resp, err := client.DeleteTag(ctx, userName, repoName, &api.DeleteTagParams{RefName: tagName})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResp, err := client.GetTag(ctx, userName, repoName, &api.GetTagParams{RefName: tagName})
				convey.So(err, convey.ShouldBeNil)
				convey.So(getResp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})
		})

		c.Convey("list tag", func(c convey.C) {
			repo2Name := "tagTest2"
			prefix := "vt"
			c.Convey("init", func() {
				_ = createRepo(ctx, client, repo2Name, false)
				_ = createWip(ctx, client, userName, repo2Name, "main")
				_ = uploadObject(ctx, client, userName, repo2Name, "main", "b.txt", true)
				_ = commitWip(ctx, client, userName, repo2Name, "main", "commit wip")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.01", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.02", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.03", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.04", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.05", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.06", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.07", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.08", "main")
				createTag(ctx, client, userName, repo2Name, prefix+"00.00.09", "main")
			})

			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.ListTags(ctx, userName, repo2Name, &api.ListTagsParams{})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to list branchs from non exit user", func() {
				resp, err := client.ListTags(ctx, "mock_owner", repo2Name, &api.ListTagsParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to list branchs in non exit repo", func() {
				resp, err := client.ListTags(ctx, userName, "mockrepo", &api.ListTagsParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to list branches in others repo", func() {
				resp, err := client.ListTags(ctx, "jimmy", "happygo", &api.ListTagsParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success list branch", func() {
				resp, err := client.ListTags(ctx, userName, repo2Name, &api.ListTagsParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				respResult, err := api.ParseListBranchesResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(respResult.JSON200.Results, convey.ShouldHaveLength, 9)
			})

			c.Convey("success list branch by prefix", func() {
				resp, err := client.ListTags(ctx, userName, repo2Name, &api.ListTagsParams{
					Prefix: &prefix,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				respResult, err := api.ParseListBranchesResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(respResult.JSON200.Results, convey.ShouldHaveLength, 9)
			})

			c.Convey("success list branch paganation", func() {
				var after *int64
				for i := 0; i < 5; i++ {
					resp, err := client.ListTags(ctx, userName, repo2Name, &api.ListTagsParams{
						After:  after,
						Amount: utils.Int(2),
					})
					convey.So(err, convey.ShouldBeNil)
					convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

					result, err := api.ParseListTagsResponse(resp)
					convey.So(err, convey.ShouldBeNil)
					convey.ShouldHaveLength(*result.JSON200, 2)
					if i >= 5 {
						convey.ShouldBeFalse((*result.JSON200).Pagination.HasMore)
					} else {
						convey.ShouldBeTrue((*result.JSON200).Pagination.HasMore)
						val, err := strconv.ParseInt((*result.JSON200).Pagination.NextOffset, 10, 64)
						convey.So(err, convey.ShouldBeNil)
						next := time.UnixMilli(val)
						after = utils.Int64(next.UnixMilli())
					}
				}

			})
		})

	}
}
