package cmd

import (
	"context"

	"github.com/jiaozifs/jiaozifs/block/params"

	"github.com/jiaozifs/jiaozifs/block"
	"github.com/jiaozifs/jiaozifs/block/factory"

	logging "github.com/ipfs/go-log/v2"
	apiImpl "github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/fx_opt"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/migrations"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
	"github.com/uptrace/bun"
)

var log = logging.Logger("main")

// daemonCmd represents the daemon command
var daemonCmd = &cobra.Command{
	Use:   "daemon",
	Short: "daemon program of jiaozifs",
	Long:  ``,
	RunE: func(cmd *cobra.Command, args []string) error {
		cfg, err := config.LoadConfig(cfgFile)
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
			fx_opt.Override(new(*config.APIConfig), &cfg.API),
			fx_opt.Override(new(*config.DatabaseConfig), &cfg.Database),
			fx_opt.Override(new(*config.AuthConfig), &cfg.Auth),
			fx_opt.Override(new(params.AdapterConfig), &cfg.Blockstore),
			//blockstore
			fx_opt.Override(new(block.Adapter), factory.BuildBlockAdapter),
			//database
			fx_opt.Override(new(*bun.DB), models.SetupDatabase),
			fx_opt.Override(fx_opt.NextInvoke(), migrations.MigrateDatabase),
			fx_opt.Override(new(*models.IUserRepo), models.NewUserRepo),
			//api
			fx_opt.Override(fx_opt.NextInvoke(), apiImpl.SetupAPI),
		)
		if err != nil {
			return err
		}

		go utils.CatchSig(cmd.Context(), shutdown)

		<-shutdown
		log.Info("graceful shutdown")
		return stop(cmd.Context())
	},
}

func init() {
	rootCmd.AddCommand(daemonCmd)
	daemonCmd.Flags().String("db", "", "pg connection string eg. postgres://user:pass@localhost:5432/jiaozifs?sslmode=disable")
	daemonCmd.Flags().String("log-level", "INFO", "set log level eg. DEBUG INFO ERROR")

	_ = viper.BindPFlag("database.connection", daemonCmd.Flags().Lookup("db"))
	_ = viper.BindPFlag("log.level", daemonCmd.Flags().Lookup("log-level"))
}
