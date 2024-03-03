package controller

import (
	"context"
	"net/http"

	"github.com/google/uuid"

	"github.com/jiaozifs/jiaozifs/api"

	"github.com/jiaozifs/jiaozifs/auth"

	"github.com/jiaozifs/jiaozifs/auth/rbac"
	"go.uber.org/fx"
)

type BaseController struct {
	fx.In

	PermissionCheck rbac.PermissionCheck
}

func (c *BaseController) authorize(ctx context.Context, w *api.JiaozifsResponse, perms rbac.Node) bool {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Unauthorized()
		return false
	}

	resp, err := c.PermissionCheck.Authorize(ctx, &rbac.AuthorizationRequest{
		OperatorID:          operator.ID,
		RequiredPermissions: perms,
	})
	if err != nil {
		w.String(err.Error(), http.StatusInternalServerError)
		return false
	}

	if resp.Error != nil {
		w.Code(http.StatusUnauthorized)
		return false
	}
	if !resp.Allowed {
		w.String("User does not have the required permissions", http.StatusInternalServerError)
		return false
	}
	return true
}

func (c *BaseController) authorizeMember(ctx context.Context, w *api.JiaozifsResponse, repoID uuid.UUID, perms rbac.Node) bool {
	operator, err := auth.GetOperator(ctx)
	if err != nil {
		w.Unauthorized()
		return false
	}

	resp, err := c.PermissionCheck.AuthorizeMember(ctx, repoID, &rbac.AuthorizationRequest{
		OperatorID:          operator.ID,
		RequiredPermissions: perms,
	})
	if err != nil {
		w.String(err.Error(), http.StatusInternalServerError)
		return false
	}

	if resp.Error != nil {
		w.Code(http.StatusUnauthorized)
		return false
	}
	if !resp.Allowed {
		w.String("User does not have the required permissions", http.StatusInternalServerError)
		return false
	}
	return true
}
