/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"context"

	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/fx_opt"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var log = logging.Logger("main")

// daemonCmd represents the daemon command
var daemonCmd = &cobra.Command{
	Use:   "daemon",
	Short: "daemon program of jiaozifs",
	Long:  ``,
	RunE: func(cmd *cobra.Command, args []string) error {
		cfg := config.Config{}
		err := viper.Unmarshal(&cfg)
		if err != nil {
			return err
		}

		err = logging.SetLogLevel("*", cfg.Log.Level)
		if err != nil {
			return err
		}

		shutdown := make(utils.Shutdown)
		stop, err := fx_opt.New(cmd.Context(),
			fx_opt.Override(new(context.Context), cmd.Context()),
			fx_opt.Override(new(utils.Shutdown), shutdown),
			//config
			fx_opt.Override(new(*config.Config), cfg),
			//business

			//api

		)
		if err != nil {
			return err
		}

		go utils.CatchSig(cmd.Context(), shutdown)

		<-shutdown
		log.Info("graceful shutdown")
		return stop(cmd.Context())
		return nil
	},
}

func init() {
	rootCmd.AddCommand(daemonCmd)
}
