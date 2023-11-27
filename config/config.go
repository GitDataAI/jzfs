package config

type Config struct {
	Path string    `mapstructure:"config"`
	Log  LogConfig `mapstructure:"log"`
}

type LogConfig struct {
	Level string `mapstructure:"level"`
}
