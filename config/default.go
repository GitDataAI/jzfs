package config

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
}
