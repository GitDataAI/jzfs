package controller

import (
	"context"
	"encoding/hex"
	"errors"
	"net/http"
	"regexp"
	"time"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/go-openapi/swag"
	"github.com/gorilla/sessions"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	openapitypes "github.com/oapi-codegen/runtime/types"
	"go.uber.org/fx"
	"golang.org/x/crypto/bcrypt"
)

var userCtlLog = logging.Logger("user_ctl")
var usernameRegex = regexp.MustCompile(`^[a-zA-Z0-9][a-zA-Z0-9_-]{1,28}[a-zA-Z0-9]$`)

const (
	AuthHeader = "Authorization"
)

func CheckUserName(name string) error {
	if !usernameRegex.MatchString(name) {
		return errors.New("invalid username: it must start and end with a letter or digit, can contain letters, digits, hyphens, and cannot start or end with a hyphen; the length must be between 3 and 30 characters")
	}
	return nil
}

type UserController struct {
	fx.In

	SessionStore sessions.Store
	Repo         models.IRepo
	Config       *config.AuthConfig
}

func (userCtl UserController) Login(ctx context.Context, w *api.JiaozifsResponse, r *http.Request, body api.LoginJSONRequestBody) {
	// get user encryptedPassword by username
	ep, err := userCtl.Repo.UserRepo().GetEPByName(ctx, body.Name)
	if err != nil {
		w.Code(http.StatusUnauthorized)
		return
	}

	// Compare ep and password
	err = bcrypt.CompareHashAndPassword([]byte(ep), []byte(body.Password))
	if err != nil {
		w.Code(http.StatusUnauthorized)
		return
	}
	// Generate user token
	loginTime := time.Now()
	expires := loginTime.Add(auth.ExpirationDuration)
	secretKey, err := hex.DecodeString(userCtl.Config.SecretKey)
	if err != nil {
		w.Error(err)
		return
	}

	tokenString, err := auth.GenerateJWTLogin(secretKey, body.Name, loginTime, expires)
	if err != nil {
		w.Error(err)
		return
	}

	userCtlLog.Infof("user %s login successful", body.Name)

	internalAuthSession, _ := userCtl.SessionStore.Get(r, auth.InternalAuthSessionName)
	internalAuthSession.Values[auth.TokenSessionKeyName] = tokenString
	err = userCtl.SessionStore.Save(r, w, internalAuthSession)
	if err != nil {
		userCtlLog.Errorf("Failed to save internal auth session %v", err)
		w.Code(http.StatusInternalServerError)
		return
	}
	w.JSON(api.AuthenticationToken{
		Token:           tokenString,
		TokenExpiration: swag.Int64(expires.Unix()),
	})
}

func (userCtl UserController) Register(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request, body api.RegisterJSONRequestBody) {
	err := CheckUserName(body.Name)
	if err != nil {
		w.BadRequest(err.Error())
		return
	}

	register := auth.Register{
		Username: body.Name,
		Email:    string(body.Email),
		Password: body.Password,
	}

	// perform register
	err = register.Register(ctx, userCtl.Repo.UserRepo())
	if err != nil {
		w.Error(err)
		return
	}
	w.OK()
}

func (userCtl UserController) GetUserInfo(ctx context.Context, w *api.JiaozifsResponse, _ *http.Request) {
	// Get token from Header
	user, err := auth.GetOperator(ctx)
	if err != nil {
		w.Code(http.StatusForbidden)
		return
	}

	// perform GetUserInfo
	userInfo := api.UserInfo{
		Name:            user.Name,
		Email:           openapitypes.Email(user.Email),
		CurrentSignInAt: utils.Int64(user.CurrentSignInAt.UnixMilli()),
		CurrentSignInIp: &user.CurrentSignInIP,
		LastSignInAt:    utils.Int64(user.LastSignInAt.UnixMilli()),
		LastSignInIp:    &user.LastSignInIP,
		UpdatedAt:       user.UpdatedAt.UnixMilli(),
		CreatedAt:       user.CreatedAt.UnixMilli(),
	}
	w.JSON(userInfo)
}

func (userCtl UserController) Logout(_ context.Context, w *api.JiaozifsResponse, r *http.Request) {
	session, err := userCtl.SessionStore.Get(r, auth.InternalAuthSessionName)
	if err != nil {
		w.Error(err)
		return
	}
	session.Options.MaxAge = -1
	if session.Save(r, w) != nil {
		userCtlLog.Errorf("Failed to save internal auth session %v", err)
		w.Error(err)
		return
	}
	http.Redirect(w, r, "/", http.StatusTemporaryRedirect)
}
