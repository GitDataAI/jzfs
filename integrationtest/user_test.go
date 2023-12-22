package integrationtest

import (
	"context"
	"net/http"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
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
