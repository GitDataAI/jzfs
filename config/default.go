package config

import "github.com/jiaozifs/jiaozifs/utils"

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
		SecretKey: []byte("THIS_MUST_BE_CHANGED_IN_PRODUCTION"),
	},
}
