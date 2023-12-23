package apiimpl

import (
	"context"
	"errors"
	"net"
	"net/http"
	"net/url"

	"github.com/MadAppGang/httplog"
	"github.com/flowchartsman/swaggerui"
	"github.com/getkin/kin-openapi/openapi3filter"
	"github.com/go-chi/chi/v5"
	"github.com/gorilla/sessions"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/auth/crypt"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	middleware "github.com/oapi-codegen/nethttp-middleware"
	"github.com/rs/cors"
	"go.uber.org/fx"
)

var log = logging.Logger("rpc")

const APIV1Prefix = "/api/v1"

func SetupAPI(lc fx.Lifecycle, apiConfig *config.APIConfig, secretStore crypt.SecretStore, sessionStore sessions.Store, repo models.IRepo, controller APIController) error {
	swagger, err := api.GetSwagger()
	if err != nil {
		return err
	}

	// This is how you set up a basic chi router
	r := chi.NewRouter()
	r.Use(httplog.LoggerWithName("http"),
		cors.New(cors.Options{
			AllowedOrigins: []string{"*"},
			AllowedMethods: []string{
				http.MethodHead,
				http.MethodGet,
				http.MethodPost,
				http.MethodPut,
				http.MethodPatch,
				http.MethodDelete,
			},
			AllowedHeaders:   []string{"*"},
			AllowCredentials: true,
		}).Handler,
	)
	// Use our validation middleware to check all requests against the
	// OpenAPI schema.
	apiRouter := r.With(
		middleware.OapiRequestValidatorWithOptions(swagger, &middleware.Options{
			Options: openapi3filter.Options{
				AuthenticationFunc: openapi3filter.NoopAuthenticationFunc,
			},
			SilenceServersWarning: true,
		}),
		auth.Middleware(swagger, nil, secretStore, repo.UserRepo(), sessionStore),
	)

	raw, err := api.RawSpec()
	if err != nil {
		return err
	}

	api.HandlerFromMuxWithBaseURL(controller, apiRouter, APIV1Prefix)
	r.Handle("/api/docs/*", http.StripPrefix("/api/docs", swaggerui.Handler(raw)))

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
