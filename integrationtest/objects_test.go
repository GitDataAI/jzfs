package integrationtest

import (
	"bytes"
	"context"
	"encoding/hex"
	"fmt"
	"io"
	"net/http"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func ObjectSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "molly"
		repoName := "dataspace"
		refName := "feat/obj_test"

		createUser(ctx, c, client, userName)
		loginAndSwitch(ctx, c, client, userName)
		createRepo(ctx, c, client, repoName)
		createBranch(ctx, c, client, userName, repoName, "main", refName)
		createWip(ctx, c, client, "feat get obj test", userName, repoName, refName)

		c.Convey("upload object", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.UploadObjectWithBody(ctx, userName, repoName, &api.UploadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to create branch in non exit user", func() {
				resp, err := client.UploadObjectWithBody(ctx, "mockuser", "main", &api.UploadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to upload in non exit repo", func() {
				resp, err := client.UploadObjectWithBody(ctx, userName, "fakerepo", &api.UploadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to upload object in non exit branch", func() {
				resp, err := client.UploadObjectWithBody(ctx, userName, repoName, &api.UploadObjectParams{
					Branch: "mockref",
					Path:   "a.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to upload object in no wip ", func() {
				resp, err := client.UploadObjectWithBody(ctx, userName, repoName, &api.UploadObjectParams{
					Branch: "main",
					Path:   "a.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden upload object in others", func() {
				resp, err := client.UploadObjectWithBody(ctx, "jimmy", "happygo", &api.UploadObjectParams{
					Branch: "main",
					Path:   "a.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.UploadObjectWithBody(ctx, userName, repoName, &api.UploadObjectParams{
					Branch: refName,
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success upload object", func() {
				resp, err := client.UploadObjectWithBody(ctx, userName, repoName, &api.UploadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
			})

			c.Convey("success upload object on subpath", func() {
				resp, err := client.UploadObjectWithBody(ctx, userName, repoName, &api.UploadObjectParams{
					Branch: refName,
					Path:   "a/b.bin",
				}, "application/octet-stream", bytes.NewReader([]byte{1, 2, 3, 4, 5, 6, 7, 8, 1, 1, 1, 1}))
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
			})
		})

		//commit object to branch
		c.Convey("commit wip", func() {
			resp, err := client.CommitWip(ctx, userName, repoName, &api.CommitWipParams{
				RefName: refName,
				Msg:     "test commit msg",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
		})

		c.Convey("head object", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to head object in non exit user", func() {
				resp, err := client.HeadObject(ctx, "mock user", repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to head object in non exit repo", func() {
				resp, err := client.HeadObject(ctx, userName, "fakerepo", &api.HeadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to head object in non exit branch", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: "mockref",
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden head object in others", func() {
				resp, err := client.HeadObject(ctx, "jimmy", "happygo", &api.HeadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "c/d.txt",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to head object", func() {
				resp, err := client.HeadObject(ctx, userName, repoName, &api.HeadObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
				etag := resp.Header.Get("ETag")
				convey.So(etag, convey.ShouldEqual, `"0ee0646c1c77d8131cc8f4ee65c7673b"`)
			})
		})

		c.Convey("get object", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("fail to get object in non exit user", func() {
				resp, err := client.GetObject(ctx, "mock user", repoName, &api.GetObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit repo", func() {
				resp, err := client.GetObject(ctx, userName, "fakerepo", &api.GetObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("fail to get object in non exit branch", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: "mockref",
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("forbidden get object in others", func() {
				resp, err := client.GetObject(ctx, "jimmy", "happygo", &api.GetObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("empty path", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: refName,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit path", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: refName,
					Path:   "c/d.txt",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("success to get object", func() {
				resp, err := client.GetObject(ctx, userName, repoName, &api.GetObjectParams{
					Branch: refName,
					Path:   "a.bin",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				reader := hash.NewHashingReader(resp.Body, hash.Md5)
				data, err := io.ReadAll(reader)
				fmt.Println(data)
				convey.So(err, convey.ShouldBeNil)
				etag := resp.Header.Get("ETag")

				exectEtag := fmt.Sprintf(`"%s"`, hex.EncodeToString(reader.Md5.Sum(nil)))
				convey.So(etag, convey.ShouldEqual, exectEtag)
			})
		})
	}
}
