package controller

import (
	"context"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/models/rbacmodel"
	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth/rbac"
	"github.com/jiaozifs/jiaozifs/models"
	"go.uber.org/fx"
)

type MemberController struct {
	fx.In
	BaseController

	Repo models.IRepo
}

func (memberCtl MemberController) UpdateMemberGroup(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.UpdateMemberGroupParams) {
	owner, err := memberCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := memberCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}
	if !memberCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.AddGroupMemberAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	listMemberParams := models.NewUpdateMemberParams().SetFilterUserID(params.UserId).SetFilterRepoID(repository.ID).SetUpdateGroupID(params.GroupId)
	err = memberCtl.Repo.MemberRepo().UpdateMember(ctx, listMemberParams)
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (memberCtl MemberController) InviteMember(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.InviteMemberParams) {
	owner, err := memberCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := memberCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}
	if !memberCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.AddGroupMemberAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	if owner.ID == params.UserId {
		w.BadRequest("not need to invite self")
	}

	// todo user need to confirm?
	member, err := memberCtl.Repo.MemberRepo().Insert(ctx, &models.Member{
		UserID:    params.UserId,
		RepoID:    repository.ID,
		GroupID:   params.GroupId,
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
	})
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(utils.Silent(memberToDto(member)), http.StatusCreated)

}

func (memberCtl MemberController) RevokeMember(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string, params api.RevokeMemberParams) {
	owner, err := memberCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := memberCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}
	if !memberCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.RemoveGroupMemberAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	_, err = memberCtl.Repo.MemberRepo().DeleteMember(ctx, models.NewDeleteMemberParams().SetRepoID(repository.ID).SetUserID(params.UserId))
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (memberCtl MemberController) ListMembers(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, ownerName string, repositoryName string) {
	owner, err := memberCtl.Repo.UserRepo().Get(ctx, models.NewGetUserParams().SetName(ownerName))
	if err != nil {
		w.Error(err)
		return
	}

	repository, err := memberCtl.Repo.RepositoryRepo().Get(ctx, models.NewGetRepoParams().SetName(repositoryName).SetOwnerID(owner.ID))
	if err != nil {
		w.Error(err)
		return
	}
	if !memberCtl.authorizeMember(ctx, w, repository.ID, rbac.Node{
		Permission: rbac.Permission{
			Action:   rbacmodel.ListGroupMemberAction,
			Resource: rbacmodel.RepoURArn(owner.ID.String(), repository.ID.String()),
		},
	}) {
		return
	}

	listMemberParams := models.NewListMembersParams().SetRepoID(repository.ID)
	members, err := memberCtl.Repo.MemberRepo().ListMember(ctx, listMemberParams)
	if err != nil {
		w.Error(err)
		return
	}
	w.JSON(utils.Silent(utils.ArrMap(members, memberToDto)))
}

func memberToDto(m *models.Member) (api.Member, error) {
	return api.Member{
		CreatedAt: m.CreatedAt.UnixMilli(),
		GroupId:   m.GroupID,
		Id:        m.ID,
		RepoId:    m.RepoID,
		UpdatedAt: m.UpdatedAt.UnixMilli(),
		UserId:    m.UserID,
	}, nil
}
