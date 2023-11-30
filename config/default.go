package config

var DefaultLocalBSPath = "~/.jiaozifs/blockstore"

var defaultCfg = Config{
	Path: "~/.jiaozifs/config.toml",
	Log: LogConfig{
		Level: "INFO",
	},
	API: APIConfig{
		Listen: "http://127.0.0.1:34913",
	},
	Auth: AuthConfig{
		SecretKey: []byte("THIS_MUST_BE_CHANGED_IN_PRODUCTION"),
	},
	Blockstore: BlockStoreConfig{
		Type:                   "local",
		DefaultNamespacePrefix: nil,
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
}
