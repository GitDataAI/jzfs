package auth

import (
	"context"
	"errors"
	"time"

	"github.com/golang-jwt/jwt"
	openapi_types "github.com/oapi-codegen/runtime/types"

	"github.com/go-openapi/swag"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/models"
	"golang.org/x/crypto/bcrypt"
)

var log = logging.Logger("auth")

type Login struct {
	Username string `json:"username"`
	Password string `json:"password"`
}

func (l *Login) Login(ctx context.Context, authService Service) (token api.AuthenticationToken, err error) {
	// Get user encryptedPassword by username
	ep, err := authService.GetEPByName(ctx, l.Username)
	if err != nil {
		log.Errorf("username err: %s", err)
		return token, err
	}

	// Compare ep and password
	err = bcrypt.CompareHashAndPassword([]byte(ep), []byte(l.Password))
	if err != nil {
		log.Errorf("password err: %s", err)
		return token, err
	}
	// Generate user token
	loginTime := time.Now()
	expires := loginTime.Add(expirationDuration)
	secretKey := authService.GetSecretKey()

	tokenString, err := GenerateJWTLogin(secretKey, l.Username, loginTime, expires)
	if err != nil {
		log.Errorf("generate token err: %s", err)
		return token, err
	}

	token.Token = tokenString
	token.TokenExpiration = swag.Int64(expires.Unix())

	return token, nil
}

type Register struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

func (r *Register) Register(ctx context.Context, authService Service) (msg api.RegistrationMsg, err error) {
	// check username, email
	if authService.CheckUserByNameEmail(ctx, r.Username, r.Email) {
		msg.Message = "The username or email has already been registered"
		return
	}

	password, err := bcrypt.GenerateFromPassword([]byte(r.Password), passwordCost)
	if err != nil {
		msg.Message = "Generate Password err"
		return
	}

	// insert db
	user := &models.User{
		Name:              r.Username,
		Email:             r.Email,
		EncryptedPassword: string(password),
		CurrentSignInAt:   time.Time{},
		LastSignInAt:      time.Time{},
		CurrentSignInIP:   "",
		LastSignInIP:      "",
		CreatedAt:         time.Now(),
		UpdatedAt:         time.Time{},
	}
	insertUser, err := authService.Insert(ctx, user)
	if err != nil {
		msg.Message = "register user err"
		return
	}
	// return
	msg.Message = insertUser.Name + " register success"
	return msg, nil
}

type UserInfo struct {
	Token string `json:"token"`
}

func (u *UserInfo) UserProfile(ctx context.Context, authService Service) (api.UserInfo, error) {
	userInfo := api.UserInfo{}
	// Parse JWT Token
	token, err := jwt.Parse(u.Token, func(token *jwt.Token) (interface{}, error) {
		return authService.GetSecretKey(), nil
	})
	if err != nil {
		return userInfo, err
	}
	// Check Token validity
	if !token.Valid {
		return userInfo, errors.New("token is invalid")
	}
	// Get username by token
	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return userInfo, errors.New("failed to extract claims from JWT token")
	}
	username := claims["sub"].(string)

	// Get user by username
	user, err := authService.GetUserByName(ctx, username)
	if err != nil {
		return userInfo, err
	}
	userInfo = api.UserInfo{
		CreatedAt:       &user.CreatedAt,
		CurrentSignInAt: &user.CurrentSignInAt,
		CurrentSignInIP: &user.CurrentSignInIP,
		Email:           openapi_types.Email(user.Email),
		LastSignInAt:    &user.LastSignInAt,
		LastSignInIP:    &user.LastSignInIP,
		UpdateAt:        &user.UpdatedAt,
		Username:        user.Name,
	}
	return userInfo, nil
}
