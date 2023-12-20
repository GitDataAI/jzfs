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
		refName := "feat/wip_obj_test"

		createUser(ctx, c, client, userName)
		loginAndSwitch(ctx, c, client, userName)
		createRepo(ctx, c, client, repoName)
		createBranch(ctx, c, client, userName, repoName, "main", refName)
		createWip(ctx, c, client, userName, repoName, refName)
		uploadRandomObject(ctx, c, client, userName, repoName, refName, "m.dat")
		uploadRandomObject(ctx, c, client, userName, repoName, refName, "g/m.dat")

		c.Convey("head object", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to head object in non exit user", func() {
				resp, err := client.HeadObject(ctx, "mock user", repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to head object in non exit repo", func() {
				resp, err := client.HeadObject(ctx, userName, "fakerepo", &api.HeadObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to head object in non exit branch", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: "mockref",
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden head object in others", func() {
				resp, err := client.HeadObject(ctx, "jimmy", "happygo", &api.HeadObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Path:  "",
					IsWip: utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "c/d.txt",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to head object", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
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
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to get object in non exit user", func() {
				resp, err := client.GetObject(ctx, "mock user", repoName, &api.GetObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit repo", func() {
				resp, err := client.GetObject(ctx, userName, "fakerepo", &api.GetObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit branch", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: "mockref",
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden get object in others", func() {
				resp, err := client.GetObject(ctx, "jimmy", "happygo", &api.GetObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: refName,
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: refName,
					Path:   "c/d.txt",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to get object", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: refName,
					Path:   "m.dat",
					IsWip:  utils.Bool(true),
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
					Branch: refName,
					Path:   "g/m.dat",
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to delete object in non exit user", func() {
				resp, err := client.DeleteObject(ctx, "mockUser", repoName, &api.DeleteObjectParams{
					Branch: refName,
					Path:   "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to delete object in non exit repo", func() {
				resp, err := client.DeleteObject(ctx, userName, "fakerepo", &api.DeleteObjectParams{
					Branch: refName,
					Path:   "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to delete object in non exit branch", func() {
				resp, err := client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					Branch: "mockref",
					Path:   "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden delete object in others", func() {
				resp, err := client.DeleteObject(ctx, "jimmy", "happygo", &api.DeleteObjectParams{
					Branch: "main",
					Path:   "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					Branch: refName,
					Path:   "",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					Branch: refName,
					Path:   "mm/t.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to delete object", func(c convey.C) {
				//ensure exit
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "g/m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				resp, err = client.DeleteObject(ctx, userName, repoName, &api.DeleteObjectParams{
					Branch: refName,
					Path:   "g/m.dat",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				commitWip(ctx, c, client, userName, repoName, refName)

				//ensure not exit
				resp, err = client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "g/m.dat",
					IsWip:  utils.Bool(true),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})
		})
	}
}