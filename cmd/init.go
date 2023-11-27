/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/spf13/cobra"
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "init jiaozifs ",
	Long:  `create default config file for jiaoozifs`,
	RunE: func(cmd *cobra.Command, args []string) error {
		return config.InitConfig()
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}
