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
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

			loginResp, err := client.Login(ctx, api.LoginJSONRequestBody{
				Username: userName,
				Password: "12345678",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(loginResp.StatusCode, convey.ShouldEqual, http.StatusOK)

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
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
		})

		c.Convey("success create repo name", func() {
			resp, err := client.CreateRepository(ctx, api.CreateRepository{
				Description: utils.String("test resp"),
				Name:        "happyrun",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

			grp, err := api.ParseGetRepositoryResponse(resp)
			convey.So(err, convey.ShouldBeNil)
			convey.So(grp.JSON200.Head, convey.ShouldEqual, controller.DefaultBranchName)
			fmt.Println(grp.JSON200.ID)
			//check default branch created
			branchResp, err := client.GetBranch(ctx, userName, grp.JSON200.Name, &api.GetBranchParams{RefName: controller.DefaultBranchName})
			convey.So(err, convey.ShouldBeNil)
			convey.So(branchResp.StatusCode, convey.ShouldEqual, http.StatusOK)

			brp, err := api.ParseGetBranchResponse(branchResp)
			convey.So(err, convey.ShouldBeNil)
			convey.So(brp.JSON200.Name, convey.ShouldEqual, controller.DefaultBranchName)
		})

		c.Convey("add second repo ", func() {
			resp, err := client.CreateRepository(ctx, api.CreateRepository{
				Description: utils.String("test resp"),
				Name:        "happygo",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
		})

		c.Convey("duplicate repo", func() {
			resp, err := client.CreateRepository(ctx, api.CreateRepository{
				Description: utils.String("test resp"),
				Name:        "happyrun",
			})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusInternalServerError)
		})

		c.Convey("list repository", func() {
			resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

			listRepos, err := api.ParseListRepositoryResponse(resp)
			convey.So(err, convey.ShouldBeNil)

			convey.So(len(*listRepos.JSON200), convey.ShouldEqual, 2)
		})

		c.Convey("list repository by prefix", func() {
			resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{RepoPrefix: utils.String("happy")})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

			listRepos, err := api.ParseListRepositoryResponse(resp)
			convey.So(err, convey.ShouldBeNil)

			convey.So(len(*listRepos.JSON200), convey.ShouldEqual, 2)
		})

		c.Convey("list repository by prefix but found nothing", func() {
			resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{RepoPrefix: utils.String("bad")})
			convey.So(err, convey.ShouldBeNil)
			convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

			listRepos, err := api.ParseListRepositoryResponse(resp)
			convey.So(err, convey.ShouldBeNil)

			convey.So(len(*listRepos.JSON200), convey.ShouldEqual, 0)
		})
	}
}
