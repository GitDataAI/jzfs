package integrationtest

import (
	"context"
	"net/http"

	openapi_types "github.com/oapi-codegen/runtime/types"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func UserSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "admin2"

		c.Convey("init user", func(c convey.C) {
			createUser(ctx, client, userName)
		})

		c.Convey("invalid username", func() {
			resp, err := client.Register(ctx, api.RegisterJSONRequestBody{
				Name:     "admin!@#",
				Password: "12345678",
				Email:    openapi_types.Email("mock123@gmail.com"),
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
		})

		c.Convey("usr profile no cookie", func() {
			resp, err := client.GetUserInfo(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
		})

		c.Convey("login fail", func() {
			resp, err := client.Login(ctx, api.LoginJSONRequestBody{
				Name:     "admin2",
				Password: " vvvvvvvv",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
		})

		c.Convey("admin login", func(c convey.C) {
			loginAndSwitch(ctx, client, userName, false)
		})

		c.Convey("usr profile", func() {
			resp, err := client.GetUserInfo(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
		})

		c.Convey("admin login again", func(c convey.C) {
			loginAndSwitch(ctx, client, userName, true)
		})

		c.Convey("usr profile with cookie", func() {
			resp, err := client.GetUserInfo(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
		})
		c.Convey("refresh token", func() {
			resp, err := client.RefreshToken(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

			_, err = api.ParseRefreshTokenResponse(resp)
			convey.So(err, convey.ShouldBeNil)
		})

		c.Convey("no auth refresh", func() {
			re := client.RequestEditors
			client.RequestEditors = nil
			resp, err := client.RefreshToken(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			client.RequestEditors = re
		})

	}
}
