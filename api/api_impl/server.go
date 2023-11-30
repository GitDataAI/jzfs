package apiimpl

import (
	"context"
	"errors"
	"net"
	"net/http"

	"net/url"

	"github.com/getkin/kin-openapi/openapi3filter"
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/cors"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/config"
	middleware "github.com/oapi-codegen/nethttp-middleware"
	"go.uber.org/fx"
)

var log = logging.Logger("rpc")

const APIV1Prefix = "/api/v1"

func SetupAPI(lc fx.Lifecycle, apiConfig *config.APIConfig, authCfg *config.AuthConfig, controller APIController) error {
	swagger, err := api.GetSwagger()
	if err != nil {
		return err
	}

	//sessionStore := sessions.NewCookieStore([]byte(authCfg.SecretKey))

	// Clear out the servers array in the swagger spec, that skips validating
	// that server names match. We don't know how this thing will be run.
	swagger.Servers = nil
	// This is how you set up a basic chi router
	r := chi.NewRouter()

	// Use our validation middleware to check all requests against the
	// OpenAPI schema.
	r.Use(
		cors.Handler(cors.Options{
			// Basic CORS
			// for more ideas, see: https://developer.github.com/v3/#cross-origin-resource-sharing

			// AllowedOrigins:   []string{"https://foo.com"}, // Use this to allow specific origin hosts
			AllowedOrigins: []string{"https://*", "http://*"},
			// AllowOriginFunc:  func(r *http.Request, origin string) bool { return true },
			AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
			AllowedHeaders:   []string{"*"},
			ExposedHeaders:   []string{"*"},
			AllowCredentials: false,
			MaxAge:           300, // Maximum value not ignored by any of major browsers
		}),

		middleware.OapiRequestValidatorWithOptions(swagger, &middleware.Options{
			Options: openapi3filter.Options{
				AuthenticationFunc: func(ctx context.Context, input *openapi3filter.AuthenticationInput) error {
					return nil
				},
			},
		}),
	)

	//api.HandlerFromMuxWithBaseURL(controller, r, APIV1Prefix)
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
