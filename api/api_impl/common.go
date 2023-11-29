package apiimpl

import (
	"encoding/json"
	"fmt"
	"github.com/go-openapi/swag"
	"github.com/gorilla/sessions"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"golang.org/x/crypto/bcrypt"
	"net/http"
	"time"

	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/version"
	"go.uber.org/fx"
)

const (
	expirationDuration      = time.Hour
	passwordCost            = 12
	TokenSessionKeyName     = "token"
	InternalAuthSessionName = "internal_auth_session"
)

var _ api.ServerInterface = (*APIController)(nil)

type APIController struct {
	fx.In

	UserRepo *models.IUserRepo
	Config   *config.Config

	sessionStore sessions.Store
}

// Login User login
func (A APIController) Login(w *api.JiaozifsResponse, r *http.Request, params api.LoginParams) {
	ctx := r.Context()
	// Encrypt password
	encryptPassword, _ := bcrypt.GenerateFromPassword([]byte(params.SecretAccessKey), passwordCost)
	// Get user by SA, EP
	user, err := auth.UserByAuth(ctx, *A.UserRepo, params.AccessKeyId, string(encryptPassword))
	if err != nil {
		writeResponse(w, r, http.StatusUnauthorized, http.StatusText(http.StatusUnauthorized))
		return
	}

	// Generate user token
	loginTime := time.Now()
	expires := loginTime.Add(expirationDuration)
	secret := A.Config.Auth.SecretKey

	tokenString, err := utils.GenerateJWTLogin(secret, user.Name, loginTime, expires)
	if err != nil {
		writeError(w, r, http.StatusInternalServerError, http.StatusText(http.StatusInternalServerError))
		return
	}

	// TODO Session
	//internalAuthSession, _ := A.sessionStore.Get(r, InternalAuthSessionName)
	//internalAuthSession.Values[TokenSessionKeyName] = tokenString
	//err = A.sessionStore.Save(r, w, internalAuthSession)
	//if err != nil {
	//	log.Errorf("Failed to save internal auth session")
	//	writeError(w, r, http.StatusInternalServerError, http.StatusText(http.StatusInternalServerError))
	//	return
	//}

	// Response Client
	resp := api.AuthenticationToken{
		Token:           tokenString,
		TokenExpiration: swag.Int64(expires.Unix()),
	}
	writeResponse(w, r, http.StatusOK, resp)
}

func (A APIController) GetVersion(w *api.JiaozifsResponse, _ *http.Request) {
	swagger, err := api.GetSwagger()
	if err != nil {
		w.RespError(err)
		return
	}

	w.RespJSON(api.VersionResult{
		ApiVersion: swagger.Info.Version,
		Version:    version.UserVersion(),
	})
}

func writeError(w http.ResponseWriter, r *http.Request, code int, v interface{}) {
	apiErr := api.Error{
		Message: fmt.Sprint(v),
	}
	writeResponse(w, r, code, apiErr)
}

func writeResponse(w http.ResponseWriter, r *http.Request, code int, response interface{}) {
	// check first if the client canceled the request
	//if httputil.IsRequestCanceled(r) {
	//	w.WriteHeader(httpStatusClientClosedRequest) // Client closed request
	//	return
	//}
	// nobody - just status code
	if response == nil {
		w.WriteHeader(code)
		return
	}
	// encode response body as json
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("X-Content-Type-Options", "nosniff")
	w.WriteHeader(code)
	err := json.NewEncoder(w).Encode(response)
	if err != nil {
		log.Errorf("code %d, Failed to write encoded json response", code)
	}
}
