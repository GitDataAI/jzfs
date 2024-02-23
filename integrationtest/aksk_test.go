package integrationtest

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func AkSkSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	var aksk *api.Aksk
	return func(c convey.C) {
		userName := "muly"
		createUser(ctx, c, client, userName)
		loginAndSwitch(ctx, c, client, "muly login", userName, false)

		c.Convey("create aksk", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.CreateAksk(ctx, &api.CreateAkskParams{Description: utils.String("create ak sk")})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success create branch", func() {
				resp, err := client.CreateAksk(ctx, &api.CreateAkskParams{Description: utils.String("create ak sk")})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)

				akskResult, err := api.ParseCreateAkskResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				aksk = akskResult.JSON201
			})
		})

		c.Convey("get aksk", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetAksk(ctx, &api.GetAkskParams{Id: &aksk.Id})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("get ak by id", func() {
				resp, err := client.GetAksk(ctx, &api.GetAkskParams{Id: &aksk.Id})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				akskResult, err := api.ParseGetAkskResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldEqual(akskResult.JSON200.AccessKey, aksk.AccessKey)
				convey.ShouldEqual(akskResult.JSON200.SecretKey, aksk.SecretKey)
				convey.ShouldEqual(akskResult.JSON200.Description, aksk.Description)
			})

			c.Convey("get ak by ak", func() {
				resp, err := client.GetAksk(ctx, &api.GetAkskParams{AccessKey: &aksk.AccessKey})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				akskResult, err := api.ParseGetAkskResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldEqual(akskResult.JSON200.Id, aksk.Id)
				convey.ShouldEqual(akskResult.JSON200.SecretKey, aksk.SecretKey)
				convey.ShouldEqual(akskResult.JSON200.Description, aksk.Description)
			})
			c.Convey("get nothing by ak", func() {
				resp, err := client.GetAksk(ctx, &api.GetAkskParams{AccessKey: utils.String("aaaa")})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})
		})

		c.Convey("delete aksk", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.DeleteAksk(ctx, &api.DeleteAkskParams{Id: &aksk.Id})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("delete ak by id", func() {
				aksk, err := createAksk(ctx, client)
				convey.So(err, convey.ShouldBeNil)

				resp, err := client.DeleteAksk(ctx, &api.DeleteAkskParams{Id: &aksk.Id})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})

			c.Convey("delete ak by ak", func() {
				aksk, err := createAksk(ctx, client)
				convey.So(err, convey.ShouldBeNil)

				resp, err := client.DeleteAksk(ctx, &api.DeleteAkskParams{AccessKey: &aksk.AccessKey})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})
			c.Convey("delete nothing by ak", func() {
				resp, err := client.DeleteAksk(ctx, &api.DeleteAkskParams{AccessKey: utils.String("fakekey")})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})
		})

		c.Convey("list aksk", func(c convey.C) {
			c.Convey("prepare aksk", func() {
				_, _ = createAksk(ctx, client)
				_, _ = createAksk(ctx, client)
				_, _ = createAksk(ctx, client)
				_, _ = createAksk(ctx, client)
				_, _ = createAksk(ctx, client)
			})
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.ListAksks(ctx, &api.ListAksksParams{})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("delete nothing by ak", func() {
				resp, err := client.ListAksks(ctx, &api.ListAksksParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseListAksksResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldHaveLength(result.JSON200, 6)
			})
		})
	}
}

func createAksk(ctx context.Context, client *api.Client) (*api.Aksk, error) {
	resp, err := client.CreateAksk(ctx, &api.CreateAkskParams{Description: utils.String("create ak sk")})
	if err != nil {
		return nil, err
	}

	akskResult, err := api.ParseCreateAkskResponse(resp)
	if err != nil {
		return nil, err
	}
	return akskResult.JSON201, nil
}
