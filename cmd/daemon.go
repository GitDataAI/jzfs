package cmd

import (
	"context"

	"github.com/GitDataAI/jiaozifs/auth/rbac"

	"github.com/GitDataAI/jiaozifs/auth/aksk"

	"github.com/pelletier/go-toml/v2"

	apiImpl "github.com/GitDataAI/jiaozifs/api/api_impl"
	"github.com/GitDataAI/jiaozifs/auth"
	"github.com/GitDataAI/jiaozifs/auth/crypt"
	"github.com/GitDataAI/jiaozifs/block/params"
	"github.com/GitDataAI/jiaozifs/config"
	"github.com/GitDataAI/jiaozifs/fx_opt"
	"github.com/GitDataAI/jiaozifs/models"
	"github.com/GitDataAI/jiaozifs/models/migrations"
	"github.com/GitDataAI/jiaozifs/utils"
	"github.com/GitDataAI/jiaozifs/version"
	"github.com/gorilla/sessions"
	logging "github.com/ipfs/go-log/v2"
	"github.com/spf13/cobra"
	"github.com/uptrace/bun"
)

var log = logging.Logger("main")

// daemonCmd represents the daemon command
var daemonCmd = &cobra.Command{
	Use:   "daemon",
	Short: "daemon program of jiaozifs",
	Long:  ``,
	RunE: func(cmd *cobra.Command, _ []string) error {
		cfg, err := config.LoadConfig(cfgFile)
		if err != nil {
			return err
		}

		err = logging.SetLogLevel("*", cfg.Log.Level)
		if err != nil {
			return err
		}

		cfgData, err := toml.Marshal(cfg)
		if err != nil {
			return err
		}
		log.Debug(string(cfgData))

		shutdown := make(utils.Shutdown)
		stop, err := fx_opt.New(cmd.Context(),
			fx_opt.Override(new(context.Context), cmd.Context()),
			fx_opt.Override(new(utils.Shutdown), shutdown),
			//version
			fx_opt.Override(new(version.IChecker), version.NewVersionChecker),
			//config
			fx_opt.Override(new(*config.Config), cfg),
			fx_opt.Override(new(*config.APIConfig), &cfg.API),
			fx_opt.Override(new(*config.AuthConfig), &cfg.Auth),
			fx_opt.Override(new(*config.DatabaseConfig), &cfg.Database),
			fx_opt.Override(new(params.AdapterConfig), &cfg.Blockstore),
			//database
			fx_opt.Override(new(*bun.DB), models.SetupDatabase),
			fx_opt.Override(new(models.IRepo), func(db *bun.DB) models.IRepo {
				return models.NewRepo(db)
			}),
			fx_opt.Override(new(models.IUserRepo), func(repo models.IRepo) models.IUserRepo {
				return repo.UserRepo()
			}),

			fx_opt.Override(fx_opt.NextInvoke(), migrations.MigrateDatabase),
			//permission
			fx_opt.Override(new(rbac.PermissionCheck), func(repo models.IRepo) rbac.PermissionCheck {
				return rbac.NewRbacAuth(repo)
			}),

			//api
			fx_opt.Override(new(crypt.SecretStore), auth.NewSectetStore),
			fx_opt.Override(new(sessions.Store), auth.NewSessionStore),
			fx_opt.Override(new(*auth.BasicAuthenticator), auth.NewBasicAuthenticator),
			fx_opt.Override(new(aksk.Verifier), auth.NewAkskVerifier),
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
}
