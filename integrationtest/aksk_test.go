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

		c.Convey("init", func(c convey.C) {
			createUser(ctx, client, userName)
			loginAndSwitch(ctx, client, userName, false)
		})

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
				convey.ShouldEqual(akskResult.JSON200.Description, aksk.Description)
			})

			c.Convey("get ak by ak", func() {
				resp, err := client.GetAksk(ctx, &api.GetAkskParams{AccessKey: &aksk.AccessKey})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				akskResult, err := api.ParseGetAkskResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldEqual(akskResult.JSON200.Id, aksk.Id)
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
				aksk := createAksk(ctx, client)

				resp, err := client.DeleteAksk(ctx, &api.DeleteAkskParams{Id: &aksk.Id})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})

			c.Convey("delete ak by ak", func() {
				aksk := createAksk(ctx, client)

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
				_ = createAksk(ctx, client)
				_ = createAksk(ctx, client)
				_ = createAksk(ctx, client)
				_ = createAksk(ctx, client)
				_ = createAksk(ctx, client)
			})
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.ListAksks(ctx, &api.ListAksksParams{})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("list aksk success", func() {
				resp, err := client.ListAksks(ctx, &api.ListAksksParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseListAksksResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldHaveLength(result.JSON200, 6)
			})
		})

		c.Convey("aksk user", func(c convey.C) {
			c.Convey("success", func(c convey.C) {
				aksk := createAksk(ctx, client)

				cli, err := api.NewClient(urlStr+apiimpl.APIV1Prefix, api.AkSkOption(aksk.AccessKey, aksk.SecretKey))
				convey.So(err, convey.ShouldBeNil)

				resp, err := cli.GetUserInfo(ctx)
				convey.So(err, convey.ShouldBeNil)

				user, err := api.ParseGetUserInfoResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldEqual(user.JSON200.Name, userName)
			})
			c.Convey("wrong sk", func(c convey.C) {
				aksk := createAksk(ctx, client)

				client, err := api.NewClient(urlStr+apiimpl.APIV1Prefix, api.AkSkOption(aksk.AccessKey, "fakesk"))
				convey.So(err, convey.ShouldBeNil)

				resp, err := client.GetUserInfo(ctx)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldEqual(resp.StatusCode, http.StatusUnauthorized)
			})

			c.Convey("ak not exit", func(c convey.C) {
				client, err := api.NewClient(urlStr+apiimpl.APIV1Prefix, api.AkSkOption("fakesk", "fakesk"))
				convey.So(err, convey.ShouldBeNil)

				resp, err := client.GetUserInfo(ctx)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldEqual(resp.StatusCode, http.StatusUnauthorized)
			})
		})
	}
}
