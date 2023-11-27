package config

var defaultCfg = Config{
	Path: "~/.jiaozifs/config.toml",
	Log: LogConfig{
		Level: "INFO",
	},
	API: APIConfig{
		Listen: "0.0.0.0:34913",
	},
}
