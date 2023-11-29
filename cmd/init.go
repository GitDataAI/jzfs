package cmd

import (
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
		return viper.BindPFlag("database.connection", cmd.Flags().Lookup("db"))
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		return config.InitConfig()
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
	initCmd.Flags().String("db", "", "pg connection string eg. postgres://user:pass@localhost:5432/jiaozifs?sslmode=disable")
}
