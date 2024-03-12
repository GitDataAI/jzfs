package auth

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"strings"

	logging "github.com/ipfs/go-log/v2"

	"github.com/GitDataAI/jiaozifs/utils"

	"github.com/GitDataAI/jiaozifs/auth/aksk"

	"github.com/golang-jwt/jwt/v5"

	"github.com/GitDataAI/jiaozifs/auth/crypt"
	"github.com/GitDataAI/jiaozifs/models"
	"github.com/getkin/kin-openapi/openapi3"
	"github.com/getkin/kin-openapi/routers"
	"github.com/getkin/kin-openapi/routers/legacy"
	"github.com/gorilla/sessions"
)

const (
	TokenSessionKeyName     = "token"
	InternalAuthSessionName = "internal_auth_session"
	IDTokenClaimsSessionKey = "id_token_claims"
)

var log = logging.Logger("auth")
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

func Middleware(swagger *openapi3.T,
	authenticator *BasicAuthenticator,
	secretStore crypt.SecretStore,
	userRepo models.IUserRepo,
	akskRepo models.IAkskRepo,
	sessionStore sessions.Store,
	verifier aksk.Verifier,
) func(next http.Handler) http.Handler {
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
			user, err := checkSecurityRequirements(r, securityRequirements, authenticator, sessionStore, secretStore, verifier, userRepo, akskRepo)
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
	authenticator *BasicAuthenticator,
	sessionStore sessions.Store,
	secretStore crypt.SecretStore,
	verifier aksk.Verifier,
	userRepo models.IUserRepo,
	akskRepo models.IAkskRepo,
) (*models.User, error) {
	ctx := r.Context()
	var user *models.User
	var err error

	for _, securityRequirement := range securityRequirements {
		securityKeys := getSecurityKey(securityRequirement)
		if utils.Contain(securityKeys, "jwt_token") {
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
			user, err = userByToken(ctx, userRepo, secretStore.SharedSecret(), token)
		} else if utils.Contain(securityKeys, "basic_auth") {
			// validate using basic auth
			userName, password, ok := r.BasicAuth()
			if !ok {
				continue
			}

			user, err = userByAuth(ctx, authenticator, userName, password)
		} else if utils.Contain(securityKeys, "cookie_auth") {
			var internalAuthSession *sessions.Session
			internalAuthSession, _ = sessionStore.Get(r, InternalAuthSessionName)
			token := ""
			if internalAuthSession != nil {
				token, _ = internalAuthSession.Values[TokenSessionKeyName].(string)
			}
			if token == "" {
				continue
			}
			user, err = userByToken(ctx, userRepo, secretStore.SharedSecret(), token)
		} else if utils.Contain(securityKeys, aksk.AccessKeykey) {
			isAkskRequest := verifier.IsAkskCredential(r)
			if !isAkskRequest {
				continue
			}
			user, err = userByAKSK(ctx, akskRepo, userRepo, verifier, r)
		} else {
			// unknown security requirement to check
			log.With("provider", securityKeys).Error("Authentication middleware unknown security requirement provider")
			return nil, ErrAuthenticatingRequest
		}

		if err != nil {
			return nil, err
		}
		if user != nil {
			return user, nil
		}
	}
	return nil, nil
}

func userByAKSK(ctx context.Context, akskRepo models.IAkskRepo, userRepo models.IUserRepo, verifier aksk.Verifier, r *http.Request) (*models.User, error) {
	ak, err := verifier.Verify(r)
	if err != nil {
		return nil, err
	}

	akModel, err := akskRepo.Get(ctx, models.NewGetAkSkParams().SetAccessKey(ak))
	if err != nil {
		return nil, err
	}

	userModel, err := userRepo.Get(ctx, models.NewGetUserParams().SetID(akModel.UserID))
	if err != nil {
		return nil, err
	}
	return userModel, nil
}

func userByToken(ctx context.Context, userRepo models.IUserRepo, secret []byte, tokenString string) (*models.User, error) {
	claims, err := VerifyToken(secret, tokenString)
	if err != nil {
		return nil, ErrAuthenticatingRequest
	}

	// make sure no audience is set for login token
	validator := jwt.NewValidator(jwt.WithAudience(LoginAudience))
	if err = validator.Validate(claims); err != nil {
		return nil, fmt.Errorf("invalid token: %s %w", err, ErrAuthenticatingRequest)
	}

	username, err := claims.GetSubject()
	if err != nil {
		return nil, err
	}
	userData, err := userRepo.Get(ctx, models.NewGetUserParams().SetName(username))
	if err != nil {
		log.With(
			"token", tokenString,
			"username", username,
			"subject", username,
		).Debugf("could not find user id by credentials %v", err)
		return nil, ErrAuthenticatingRequest
	}
	return userData, nil
}

func userByAuth(ctx context.Context, authenticator *BasicAuthenticator, accessKey string, secretKey string) (*models.User, error) {
	user, err := authenticator.AuthenticateUser(ctx, accessKey, secretKey)
	if err != nil {
		log.With("user", accessKey).Errorf("authenticate %v", err)
		return nil, ErrAuthenticatingRequest
	}
	return user, nil
}

func getSecurityKey(security openapi3.SecurityRequirement) []string {
	var keys []string
	for key := range security {
		keys = append(keys, key)
	}
	return keys
}
