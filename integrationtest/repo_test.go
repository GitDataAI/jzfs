package integrationtest

import (
	"context"
	"net/http"
	"strconv"

	"github.com/GitDataAI/jiaozifs/api"
	apiimpl "github.com/GitDataAI/jiaozifs/api/api_impl"
	"github.com/GitDataAI/jiaozifs/controller"
	"github.com/GitDataAI/jiaozifs/utils"
	"github.com/smartystreets/goconvey/convey"
)

func RepoSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)
	return func(c convey.C) {
		userName := "jimmy"
		repoName := "happyrun"

		c.Convey("init", func(_ convey.C) {
			loginAndSwitch(ctx, client, "admin2", true)
			_ = createRepo(ctx, client, "admin2_repo", false)

			_ = createUser(ctx, client, userName)
			loginAndSwitch(ctx, client, userName, false)
		})
		c.Convey("create repo", func(c convey.C) {
			c.Convey("forbidden create repo name", func() {
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description: utils.String("test resp"),
					Name:        "repo",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("config error", func() {
				cfg := `{"Type":"local",DefaultNamespacePrefix":null,"Local":{"Path":"~/.jiaozifs/blockstore","ImportEnabled":false,"ImportHidden":false,"AllowedExternalPrefixes":null},"S3":null,"Azure":null,"GS":null}`
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description:      utils.String("test resp"),
					Name:             "happygo",
					BlockstoreConfig: utils.String(cfg),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("local not support", func() {
				cfg := `{"Type":"local","DefaultNamespacePrefix":null,"Local":{"Path":"~/.jiaozifs/blockstore","ImportEnabled":false,"ImportHidden":false,"AllowedExternalPrefixes":null},"S3":null,"Azure":null,"GS":null}`
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description:      utils.String("test resp"),
					Name:             "happygo",
					BlockstoreConfig: utils.String(cfg),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusForbidden)
			})

			c.Convey("success create repo name", func() {
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description: utils.String("test resp"),
					Name:        repoName,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)

				grp, err := api.ParseCreateRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(grp.JSON201.Head, convey.ShouldEqual, controller.DefaultBranchName)
				//check default branch created
				branchResp, err := client.GetBranch(ctx, userName, grp.JSON201.Name, &api.GetBranchParams{RefName: controller.DefaultBranchName})
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
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
			})

			c.Convey("duplicate repo name", func() {
				resp, err := client.CreateRepository(ctx, api.CreateRepository{
					Description: utils.String("test resp"),
					Name:        repoName,
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
				resp, err := client.ListRepositoryOfAuthenticatedUser(ctx, &api.ListRepositoryOfAuthenticatedUserParams{})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("list repository in authenticated user", func() {
				resp, err := client.ListRepositoryOfAuthenticatedUser(ctx, &api.ListRepositoryOfAuthenticatedUserParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 2)
			})

			c.Convey("success list repository of authenticatedUser and next page exists", func() {
				resp, err := client.ListRepositoryOfAuthenticatedUser(ctx, &api.ListRepositoryOfAuthenticatedUserParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 2)

				newResp, err := client.ListRepositoryOfAuthenticatedUser(ctx, &api.ListRepositoryOfAuthenticatedUserParams{
					After:  utils.Int64(listRepos.JSON200.Results[0].UpdatedAt),
					Amount: utils.Int(1),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(newResp.StatusCode, convey.ShouldEqual, http.StatusOK)

				newListRepos, err := api.ParseListRepositoryResponse(newResp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(newListRepos.JSON200.Pagination.HasMore, convey.ShouldBeTrue)
				convey.So(len(newListRepos.JSON200.Results), convey.ShouldEqual, 1)
			})

			c.Convey("success list repository of authenticatedUser, set page amount 0", func() {
				resp, err := client.ListRepositoryOfAuthenticatedUser(ctx, &api.ListRepositoryOfAuthenticatedUserParams{
					Amount: utils.Int(0),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 2)
			})

			c.Convey("list repository", func() {
				resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 2)
			})

			c.Convey("list repository by prefix", func() {
				resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{Prefix: utils.String("happy")})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 2)
			})

			c.Convey("success list repository and next page exists", func() {
				resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 2)

				newResp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{
					After:  utils.Int64(listRepos.JSON200.Results[0].UpdatedAt),
					Amount: utils.Int(1),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(newResp.StatusCode, convey.ShouldEqual, http.StatusOK)

				newListRepos, err := api.ParseListRepositoryResponse(newResp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(newListRepos.JSON200.Pagination.HasMore, convey.ShouldBeTrue)
				convey.So(len(newListRepos.JSON200.Results), convey.ShouldEqual, 1)
			})

			c.Convey("success list repository, set page amount 0", func() {
				resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{
					Amount: utils.Int(0),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 2)
			})

			c.Convey("list repository by prefix but found nothing", func() {
				resp, err := client.ListRepository(ctx, userName, &api.ListRepositoryParams{Prefix: utils.String("bad")})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				listRepos, err := api.ParseListRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)

				convey.So(len(listRepos.JSON200.Results), convey.ShouldEqual, 0)
			})

			c.Convey("list others repository", func() {
				resp, err := client.ListRepository(ctx, "admin2", &api.ListRepositoryParams{Prefix: utils.String("bad")})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
		})

		c.Convey("get repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetRepository(ctx, userName, repoName)
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("get repository", func() {
				resp, err := client.GetRepository(ctx, userName, repoName)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResult, err := api.ParseGetRepositoryResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(getResult.JSON200.Name, convey.ShouldEqual, repoName)
			})

			c.Convey("get not exit repo", func() {
				resp, err := client.GetRepository(ctx, userName, "happyrun_mock")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("get from non exit user", func() {
				resp, err := client.GetRepository(ctx, "telo", repoName)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("get other's repo", func() {
				resp, err := client.GetRepository(ctx, "admin2", "admin2_repo")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
		})

		c.Convey("update repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.UpdateRepository(ctx, userName, repoName, api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(""),
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success update repository", func() {
				description := "mock description"
				resp, err := client.UpdateRepository(ctx, userName, repoName, api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(description),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResp, err := client.GetRepository(ctx, userName, repoName)
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
				resp, err := client.UpdateRepository(ctx, "telo", repoName, api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(description),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update repository in other's repo", func() {
				description := ""
				resp, err := client.UpdateRepository(ctx, "admin2", "admin2_repo", api.UpdateRepositoryJSONRequestBody{
					Description: utils.String(description),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("update head to not exit", func() {
				resp, err := client.UpdateRepository(ctx, userName, repoName, api.UpdateRepositoryJSONRequestBody{
					Head: utils.String("xxx"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("create branch", func(_ convey.C) {
				createBranch(ctx, client, userName, repoName, "main", "feat/ano_branch")
			})

			c.Convey("update default head success", func() {
				resp, err := client.UpdateRepository(ctx, userName, repoName, api.UpdateRepositoryJSONRequestBody{
					Head: utils.String("feat/ano_branch"),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})
		})

		c.Convey("get commits in repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetCommitsInRef(ctx, userName, repoName, &api.GetCommitsInRefParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
			c.Convey("update repository in not exit repo", func() {
				resp, err := client.GetCommitsInRef(ctx, userName, "happyrunfake", &api.GetCommitsInRefParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update repository in non exit user", func() {
				resp, err := client.GetCommitsInRef(ctx, "telo", repoName, &api.GetCommitsInRefParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update repository in other's repo", func() {
				resp, err := client.GetCommitsInRef(ctx, "admin2", "admin2_repo", &api.GetCommitsInRefParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)

			})

			c.Convey("add commit to branch", func(_ convey.C) {
				createWip(ctx, client, userName, repoName, controller.DefaultBranchName)
				uploadObject(ctx, client, userName, repoName, controller.DefaultBranchName, "a.txt", true)
				_ = commitWip(ctx, client, userName, repoName, controller.DefaultBranchName, "first commit")
			})

			c.Convey("success get commits", func() {
				resp, err := client.GetCommitsInRef(ctx, userName, repoName, &api.GetCommitsInRefParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetCommitsInRefResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 1)
				convey.So((*result.JSON200)[0].Message, convey.ShouldEqual, "first commit")
			})

			c.Convey("add double commit to branch", func(_ convey.C) {
				uploadObject(ctx, client, userName, repoName, controller.DefaultBranchName, "b.txt", true)
				_ = commitWip(ctx, client, userName, repoName, controller.DefaultBranchName, "second commit")
				uploadObject(ctx, client, userName, repoName, controller.DefaultBranchName, "c.txt", true)
				_ = commitWip(ctx, client, userName, repoName, controller.DefaultBranchName, "third commit")
			})

			c.Convey("success get commits by params", func() {
				resp, err := client.GetCommitsInRef(ctx, userName, repoName, &api.GetCommitsInRefParams{
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetCommitsInRefResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*result.JSON200, convey.ShouldHaveLength, 3)
				convey.So((*result.JSON200)[0].Message, convey.ShouldEqual, "third commit")

				newResp, err := client.GetCommitsInRef(ctx, userName, repoName, &api.GetCommitsInRefParams{
					After:   utils.Int64((*result.JSON200)[0].Committer.When),
					Amount:  utils.Int(1),
					RefName: utils.String(controller.DefaultBranchName),
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				newResult, err := api.ParseGetCommitsInRefResponse(newResp)
				convey.So(err, convey.ShouldBeNil)
				convey.So(*newResult.JSON200, convey.ShouldHaveLength, 1)
				convey.So((*newResult.JSON200)[0].Message, convey.ShouldEqual, "second commit")
			})
		})

		c.Convey("delete repository", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.DeleteRepository(ctx, userName, repoName, &api.DeleteRepositoryParams{})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
			c.Convey("delete repository in not exit repo", func() {
				resp, err := client.DeleteRepository(ctx, userName, "happyrunfake", &api.DeleteRepositoryParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("delete repository in non exit user", func() {
				resp, err := client.DeleteRepository(ctx, "telo", repoName, &api.DeleteRepositoryParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("delete repository in other's repo", func() {
				resp, err := client.DeleteRepository(ctx, "admin2", "admin2_repo", &api.DeleteRepositoryParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("delete repository successful", func() {
				resp, err := client.DeleteRepository(ctx, userName, repoName, &api.DeleteRepositoryParams{})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				getResp, err := client.GetRepository(ctx, userName, repoName)
				convey.So(err, convey.ShouldBeNil)
				convey.So(getResp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})
		})

		c.Convey("get archive file", func(c convey.C) {
			userName2 := "testarchive"
			repo2Name := "archiverepo"
			refName := "testbranch"
			tag := "vt0.0.1"
			c.Convey("init", func() {
				loginAndSwitch(ctx, client, "admin2", true)
				_ = createUser(ctx, client, userName2)
				loginAndSwitch(ctx, client, userName2, false)
				_ = createRepo(ctx, client, repo2Name, false)
				_ = createBranch(ctx, client, userName2, repo2Name, "main", refName)
				_ = createWip(ctx, client, userName2, repo2Name, refName)
				_ = uploadObject(ctx, client, userName2, repo2Name, refName, "g.txt", true)
				_ = uploadObject(ctx, client, userName2, repo2Name, refName, "a/b.txt", true)
				_ = uploadObject(ctx, client, userName2, repo2Name, refName, "b/b.txt", true)
				_ = uploadObject(ctx, client, userName2, repo2Name, refName, "c/b.txt", true)
				_ = commitWip(ctx, client, userName2, repo2Name, refName, "aaa")
				_ = createTag(ctx, client, userName2, repo2Name, tag, refName)
			})
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.GetArchive(ctx, userName2, repo2Name, &api.GetArchiveParams{
					ArchiveType: api.Zip,
					RefName:     refName,
					RefType:     api.RefTypeBranch,
				})
				client.RequestEditors = re
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})
			c.Convey("get archive in not exit repo", func() {
				resp, err := client.GetArchive(ctx, userName2, "happyrunfake", &api.GetArchiveParams{
					ArchiveType: api.Zip,
					RefName:     refName,
					RefType:     api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("get archive in non exit user", func() {
				resp, err := client.GetArchive(ctx, "telo", repo2Name, &api.GetArchiveParams{
					ArchiveType: api.Zip,
					RefName:     refName,
					RefType:     api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("get archive in non exit ref", func() {
				resp, err := client.GetArchive(ctx, userName2, repo2Name, &api.GetArchiveParams{
					ArchiveType: api.Zip,
					RefName:     "gggg",
					RefType:     api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("get archive in non other repo", func() {
				resp, err := client.GetArchive(ctx, "jimmy", "happygo", &api.GetArchiveParams{
					ArchiveType: api.Zip,
					RefName:     "main",
					RefType:     api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("success get zip archive in branch", func() {
				resp, err := client.GetArchive(ctx, userName2, repo2Name, &api.GetArchiveParams{
					ArchiveType: api.Zip,
					RefName:     refName,
					RefType:     api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetArchiveResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				sizeStr := resp.Header.Get("Content-Length")
				size, err := strconv.Atoi(sizeStr)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldHaveLength(size, result.Body)
			})

			c.Convey("success get car archive in branch", func() {
				resp, err := client.GetArchive(ctx, userName2, repo2Name, &api.GetArchiveParams{
					ArchiveType: api.Car,
					RefName:     refName,
					RefType:     api.RefTypeBranch,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetArchiveResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				sizeStr := resp.Header.Get("Content-Length")
				size, err := strconv.Atoi(sizeStr)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldHaveLength(size, result.Body)
			})

			c.Convey("success get zip archive in tag", func() {
				resp, err := client.GetArchive(ctx, userName2, repo2Name, &api.GetArchiveParams{
					ArchiveType: api.Zip,
					RefName:     tag,
					RefType:     api.RefTypeTag,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseGetArchiveResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				sizeStr := resp.Header.Get("Content-Length")
				size, err := strconv.Atoi(sizeStr)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldHaveLength(size, result.Body)
			})

		})
	}
}
