package config

import (
	"fmt"
	logging "github.com/ipfs/go-log/v2"
	ms "github.com/mitchellh/mapstructure"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"os"
	"path"
)

var log = logging.Logger("log")

type Config struct {
	Path string    `mapstructure:"config"`
	Log  LogConfig `mapstructure:"log"`
	API  APIConfig `mapstructure:"api"`
}

type LogConfig struct {
	Level string `mapstructure:"level"`
}

type APIConfig struct {
	Listen string `mapstructure:"listen"`
}

func InitConfig() error {
	// Find home directory.
	home, err := os.UserHomeDir()
	cobra.CheckErr(err)
	jiaoziHome := path.Join(home, ".jiaozifs")
	defaultPath := path.Join(jiaoziHome, "config.toml")

	// Search config in home directory with name ".jiaozifs" (without extension).
	viper.AddConfigPath(path.Join(home, ".jiaozifs"))
	viper.SetConfigType("toml")
	viper.SetConfigName("config")
	if len(viper.ConfigFileUsed()) == 0 {
		data := make(map[string]interface{})
		err = ms.Decode(defaultCfg, &data)
		if err != nil {
			return err
		}
		for k, v := range data {
			viper.SetDefault(k, v)
		}
		err = os.MkdirAll(jiaoziHome, 0777)
		if err != nil {
			return err
		}
		return viper.WriteConfigAs(defaultPath)
	}
	return fmt.Errorf("config already exit in %s", defaultPath)
}

// LoadConfig reads in config file and ENV variables if set.
func LoadConfig(cfgFile string) (*Config, error) {
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
	err := viper.ReadInConfig()
	if err != nil {
		return nil, err
	}

	cfg := &Config{}
	return cfg, viper.Unmarshal(cfg)
}
