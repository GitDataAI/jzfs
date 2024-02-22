package apiimpl

import (
	"context"
	"errors"
	"fmt"
	"net"
	"net/http"
	"net/url"
	"strings"

	"github.com/hellofresh/health-go/v5"

	"github.com/getkin/kin-openapi/openapi3"
	"github.com/getkin/kin-openapi/routers/gorillamux"

	"github.com/MadAppGang/httplog"
	"github.com/flowchartsman/swaggerui"
	"github.com/getkin/kin-openapi/openapi3filter"
	"github.com/getkin/kin-openapi/routers"
	"github.com/go-chi/chi/v5"
	"github.com/gorilla/sessions"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/auth/crypt"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/rs/cors"
	"go.uber.org/fx"
)

var log = logging.Logger("rpc")

const (
	APIV1Prefix                    = "/api/v1"
	extensionValidationExcludeBody = "x-validation-exclude-body"
)

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
		OapiRequestValidatorWithOptions(swagger, &openapi3filter.Options{
			AuthenticationFunc: openapi3filter.NoopAuthenticationFunc,
		}),
		auth.Middleware(swagger, nil, secretStore, repo.UserRepo(), sessionStore),
	)

	raw, err := api.RawSpec()
	if err != nil {
		return err
	}

	api.HandlerFromMuxWithBaseURL(controller, apiRouter, APIV1Prefix)
	r.Handle("/api/docs/*", http.StripPrefix("/api/docs", swaggerui.Handler(raw)))
	h, _ := health.New(health.WithComponent(health.Component{
		Name:    "myservice",
		Version: "v1.0",
	}))
	r.Get("/status", h.HandlerFunc)

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

// OapiRequestValidatorWithOptions Creates middleware to validate request by swagger spec.
func OapiRequestValidatorWithOptions(swagger *openapi3.T, options *openapi3filter.Options) func(next http.Handler) http.Handler {
	router, err := gorillamux.NewRouter(swagger)
	if err != nil {
		panic(err)
	}

	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

			// validate request
			if statusCode, err := validateRequest(r, router, options); err != nil {
				http.Error(w, err.Error(), statusCode)
				return
			}

			// serve
			next.ServeHTTP(w, r)
		})
	}
}

// validateRequest is called from the middleware above and actually does the work
// of validating a request.
func validateRequest(r *http.Request, router routers.Router, options *openapi3filter.Options) (int, error) {
	// Find route
	route, pathParams, err := router.FindRoute(r)
	if err != nil {
		return http.StatusNotFound, err // We failed to find a matching route for the request.
	}

	// Validate request
	requestValidationInput := &openapi3filter.RequestValidationInput{
		Request:    r,
		PathParams: pathParams,
		Route:      route,
	}

	if options != nil {
		optCopy := *options
		requestValidationInput.Options = &optCopy
	}

	if _, ok := route.Operation.Extensions[extensionValidationExcludeBody]; ok {
		requestValidationInput.Options.ExcludeRequestBody = true
	}

	if err := openapi3filter.ValidateRequest(r.Context(), requestValidationInput); err != nil {
		me := openapi3.MultiError{}
		if errors.As(err, &me) {
			return http.StatusBadRequest, err
		}

		switch e := err.(type) {
		case *openapi3filter.RequestError:
			// We've got a bad request
			// Split up the verbose error by lines and return the first one
			// openapi errors seem to be multi-line with a decent message on the first
			errorLines := strings.Split(e.Error(), "\n")
			return http.StatusBadRequest, fmt.Errorf(errorLines[0])
		case *openapi3filter.SecurityRequirementsError:
			return http.StatusUnauthorized, err
		default:
			// This should never happen today, but if our upstream code changes,
			// we don't want to crash the server, so handle the unexpected error.
			return http.StatusInternalServerError, fmt.Errorf("error validating route: %s", err.Error())
		}
	}

	return http.StatusOK, nil
}
