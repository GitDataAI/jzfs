package auth

import (
	"context"
	"time"

	"github.com/jiaozifs/jiaozifs/config"

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

func (l *Login) Login(ctx context.Context, repo models.IUserRepo, config *config.Config) (token api.AuthenticationToken, err error) {
	// Get user encryptedPassword by username
	ep, err := repo.GetEPByName(ctx, l.Username)
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
	secretKey := config.Auth.SecretKey

	tokenString, err := GenerateJWTLogin(secretKey, l.Username, loginTime, expires)
	if err != nil {
		log.Errorf("generate token err: %s", err)
		return token, err
	}

	log.Info("login successful")

	token.Token = tokenString
	token.TokenExpiration = swag.Int64(expires.Unix())

	return token, nil
}

type Register struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

func (r *Register) Register(ctx context.Context, repo models.IUserRepo) (err error) {
	// check username, email
	_, err1 := repo.GetUserByName(ctx, r.Username)
	_, err2 := repo.GetUserByEmail(ctx, r.Email)
	if err1 == nil || err2 == nil {
		err = ErrInvalidNameEmail
		log.Error(ErrInvalidNameEmail)
		return
	}
	// reserve temporarily
	password, err := bcrypt.GenerateFromPassword([]byte(r.Password), passwordCost)
	if err != nil {
		log.Error(ErrComparePassword)
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
	insertUser, err := repo.Insert(ctx, user)
	if err != nil {
		log.Error("create user error")
		return
	}
	// return
	log.Infof("%s registration success", insertUser.Name)
	return nil
}

type UserInfo struct {
	Token string `json:"token"`
}

func (u *UserInfo) UserProfile(ctx context.Context, repo models.IUserRepo, config *config.Config) (api.UserInfo, error) {
	userInfo := api.UserInfo{}
	// Parse JWT Token
	token, err := jwt.Parse(u.Token, func(token *jwt.Token) (interface{}, error) {
		return config.Auth.SecretKey, nil
	})
	if err != nil {
		log.Error(ErrParseToken)
		return userInfo, err
	}
	// Check Token validity
	if !token.Valid {
		log.Error(ErrInvalidToken)
		return userInfo, ErrInvalidToken
	}
	// Get username by token
	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		log.Error(ErrExtractClaims)
		return userInfo, ErrExtractClaims
	}
	username := claims["sub"].(string)

	// Get user by username
	user, err := repo.GetUserByName(ctx, username)
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
	log.Info("get user profile success")
	return userInfo, nil
}
