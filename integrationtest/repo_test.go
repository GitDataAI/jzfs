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
		CreateUser(ctx, c, client, userName)
		LoginAndSwitch(ctx, c, client, userName)

		c.Convey("create repo", func(c convey.C) {
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

			c.Convey("duplicate repo name", func() {
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description: utils.String("test resp"),
					Name:        "happyrun",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusInternalServerError)
			})

			c.Convey("invalid repo name", func() {
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description: utils.String("test resp"),
					Name:        "happyrun1@#%",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description: utils.String("test resp"),
					Name:        "happyrun2",
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
		})

		c.Convey("list repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.ListRepositoryOfAuthenticatedUser(ctx)
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("list repository in authenticated user", func() {
				resp, err := client.ListRepositoryOfAuthenticatedUser(ctx)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(*listRepos.JSON200), convey.ShouldEqual, 2)
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

			c.Convey("list others repository", func() {
				resp, err := client.ListRepository(ctx, "admin", &api.ListRepositoryParams{RepoPrefix: utils.String("bad")})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})
		})

		c.Convey("get repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetRepository(ctx, userName, "happyrun")
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("get repository", func() {
				resp, err := client.GetRepository(ctx, userName, "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResult, err := api.ParseGetRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(getResult.JSON200.Name, convey.ShouldEqual, "happyrun")
			})

			c.Convey("get not exit repo", func() {
				resp, err := client.GetRepository(ctx, userName, "happyrun_mock")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("get from non exit user", func() {
				resp, err := client.GetRepository(ctx, "telo", "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("get other's repo", func() {
				resp, err := client.GetRepository(ctx, "admin", "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})
		})

		c.Convey("update repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.UpdateRepository(ctx, userName, "happyrun", api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(""),
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success update repository", func() {
				description := "mock description"
				resp, err := client.UpdateRepository(ctx, userName, "happyrun", api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(description),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResp, err := client.GetRepository(ctx, userName, "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(getResp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResult, err := api.ParseGetRepositoryResponse(getResp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*getResult.JSON200.Description, convey.ShouldEqual, description)
			})

			c.Convey("update repository in not exit repo", func() {
				description := ""
				resp, err := client.UpdateRepository(ctx, userName, "happyrunfake", api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(description),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update repository in non exit user", func() {
				description := ""
				resp, err := client.UpdateRepository(ctx, "telo", "happyrun", api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(description),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update repository in other's repo", func() {
				description := ""
				resp, err := client.UpdateRepository(ctx, "admin", "happyrun", api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(description),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})
		})

		c.Convey("get commits in repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetCommitsInRepository(ctx, userName, "happyrun", &api.GetCommitsInRepositoryParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success get commits", func() {
				resp, err := client.GetCommitsInRepository(ctx, userName, "happyrun", &api.GetCommitsInRepositoryParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetCommitsInRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(result.JSON200, convey.ShouldNotBeNil)
				convey.So(len(*result.JSON200), convey.ShouldEqual, 0)
			})

			c.Convey("update repository in not exit repo", func() {
				resp, err := client.GetCommitsInRepository(ctx, userName, "happyrunfake", &api.GetCommitsInRepositoryParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update repository in non exit user", func() {
				resp, err := client.GetCommitsInRepository(ctx, "telo", "happyrun", &api.GetCommitsInRepositoryParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update repository in other's repo", func() {
				resp, err := client.GetCommitsInRepository(ctx, "admin", "happyrun", &api.GetCommitsInRepositoryParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})
		})

		c.Convey("delete repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.DeleteRepository(ctx, userName, "happyrun")
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
			c.Convey("delete repository in not exit repo", func() {
				resp, err := client.DeleteRepository(ctx, userName, "happyrunfake")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("delete repository in non exit user", func() {
				resp, err := client.DeleteRepository(ctx, "telo", "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("delete repository in other's repo", func() {
				resp, err := client.DeleteRepository(ctx, "admin", "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("delete repository successful", func() {
				resp, err := client.DeleteRepository(ctx, userName, "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResp, err := client.GetRepository(ctx, userName, "happyrun")
				convey.So(err, convey.ShouldBeNil)
				convey.So(getResp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})
		})
	}
}
