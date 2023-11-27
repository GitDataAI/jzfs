/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"context"
	"errors"
	"github.com/go-chi/chi/v5"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/api/api_impl"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/fx_opt"
	"github.com/jiaozifs/jiaozifs/utils"
	middleware "github.com/oapi-codegen/nethttp-middleware"
	"github.com/spf13/cobra"
	"go.uber.org/fx"
	"net"
	"net/http"
	"net/url"
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
			//business
			//api
			fx_opt.Override(fx_opt.NextInvoke(), setupAPI),
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

func setupAPI(lc fx.Lifecycle, apiConfig *config.APIConfig, controller api_impl.APIController) error {
	swagger, err := api.GetSwagger()
	if err != nil {
		return err
	}

	// Clear out the servers array in the swagger spec, that skips validating
	// that server names match. We don't know how this thing will be run.
	swagger.Servers = nil
	// This is how you set up a basic chi router
	r := chi.NewRouter()

	// Use our validation middleware to check all requests against the
	// OpenAPI schema.
	r.Use(middleware.OapiRequestValidator(swagger))

	api.HandlerFromMux(controller, r)

	url, err := url.Parse(apiConfig.Listen)
	if err != nil {
		return err
	}

	listener, err := net.Listen("tcp", url.Host)
	if err != nil {
		return err
	}
	log.Infof("Start listen api %s", listener.Addr())
	go func() {
		err := http.Serve(listener, r)
		if err != nil && !errors.Is(err, http.ErrServerClosed) {
			log.Errorf("listen address fail %s", err)
		}
	}()

	lc.Append(fx.Hook{
		OnStop: func(ctx context.Context) error {
			return listener.Close()
		},
	})
	return nil
}
