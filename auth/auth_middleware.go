package auth

import (
	"context"
	"errors"
	"net/http"
	"strings"

	"github.com/jiaozifs/jiaozifs/auth/crypt"

	"github.com/gorilla/sessions"
	"github.com/jiaozifs/jiaozifs/models"

	"github.com/getkin/kin-openapi/openapi3"
	"github.com/getkin/kin-openapi/routers"
	"github.com/getkin/kin-openapi/routers/legacy"
)

const (
	TokenSessionKeyName     = "token"
	InternalAuthSessionName = "internal_auth_session"
	IDTokenClaimsSessionKey = "id_token_claims"
)

var (
	ErrFailedToAccessStorage = errors.New("failed to access storage")
	ErrAuthenticatingRequest = errors.New("error authenticating request")
	ErrInvalidAPIEndpoint    = errors.New("invalid API endpoint")
	ErrRequestSizeExceeded   = errors.New("request size exceeded")
	ErrStorageNamespaceInUse = errors.New("storage namespace already in use")
)

// extractSecurityRequirements using Swagger returns an array of security requirements set for the request.
func extractSecurityRequirements(router routers.Router, r *http.Request) (openapi3.SecurityRequirements, error) {
	// Find route
	route, _, err := router.FindRoute(r)
	if err != nil {
		return nil, err
	}
	if route.Operation.Security == nil {
		return route.Spec.Security, nil
	}
	return *route.Operation.Security, nil
}

type CookieAuthConfig struct {
	ValidateIDTokenClaims   map[string]string
	DefaultInitialGroups    []string
	InitialGroupsClaimName  string
	FriendlyNameClaimName   string
	ExternalUserIDClaimName string
	AuthSource              string
}

func Middleware(swagger *openapi3.T, authenticator Authenticator, secretStore crypt.SecretStore, userRepo models.IUserRepo, sessionStore sessions.Store) func(next http.Handler) http.Handler {
	router, err := legacy.NewRouter(swagger)
	if err != nil {
		panic(err)
	}
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			// if request already authenticated
			if _, userNotFoundErr := GetOperator(r.Context()); userNotFoundErr == nil {
				next.ServeHTTP(w, r)
				return
			}

			securityRequirements, err := extractSecurityRequirements(router, r)
			if err != nil {
				w.WriteHeader(http.StatusBadRequest)
				_, _ = w.Write([]byte(err.Error()))
				return
			}
			user, err := checkSecurityRequirements(r, securityRequirements, authenticator, sessionStore, secretStore, userRepo)
			if err != nil {
				w.WriteHeader(http.StatusUnauthorized)
				_, _ = w.Write([]byte(err.Error()))
				return
			}
			if user != nil {
				r = r.WithContext(WithOperator(r.Context(), user))
			}
			next.ServeHTTP(w, r)
		})
	}
}

// checkSecurityRequirements goes over the security requirements and check the authentication. returns the user information and error if the security check was required.
// it will return nil user and error in case of no security checks to match.
func checkSecurityRequirements(r *http.Request,
	securityRequirements openapi3.SecurityRequirements,
	authenticator Authenticator,
	sessionStore sessions.Store,
	secretStore crypt.SecretStore,
	userRepo models.IUserRepo,
) (*models.User, error) {
	ctx := r.Context()
	var user *models.User
	var err error

	for _, securityRequirement := range securityRequirements {
		for provider := range securityRequirement {
			switch provider {
			case "jwt_token":
				// validate jwt token from header
				authHeaderValue := r.Header.Get("Authorization")
				if authHeaderValue == "" {
					continue
				}
				parts := strings.Fields(authHeaderValue)
				if len(parts) != 2 || !strings.EqualFold(parts[0], "Bearer") {
					continue
				}
				token := parts[1]
				user, err = userByToken(ctx, secretStore, userRepo, token)
			case "basic_auth":
				// validate using basic auth
				accessKey, secretKey, ok := r.BasicAuth()
				if !ok {
					continue
				}
				user, err = userByAuth(ctx, authenticator, userRepo, accessKey, secretKey)
			case "cookie_auth":
				var internalAuthSession *sessions.Session
				internalAuthSession, _ = sessionStore.Get(r, InternalAuthSessionName)
				token := ""
				if internalAuthSession != nil {
					token, _ = internalAuthSession.Values[TokenSessionKeyName].(string)
				}
				if token == "" {
					continue
				}
				user, err = userByToken(ctx, secretStore, userRepo, token)
			default:
				// unknown security requirement to check
				log.With("provider", provider).Error("Authentication middleware unknown security requirement provider")
				return nil, ErrAuthenticatingRequest
			}

			if err != nil {
				return nil, err
			}
			if user != nil {
				return user, nil
			}
		}
	}
	return nil, nil
}

func userByToken(ctx context.Context, secretStore crypt.SecretStore, userRepo models.IUserRepo, tokenString string) (*models.User, error) {
	claims, err := VerifyToken(secretStore.SharedSecret(), tokenString)
	// make sure no audience is set for login token
	if err != nil || !claims.VerifyAudience(LoginAudience, false) {
		return nil, ErrAuthenticatingRequest
	}

	username := claims.Subject
	userData, err := userRepo.Get(ctx, models.NewGetUserParams().SetName(username))
	if err != nil {
		log.With(
			"token_id", claims.Id,
			"username", username,
			"subject", claims.Subject,
		).Debugf("could not find user id by credentials %v", err)
		return nil, ErrAuthenticatingRequest
	}
	return userData, nil
}

func userByAuth(ctx context.Context, authenticator Authenticator, userRepo models.IUserRepo, accessKey string, secretKey string) (*models.User, error) {
	username, err := authenticator.AuthenticateUser(ctx, accessKey, secretKey)
	if err != nil {
		log.With("user", accessKey).Errorf("authenticate %v", err)
		return nil, ErrAuthenticatingRequest
	}
	user, err := userRepo.Get(ctx, models.NewGetUserParams().SetName(username))
	if err != nil {
		log.With("user_name", username).Debugf("could not find user id by credentials %s", err)
		return nil, ErrAuthenticatingRequest
	}
	return user, nil
}
