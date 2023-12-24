package integrationtest

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/smartystreets/goconvey/convey"
)

func GetEntriesInRefSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	baseHead := utils.String("")
	return func(c convey.C) {
		userName := "kitty"
		repoName := "black"
		branchName := "feat/get_entries_test"

		createUser(ctx, c, client, userName)
		loginAndSwitch(ctx, c, client, userName)
		createRepo(ctx, c, client, repoName)
		createBranch(ctx, c, client, userName, repoName, "main", branchName)
		createWip(ctx, c, client, "feat get entries test0", userName, repoName, branchName)
		uploadObject(ctx, c, client, "update f1 to test branch", userName, repoName, branchName, "m.dat")
		uploadObject(ctx, c, client, "update f2 to test branch", userName, repoName, branchName, "g/x.dat")
		uploadObject(ctx, c, client, "update f3 to test branch", userName, repoName, branchName, "g/m.dat")

		c.Convey("get wip entries", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeWip,
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to get entries in non exit user", func() {
				resp, err := client.GetEntriesInRef(ctx, "mock user", repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get entries in non exit repo", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, "fakerepo", &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get entries in non exit branch", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String("feat/fake_repo"),
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden get entries in others", func() {
				resp, err := client.GetEntriesInRef(ctx, "jimmy", "happygo", &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String("main"),
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("not exit path", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("a/b/c/d"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})
			c.Convey("success to get entries", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetEntriesInRefResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 2)
				convey.So(*(*result.JSON200)[0].Name, convey.ShouldEqual, "m.dat")
				convey.So(*(*result.JSON200)[1].Name, convey.ShouldEqual, "x.dat")
			})

			c.Convey("success to get entries in root", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("/"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetEntriesInRefResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 2)
				convey.So(*(*result.JSON200)[0].Name, convey.ShouldEqual, "g")
				convey.So(*(*result.JSON200)[1].Name, convey.ShouldEqual, "m.dat")
			})
		})

		commitWip(ctx, c, client, "commit kitty first changes", userName, repoName, branchName, "test")

		c.Convey("get branch entries", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeBranch,
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to get entries in non exit user", func() {
				resp, err := client.GetEntriesInRef(ctx, "mock user", repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get entries in non exit repo", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, "fakerepo", &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get entries in non exit branch", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String("feat/fake_repo"),
					Type: api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden get entries in others", func() {
				resp, err := client.GetEntriesInRef(ctx, "jimmy", "happygo", &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String("main"),
					Type: api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("not exit path", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("a/b/c/d"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("success to get entries", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("g"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetEntriesInRefResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 2)
				convey.So(*(*result.JSON200)[0].Name, convey.ShouldEqual, "m.dat")
				convey.So(*(*result.JSON200)[1].Name, convey.ShouldEqual, "x.dat")
			})

			c.Convey("success to get entries in root", func() {
				resp, err := client.GetEntriesInRef(ctx, userName, repoName, &api.GetEntriesInRefParams{
					Path: utils.String("/"),
					Ref:  utils.String(branchName),
					Type: api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetEntriesInRefResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 2)
				convey.So(*(*result.JSON200)[0].Name, convey.ShouldEqual, "g")
				convey.So(*(*result.JSON200)[1].Name, convey.ShouldEqual, "m.dat")
			})
		})

		createWip(ctx, c, client, "main wip", userName, repoName, "main")
		uploadObject(ctx, c, client, "update f1 to main branch", userName, repoName, "main", "a.dat")   //delete\
		uploadObject(ctx, c, client, "update f2 to main branch", userName, repoName, "main", "g/m.dat") //modify
		commitWip(ctx, c, client, "commit branch change", userName, repoName, "main", "test")

		c.Convey("difference", func(c convey.C) {
			c.Convey("get base and head", func() {
				resp, err := client.GetBranch(ctx, userName, repoName, &api.GetBranchParams{RefName: "main"})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
				baseBranch, err := api.ParseGetBranchResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				resp, err = client.GetBranch(ctx, userName, repoName, &api.GetBranchParams{RefName: branchName})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
				headBranch, err := api.ParseGetBranchResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				baseHead = utils.String(baseBranch.JSON200.CommitHash + "..." + headBranch.JSON200.CommitHash)
			})
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetCommitDiff(ctx, userName, repoName, utils.StringValue(baseHead), &api.GetCommitDiffParams{
					Path: utils.String("/"),
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to diff in non exit user", func() {
				resp, err := client.GetCommitDiff(ctx, "mockuser", repoName, utils.StringValue(baseHead), &api.GetCommitDiffParams{
					Path: utils.String("/"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to diff in non exit repo", func() {
				resp, err := client.GetCommitDiff(ctx, userName, "fakerepo", utils.StringValue(baseHead), &api.GetCommitDiffParams{
					Path: utils.String("/"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden diff in others", func() {
				resp, err := client.GetCommitDiff(ctx, "jimmy", "happygo", utils.StringValue(baseHead), &api.GetCommitDiffParams{
					Path: utils.String("/"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("not exit path", func() {
				resp, err := client.GetCommitDiff(ctx, userName, repoName, utils.StringValue(baseHead), &api.GetCommitDiffParams{
					Path: utils.String("/a/b/c/d"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})

			c.Convey("success to  diff", func() {
				resp, err := client.GetCommitDiff(ctx, userName, repoName, utils.StringValue(baseHead), &api.GetCommitDiffParams{
					Path: utils.String("/"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetCommitDiffResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 4)
				convey.So((*result.JSON200)[0].Path, convey.ShouldEqual, "a.dat")
				convey.So((*result.JSON200)[0].Action, convey.ShouldEqual, 2)
				convey.So((*result.JSON200)[1].Path, convey.ShouldEqual, "g/m.dat")
				convey.So((*result.JSON200)[1].Action, convey.ShouldEqual, 3)
				convey.So((*result.JSON200)[2].Path, convey.ShouldEqual, "g/x.dat")
				convey.So((*result.JSON200)[2].Action, convey.ShouldEqual, 1)
				convey.So((*result.JSON200)[3].Path, convey.ShouldEqual, "m.dat")
				convey.So((*result.JSON200)[3].Action, convey.ShouldEqual, 1)
			})
		})
	}
}
