package integrationtest

import (
	"context"
	"fmt"
	"net/http"

	"github.com/jiaozifs/jiaozifs/auth/rbac"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/smartystreets/goconvey/convey"
)

func MemberSpec(ctx context.Context, urlStr string) func(c convey.C) {
	client, _ := api.NewClient(urlStr + apiimpl.APIV1Prefix)

	user1Name := "group1test"
	testRepoName := "repo1test"

	user2Name := "group2test"
	testRepo2Name := "repo2test"

	var user1, user2 *api.UserInfo
	var repo1, repo2 *api.Repository
	var adminGroup, writeGroup, readGroup *api.Group

	var user1Token, user2Token []api.RequestEditorFn
	return func(c convey.C) {
		c.Convey("init test", func(c convey.C) {
			var err error
			createUser(ctx, client, user1Name)
			user1Token, err = getToken(ctx, client, user1Name)
			convey.ShouldBeNil(c, err)
			client.RequestEditors = user1Token

			createRepo(ctx, client, testRepoName)
			user1, err = getUser(ctx, client)
			convey.ShouldBeNil(c, err)
			repo1, err = getRepo(ctx, client, user1Name, testRepoName)
			convey.ShouldBeNil(c, err)

			client.RequestEditors = nil
			createUser(ctx, client, user2Name)
			user2Token, err = getToken(ctx, client, user2Name)
			convey.ShouldBeNil(c, err)
			client.RequestEditors = user2Token

			createRepo(ctx, client, testRepo2Name)
			user2, err = getUser(ctx, client)
			convey.ShouldBeNil(c, err)
			repo2, err = getRepo(ctx, client, user2Name, testRepo2Name)
			convey.ShouldBeNil(c, err)

			readGroup, writeGroup, adminGroup, err = getGroup(ctx, client)
			convey.ShouldNotBeNil(adminGroup)
			convey.ShouldNotBeNil(writeGroup)
			convey.ShouldBeNil(c, err)
		})

		c.Convey("invite member", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.InviteMember(ctx, user2.Name, repo2.Name, &api.InviteMemberParams{
					UserId:  user1.Id,
					GroupId: readGroup.Id,
				})
				client.RequestEditors = re
				convey.ShouldBeNil(c, err)
				convey.ShouldBeNil(c, resp)
			})
			c.Convey("invite self", func() {
				resp, err := client.InviteMember(ctx, user2.Name, repo2.Name, &api.InviteMemberParams{
					UserId:  user2.Id,
					GroupId: readGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusBadRequest)
			})

			c.Convey("not exit owner", func() {
				resp, err := client.InviteMember(ctx, "fake_owner", repo2.Name, &api.InviteMemberParams{
					UserId:  user1.Id,
					GroupId: readGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})
			c.Convey("not exit repo", func() {
				resp, err := client.InviteMember(ctx, user2.Name, "fake_repo", &api.InviteMemberParams{
					UserId:  user1.Id,
					GroupId: readGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("invite for other repo", func() {
				resp, err := client.InviteMember(ctx, user1.Name, repo1.Name, &api.InviteMemberParams{
					UserId:  user1.Id,
					GroupId: readGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("cannot read permission before granted", func() {
				client.RequestEditors = user1Token
				resp, err := client.GetRepository(ctx, user2.Name, repo2.Name)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
				client.RequestEditors = user2Token
			})

			c.Convey("invite success", func() {
				resp, err := client.InviteMember(ctx, user2.Name, repo2.Name, &api.InviteMemberParams{
					UserId:  user1.Id,
					GroupId: readGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
			})

			c.Convey("invite duplicate", func() {
				resp, err := client.InviteMember(ctx, user2.Name, repo2.Name, &api.InviteMemberParams{
					UserId:  user1.Id,
					GroupId: readGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusInternalServerError)
			})

			c.Convey("check read permission was granted", func() {
				client.RequestEditors = user1Token
				resp, err := client.GetRepository(ctx, user2.Name, repo2.Name)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
				client.RequestEditors = user2Token
			})

			c.Convey("cannot write permission with read grant", func() {
				client.RequestEditors = user1Token
				resp, err := client.CreateBranch(ctx, user2.Name, repo2.Name, api.CreateBranchJSONRequestBody{
					Name:   "testbranch",
					Source: "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
				client.RequestEditors = user2Token
			})
		})

		c.Convey("update member", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.UpdateMemberGroup(ctx, user2.Name, repo2.Name, &api.UpdateMemberGroupParams{
					UserId:  user1.Id,
					GroupId: writeGroup.Id,
				})
				client.RequestEditors = re
				convey.ShouldBeNil(c, err)
				convey.ShouldBeNil(c, resp)
			})

			c.Convey("not exit owner", func() {
				resp, err := client.UpdateMemberGroup(ctx, "fake_owner", repo2.Name, &api.UpdateMemberGroupParams{
					UserId:  user1.Id,
					GroupId: writeGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("not exit repo", func() {
				resp, err := client.UpdateMemberGroup(ctx, user2.Name, "mock_repo", &api.UpdateMemberGroupParams{
					UserId:  user1.Id,
					GroupId: writeGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update for other repo", func() {
				resp, err := client.UpdateMemberGroup(ctx, user1.Name, repo1.Name, &api.UpdateMemberGroupParams{
					UserId:  user1.Id,
					GroupId: writeGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("update success", func() {
				fmt.Println(readGroup.Id.String())
				fmt.Println(writeGroup.Id.String())
				resp, err := client.UpdateMemberGroup(ctx, user2.Name, repo2.Name, &api.UpdateMemberGroupParams{
					UserId:  user1.Id,
					GroupId: writeGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})

			c.Convey("write permission with write grant", func() {
				client.RequestEditors = user1Token
				resp, err := client.CreateBranch(ctx, user2.Name, repo2.Name, api.CreateBranchJSONRequestBody{
					Name:   "testbranch",
					Source: "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusCreated)
				client.RequestEditors = user2Token
			})
		})

		c.Convey("revoke member", func(c convey.C) {
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.RevokeMember(ctx, user2.Name, repo2.Name, &api.RevokeMemberParams{
					UserId: user1.Id,
				})
				client.RequestEditors = re
				convey.ShouldBeNil(c, err)
				convey.ShouldBeNil(c, resp)
			})

			c.Convey("not exit owner", func() {
				resp, err := client.RevokeMember(ctx, "fake_owner", repo2.Name, &api.RevokeMemberParams{
					UserId: user1.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("not exit repo", func() {
				resp, err := client.RevokeMember(ctx, user2.Name, "mock_repo", &api.RevokeMemberParams{
					UserId: user1.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("update for other repo", func() {
				resp, err := client.RevokeMember(ctx, user1.Name, repo1.Name, &api.RevokeMemberParams{
					UserId: user1.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("update success", func() {
				resp, err := client.RevokeMember(ctx, user2.Name, repo2.Name, &api.RevokeMemberParams{
					UserId: user1.Id,
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)
			})

			c.Convey("write permission with write grant", func() {
				client.RequestEditors = user1Token
				resp, err := client.CreateBranch(ctx, user2.Name, repo2.Name, api.CreateBranchJSONRequestBody{
					Name:   "testbranch",
					Source: "main",
				})
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
				client.RequestEditors = user2Token
			})
		})

		c.Convey("list member", func(c convey.C) {
			c.Convey("init", func() {
				_, err := client.InviteMember(ctx, user2.Name, repo2.Name, &api.InviteMemberParams{
					UserId:  user1.Id,
					GroupId: readGroup.Id,
				})
				convey.So(err, convey.ShouldBeNil)
			})
			c.Convey("no auth", func() {
				re := client.RequestEditors
				client.RequestEditors = nil
				resp, err := client.ListMembers(ctx, user2.Name, repo2.Name)
				client.RequestEditors = re
				convey.ShouldBeNil(c, err)
				convey.ShouldBeNil(c, resp)
			})

			c.Convey("not exit owner", func() {
				resp, err := client.ListMembers(ctx, "fake_owner", repo2.Name)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("not exit repo", func() {
				resp, err := client.ListMembers(ctx, user2.Name, "mock_repo")
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusNotFound)
			})

			c.Convey("list for other repo", func() {
				resp, err := client.ListMembers(ctx, user1.Name, repo1.Name)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusUnauthorized)
			})

			c.Convey("update success", func() {
				resp, err := client.ListMembers(ctx, user2.Name, repo2.Name)
				convey.So(err, convey.ShouldBeNil)
				convey.So(resp.StatusCode, convey.ShouldEqual, http.StatusOK)

				result, err := api.ParseListMembersResponse(resp)
				convey.So(err, convey.ShouldBeNil)
				convey.ShouldHaveLength(1, len(*result.JSON200))
			})
		})
	}
}

func getUser(ctx context.Context, client *api.Client) (*api.UserInfo, error) {
	resp, err := client.GetUserInfo(ctx)
	if err != nil {
		return nil, err
	}
	result, err := api.ParseGetUserInfoResponse(resp)
	if err != nil {
		return nil, err
	}
	return result.JSON200, nil
}

func getGroup(ctx context.Context, client *api.Client) (*api.Group, *api.Group, *api.Group, error) {
	resp, err := client.ListRepoGroup(ctx)
	if err != nil {
		return nil, nil, nil, err
	}
	result, err := api.ParseListRepoGroupResponse(resp)
	if err != nil {
		return nil, nil, nil, err
	}
	var adminGroup, writeGroup, readGroup api.Group
	for _, g := range *result.JSON200 {
		if g.Name == rbac.RepoAdmin {
			adminGroup = g
		}

		if g.Name == rbac.RepoWrite {
			writeGroup = g
		}

		if g.Name == rbac.RepoRead {
			readGroup = g
		}
	}

	return &readGroup, &writeGroup, &adminGroup, nil
}

func getRepo(ctx context.Context, client *api.Client, owner, repoName string) (*api.Repository, error) {
	resp, err := client.GetRepository(ctx, owner, repoName)
	if err != nil {
		return nil, err
	}
	result, err := api.ParseGetRepositoryResponse(resp)
	if err != nil {
		return nil, err
	}
	return result.JSON200, nil
}
func getToken(ctx context.Context, client *api.Client, userName string) ([]api.RequestEditorFn, error) {
	resp, err := client.Login(ctx, api.LoginJSONRequestBody{
		Name:     userName,
		Password: "12345678",
	})
	if err != nil {
		return nil, err
	}
	loginResult, err := api.ParseLoginResponse(resp)
	if err != nil {
		return nil, err
	}

	return []api.RequestEditorFn{func(ctx context.Context, req *http.Request) error {
		req.Header.Add("Authorization", "Bearer "+loginResult.JSON200.Token)
		return nil
	}}, nil
}
