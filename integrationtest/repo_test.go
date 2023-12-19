package integrationtest

import (
	"context"
	"fmt"
	"net/http"

	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"

	"github.com/jiaozifs/jiaozifs/controller"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/smartystreets/goconvey/convey"
)

func RepoSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "jimmy"
		c.Convey("create new user for repo", func() {
			resp, err := client.Register(ctx, api.RegisterJSONRequestBody{
				Username: userName,
				Password: "12345678",
				Email:    "mock@gmail.com",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, resp.StatusCode)

			loginResp, err := client.Login(ctx, api.LoginJSONRequestBody{
				Username: userName,
				Password: "12345678",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, loginResp.StatusCode)

			client.RequestEditors = append(client.RequestEditors, func(ctx context.Context, req *http.Request) error {
				for _, cookie := range loginResp.Cookies() {
					req.AddCookie(cookie)
				}
				return nil
			})
		})

		c.Convey("forbidden create repo name", func() {
			resp, err := client.CreateRepository(ctx, api.CreateRepository{
				Description: utils.String("test resp"),
				Name:        "repo",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusBadRequest, convey.ShouldEqual, resp.StatusCode)
		})

		c.Convey("success create repo name", func() {
			resp, err := client.CreateRepository(ctx, api.CreateRepository{
				Description: utils.String("test resp"),
				Name:        "happyrun",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, resp.StatusCode)

			grp, err := api.ParseGetRepositoryResponse(resp)
			convey.So(err, convey.ShouldBeNil)
			convey.So(controller.DefaultBranchName, convey.ShouldEqual, grp.JSON200.Head)
			fmt.Println(grp.JSON200.ID)
			//check default branch created
			branchResp, err := client.GetBranch(ctx, userName, grp.JSON200.Name, &api.GetBranchParams{RefName: controller.DefaultBranchName})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, branchResp.StatusCode)

			brp, err := api.ParseGetBranchResponse(branchResp)
			convey.So(err, convey.ShouldBeNil)
			convey.So(controller.DefaultBranchName, convey.ShouldEqual, brp.JSON200.Name)
		})

		c.Convey("duplicate repo", func() {
			resp, err := client.CreateRepository(ctx, api.CreateRepository{
				Description: utils.String("test resp"),
				Name:        "happyrun",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusInternalServerError, convey.ShouldEqual, resp.StatusCode)
		})

		c.Convey("list repository", func() {
			resp, err := client.ListRepository(ctx, userName)
			convey.So(err, convey.ShouldBeNil)
			convey.So(http.StatusOK, convey.ShouldEqual, resp.StatusCode)

			listRepos, err := api.ParseListRepositoryResponse(resp)
			convey.So(err, convey.ShouldBeNil)

			convey.So(len(*listRepos.JSON200), convey.ShouldEqual, 1)
		})
	}
}
