/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/version"
	"github.com/spf13/cobra"
)

// versionCmd represents the version command
var versionCmd = &cobra.Command{
	Use:   "version",
	Short: "version of jiaozifs",
	Long:  `jiaozifs version`,
	RunE: func(cmd *cobra.Command, args []string) error {
		cfg, err := config.LoadConfig(cfgFile)
		if err != nil {
			return err
		}
		client, err := api.NewClient(cfg.API.Listen)
		if err != nil {
			return err
		}

		swagger, err := api.GetSwagger()
		if err != nil {
			return err
		}

		versionResp, err := client.GetVersion(cmd.Context())
		if err != nil {
			return err
		}
		okResp, err := api.ParseGetVersionResponse(versionResp)
		if err != nil {
			return err
		}
		if okResp.JSON200 == nil {
			return fmt.Errorf("request version fail %d %s", okResp.HTTPResponse.StatusCode, okResp.HTTPResponse.Body)
		}
		fmt.Println("Version ", version.UserVersion())
		fmt.Println("API Version ", swagger.Info.Version)
		fmt.Println("Runtime Version ", okResp.JSON200.Version)
		fmt.Println("Runtime API Version ", okResp.JSON200.ApiVersion)
		return nil
	},
}

func init() {
	rootCmd.AddCommand(versionCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// versionCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// versionCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
