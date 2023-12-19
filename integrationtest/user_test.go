package integrationtest

import (
	"context"
	"net/http"

	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/smartystreets/goconvey/convey"
)

func UserSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "admin"
		createUser(ctx, c, client, userName)

		c.Convey("usr profile no cookie", func() {
			resp, err := client.GetUserInfo(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
		})

		c.Convey("login fail", func() {
			resp, err := client.Login(ctx, api.LoginJSONRequestBody{
				Username: "admin",
				Password: " vvvvvvvv",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
		})

		loginAndSwitch(ctx, c, client, userName)

		c.Convey("usr profile", func() {
			resp, err := client.GetUserInfo(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
		})
	}
}

func createUser(ctx context.Context, c convey.C, client *api.Client, userName string) {
	c.Convey("register "+userName, func() {
		resp, err := client.Register(ctx, api.RegisterJSONRequestBody{
			Username: userName,
			Password: "12345678",
			Email:    "mock@gmail.com",
		})
		convey.So(err, convey.ShouldBeNil)
		convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
	})
}

func loginAndSwitch(ctx context.Context, c convey.C, client *api.Client, userName string) {
	c.Convey("login "+userName, func() {
		resp, err := client.Login(ctx, api.LoginJSONRequestBody{
			Username: userName,
			Password: "12345678",
		})
		convey.So(err, convey.ShouldBeNil)
		convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

		client.RequestEditors = nil
		client.RequestEditors = append(client.RequestEditors, func(ctx context.Context, req *http.Request) error {
			for _, cookie := range resp.Cookies() {
				req.AddCookie(cookie)
			}
			return nil
		})
	})
}
