package config

import (
	"fmt"
	"os"
	"path"

	"github.com/mitchellh/go-homedir"
	ms "github.com/mitchellh/mapstructure"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

const (
	AuthRBACSimplified = "simplified"
	AuthRBACExternal   = "external"
)

type Config struct {
	Path     string         `mapstructure:"config"`
	Log      LogConfig      `mapstructure:"log"`
	API      APIConfig      `mapstructure:"api"`
	Database DatabaseConfig `mapstructure:"database"`
	Auth     AuthConfig     `mapstructure:"auth"`

	Blockstore BlockStoreConfig `mapstructure:"blockstore"`
}

type LogConfig struct {
	Level string `mapstructure:"level"`
}

type APIConfig struct {
	Listen string `mapstructure:"listen"`
}

type DatabaseConfig struct {
	Connection string `mapstructure:"connection"`
	Debug      bool   `mapstructure:"debug"`
}

type AuthConfig struct {
	SecretKey string `mapstructure:"secretKey"`

	UIConfig struct {
		RBAC               string   `mapstructure:"rbac"`
		LoginURL           string   `mapstructure:"login_url"`
		LoginFailedMessage string   `mapstructure:"login_failed_message"`
		FallbackLoginURL   *string  `mapstructure:"fallback_login_url"`
		FallbackLoginLabel *string  `mapstructure:"fallback_login_label"`
		LoginCookieNames   []string `mapstructure:"login_cookie_names"`
		LogoutURL          string   `mapstructure:"logout_url"`
	} `mapstructure:"ui_config"`
}

func InitConfig(cfgFile string) error {
	var err error
	cfgFile, err = homedir.Expand(cfgFile)
	if err != nil {
		return err
	}

	_, err = os.Stat(cfgFile)
	if err == nil {
		return fmt.Errorf("config already exit in %s", cfgFile)
	}
	if err != nil && !os.IsNotExist(err) {
		return err
	}

	viper.SetConfigFile(cfgFile)

	data := make(map[string]interface{})
	err = ms.Decode(defaultCfg, &data)
	if err != nil {
		return err
	}
	for k, v := range data {
		viper.SetDefault(k, v)
	}

	basePath := path.Dir(cfgFile)
	err = os.MkdirAll(basePath, 0755)
	if err != nil {
		return err
	}
	return viper.WriteConfigAs(cfgFile)
}

// LoadConfig reads in config file and ENV variables if set.
func LoadConfig(cfgFile string) (*Config, error) {
	var err error
	cfgFile, err = homedir.Expand(cfgFile)
	if err != nil {
		return nil, err
	}
	if len(cfgFile) > 0 {
		// Use config file from the flag.
		viper.SetConfigFile(cfgFile)
	} else {
		// Find home directory.
		home, err := os.UserHomeDir()
		cobra.CheckErr(err)

		// Search config in home directory with name ".jiaozifs" (without extension).
		viper.AddConfigPath(path.Join(home, ".jiaozifs"))
		viper.SetConfigType("toml")
		viper.SetConfigName("config")
	}

	viper.AutomaticEnv() // read in environment variables that match

	// If a config file is found, read it in.
	err = viper.ReadInConfig()
	if err != nil {
		return nil, err
	}

	cfg := &Config{}
	return cfg, viper.Unmarshal(cfg)
}
