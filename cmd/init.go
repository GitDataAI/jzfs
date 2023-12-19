package cmd

import (
	"fmt"
	"os"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "init jiaozifs ",
	Long:  `create default config file for jiaoozifs`,
	PreRunE: func(cmd *cobra.Command, args []string) error {
		//protect duplicate bind flag with daemon
		err := viper.BindPFlag("blockstore.local.path", cmd.Flags().Lookup("bs_path"))
		if err != nil {
			return err
		}

		err = viper.BindPFlag("database.debug", cmd.Flags().Lookup("db_debug"))
		if err != nil {
			return err
		}

		return viper.BindPFlag("database.connection", cmd.Flags().Lookup("db"))
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		err := config.InitConfig(cfgFile)
		if err != nil {
			return err
		}

		cfg, err := config.LoadConfig(cfgFile)
		if err != nil {
			return err
		}
		fmt.Println(cfg.API.Listen)
		if cfg.Blockstore.Type == "local" {
			_, err = os.Stat(cfg.Blockstore.Local.Path)
			if os.IsNotExist(err) {
				return os.MkdirAll(cfg.Blockstore.Local.Path, 0755)
			}
		}
		return nil
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
	initCmd.Flags().String("bs_path", config.DefaultLocalBSPath, "config blockstore path")
	initCmd.Flags().Bool("db_debug", false, "enable database debug")
	initCmd.Flags().String("db", "", "pg connection string eg. postgres://user:pass@localhost:5432/jiaozifs?sslmode=disable")
}
