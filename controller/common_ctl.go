package controller

import (
	"context"
	"net/http"

	"github.com/go-openapi/swag"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/version"
	"go.uber.org/fx"
)

var commonLog = logging.Logger("common")

type CommonController struct {
	fx.In

	VersionChecker version.IChecker
	Config         *config.Config
}

func (c CommonController) GetVersion(_ context.Context, w *api.JiaozifsResponse, _ *http.Request) {
	swagger, err := api.GetSwagger()
	if err != nil {
		w.Error(err)
		return
	}

	latestVersionResp, err := c.VersionChecker.CheckLatestVersion()
	if err == nil {
		commonLog.Errorf("fetch latest version failed: %v", err)
	}

	var latestVersion *string
	if latestVersionResp != nil {
		latestVersion = utils.String(latestVersionResp.LatestVersion)
	}

	w.JSON(api.VersionResult{
		ApiVersion:    swagger.Info.Version,
		Version:       version.UserVersion(),
		LatestVersion: latestVersion,
	})
}

func newLoginConfig(c *config.AuthConfig) *api.LoginConfig {
	return &api.LoginConfig{
		RBAC:               (*api.LoginConfigRBAC)(&c.UIConfig.RBAC),
		LoginUrl:           c.UIConfig.LoginURL,
		LoginFailedMessage: &c.UIConfig.LoginFailedMessage,
		FallbackLoginUrl:   c.UIConfig.FallbackLoginURL,
		FallbackLoginLabel: c.UIConfig.FallbackLoginLabel,
		LoginCookieNames:   c.UIConfig.LoginCookieNames,
		LogoutUrl:          c.UIConfig.LogoutURL,
	}
}

func (c CommonController) GetSetupState(_ context.Context, w *api.JiaozifsResponse, _ *http.Request) {
	state := api.SetupState{
		State:            (*api.SetupStateState)(swag.String("initialized")),
		LoginConfig:      newLoginConfig(&c.Config.Auth),
		CommPrefsMissing: swag.Bool(false),
	}
	w.JSON(state)
}
