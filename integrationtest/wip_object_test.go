package integrationtest

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func WipObjectSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "jude"
		repoName := "hash"
		branchName := "feat/wip_obj_test"

		createUser(ctx, c, client, userName)
		loginAndSwitch(ctx, c, client, userName)
		createRepo(ctx, c, client, repoName)
		createBranch(ctx, c, client, userName, repoName, "main", branchName)
		createWip(ctx, c, client, "get wip obj test", userName, repoName, branchName)
		uploadObject(ctx, c, client, "update f1 to test branch", userName, repoName, branchName, "m.dat")
		uploadObject(ctx, c, client, "update f2 to test branch", userName, repoName, branchName, "g/m.dat")

		c.Convey("head object", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to head object in non exit user", func() {
				resp, err := client.HeadObject(ctx, "mock user", repoName, &api.HeadObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to head object in non exit repo", func() {
				resp, err := client.HeadObject(ctx, userName, "fakerepo", &api.HeadObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to head object in non exit branch", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					RefName: "mockref",
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden head object in others", func() {
				resp, err := client.HeadObject(ctx, "jimmy", "happygo", &api.HeadObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Path: "",
					Type: api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					RefName: branchName,
					Path:    "c/d.txt",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to head object", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})
		})

		c.Convey("get object", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to get object in non exit user", func() {
				resp, err := client.GetObject(ctx, "mock user", repoName, &api.GetObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit repo", func() {
				resp, err := client.GetObject(ctx, userName, "fakerepo", &api.GetObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit branch", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					RefName: "mockref",
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden get object in others", func() {
				resp, err := client.GetObject(ctx, "jimmy", "happygo", &api.GetObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					RefName: branchName,
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					RefName: branchName,
					Path:    "c/d.txt",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to get object", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					RefName: branchName,
					Path:    "m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})
		})

		c.Convey("delete object", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					RefName: branchName,
					Path:    "g/m.dat",
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to delete object in non exit user", func() {
				resp, err := client.DeleteObject(ctx, "mockUser", repoName, &api.DeleteObjectParams{
					RefName: branchName,
					Path:    "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to delete object in non exit repo", func() {
				resp, err := client.DeleteObject(ctx, userName, "fakerepo", &api.DeleteObjectParams{
					RefName: branchName,
					Path:    "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to delete object in non exit branch", func() {
				resp, err := client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					RefName: "mockref",
					Path:    "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden delete object in others", func() {
				resp, err := client.DeleteObject(ctx, "jimmy", "happygo", &api.DeleteObjectParams{
					RefName: "main",
					Path:    "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					RefName: branchName,
					Path:    "",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					RefName: branchName,
					Path:    "mm/t.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to delete object", func(c convey.C) {
				//ensure exit
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					RefName: branchName,
					Path:    "g/m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				resp, err = client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					RefName: branchName,
					Path:    "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				commitWip(ctx, c, client, "commit delete object", userName, repoName, branchName, "test")

				//ensure not exit
				resp, err = client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					RefName: branchName,
					Path:    "g/m.dat",
					Type:    api.RefTypeWip,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})
		})

		uploadObject(ctx, c, client, "update f3 to test branch", userName, repoName, branchName, "a/m.dat")
		uploadObject(ctx, c, client, "update f4 to test branch", userName, repoName, branchName, "a/b.dat")
		uploadObject(ctx, c, client, "update f5 to test branch", userName, repoName, branchName, "b.dat")
		uploadObject(ctx, c, client, "update f6 to test branch", userName, repoName, branchName, "c.dat")

		c.Convey("get wip changes", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetWipChanges(ctx, userName, repoName, &api.GetWipChangesParams{
					RefName: branchName,
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to get object in non exit user", func() {
				resp, err := client.GetWipChanges(ctx, "mock user", repoName, &api.GetWipChangesParams{
					RefName: branchName,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit repo", func() {
				resp, err := client.GetWipChanges(ctx, userName, "fakerepo", &api.GetWipChangesParams{
					RefName: branchName,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit branch", func() {
				resp, err := client.GetWipChanges(ctx, userName, repoName, &api.GetWipChangesParams{
					RefName: "mockref",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden get object in others", func() {
				resp, err := client.GetWipChanges(ctx, "jimmy", "happygo", &api.GetWipChangesParams{
					RefName: "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("not exit path", func() {
				resp, err := client.GetWipChanges(ctx, userName, repoName, &api.GetWipChangesParams{
					RefName: branchName,
					Path:    utils.String("a/b/c/d"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetWipChangesResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 0)
			})

			c.Convey("success to get object", func() {
				resp, err := client.GetWipChanges(ctx, userName, repoName, &api.GetWipChangesParams{
					RefName: branchName,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetWipChangesResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 4)
			})
		})
	}
}
