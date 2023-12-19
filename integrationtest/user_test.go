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
		c.Convey("register", func() {
			resp, err := client.Register(ctx, api.RegisterJSONRequestBody{
				Username: userName,
				Password: "12345678",
				Email:    "mock@gmail.com",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, resp.StatusCode)
		})

		c.Convey("usr profile no cookie", func() {
			resp, err := client.GetUserInfo(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusForbidden, convey.ShouldEqual, resp.StatusCode)
		})

		c.Convey("login", func() {
			resp, err := client.Login(ctx, api.LoginJSONRequestBody{
				Username: "admin",
				Password: "12345678",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, resp.StatusCode)

			client.RequestEditors = append(client.RequestEditors, func(ctx context.Context, req *http.Request) error {
				for _, cookie := range resp.Cookies() {
					req.AddCookie(cookie)
				}
				return nil
			})
		})

		c.Convey("usr profile", func() {
			resp, err := client.GetUserInfo(ctx)
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, resp.StatusCode)
		})
	}
}
