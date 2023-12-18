package cmd

import (
	"os"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var cfgFile string

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "jiaozifs",
	Short: "version file for manage datasets",
	Long:  ``,
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func RootCmd() *cobra.Command {
	return rootCmd
}
func init() {
	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.jiaozifs/config.yaml)")
	rootCmd.PersistentFlags().String("listen", config.DefaultLocalBSPath, "config blockstore path")
	_ = viper.BindPFlag("api.listen", rootCmd.PersistentFlags().Lookup("listen"))
	_ = viper.BindPFlag("config", rootCmd.PersistentFlags().Lookup("config"))
}
