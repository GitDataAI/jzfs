package cmd

import (
	"os"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/mitchellh/go-homedir"
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
		return viper.BindPFlag("database.connection", cmd.Flags().Lookup("db"))
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		err := config.InitConfig()
		if err != nil {
			return err
		}
		//create a blockstore in home path for default usage
		defaultBsPath, err := homedir.Expand(config.DefaultLocalBSPath)
		if err != nil {
			return err
		}
		return os.MkdirAll(defaultBsPath, 0755)
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
	initCmd.Flags().String("db", "", "pg connection string eg. postgres://user:pass@localhost:5432/jiaozifs?sslmode=disable")
}
