package config

import (
	"encoding/hex"

	"github.com/jiaozifs/jiaozifs/utils"
)

var DefaultLocalBSPath = "~/.jiaozifs/blockstore"

var defaultCfg = Config{
	Path: "~/.jiaozifs/config.toml",
	Log: LogConfig{
		Level: "INFO",
	},
	API: APIConfig{
		Listen: "http://127.0.0.1:34913",
	},
	Blockstore: BlockStoreConfig{
		Type:                   "local",
		DefaultNamespacePrefix: utils.String("data"),
		Local: (*struct {
			Path                    string   `mapstructure:"path"`
			ImportEnabled           bool     `mapstructure:"import_enabled"`
			ImportHidden            bool     `mapstructure:"import_hidden"`
			AllowedExternalPrefixes []string `mapstructure:"allowed_external_prefixes"`
		})(&struct {
			Path                    string
			ImportEnabled           bool
			ImportHidden            bool
			AllowedExternalPrefixes []string
		}{Path: DefaultLocalBSPath, ImportEnabled: false, ImportHidden: false, AllowedExternalPrefixes: nil}),
	},
	Auth: AuthConfig{
		SecretKey: hex.EncodeToString([]byte("THIS_MUST_BE_CHANGED_IN_PRODUCTION")),
		UIConfig: struct {
			RBAC               string   `mapstructure:"rbac"`
			LoginURL           string   `mapstructure:"login_url"`
			LoginFailedMessage string   `mapstructure:"login_failed_message"`
			FallbackLoginURL   *string  `mapstructure:"fallback_login_url"`
			FallbackLoginLabel *string  `mapstructure:"fallback_login_label"`
			LoginCookieNames   []string `mapstructure:"login_cookie_names"`
			LogoutURL          string   `mapstructure:"logout_url"`
		}{RBAC: AuthRBACSimplified,
			LoginURL:           "api/v1/auth/login",
			LoginFailedMessage: "",
			LoginCookieNames:   nil,
			LogoutURL:          "auth/logout",
		},
	},
}
