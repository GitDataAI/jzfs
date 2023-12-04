package auth

import (
	"context"
	"time"

	"github.com/go-openapi/swag"
	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	"golang.org/x/crypto/bcrypt"
)

var log = logging.Logger("auth")

type Login struct {
	Username string `json:"username"`
	Password string `json:"password"`
}
type Register struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

func (l *Login) Login(userRepo models.IUserRepo, config *config.Config) (token api.AuthenticationToken, err error) {
	ctx := context.Background()
	// Get user encryptedPassword by username
	ep, err := userRepo.GetEPByName(ctx, l.Username)
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

	token.Token = tokenString
	token.TokenExpiration = swag.Int64(expires.Unix())

	return token, nil
}

func (l *Register) Register(userRepo models.IUserRepo) (msg api.RegistrationMsg, err error) {
	ctx := context.Background()
	// check username, email
	if userRepo.CheckUserByNameEmail(ctx, l.Username, l.Email) {
		msg.Message = "The username or email has already been registered"
		return
	}

	password, err := bcrypt.GenerateFromPassword([]byte(l.Password), passwordCost)
	if err != nil {
		msg.Message = "Generate Password err"
		return
	}

	// insert db
	user := &models.User{
		Name:              l.Username,
		Email:             l.Email,
		EncryptedPassword: string(password),
		CurrentSignInAt:   time.Time{},
		LastSignInAt:      time.Time{},
		CurrentSignInIP:   "",
		LastSignInIP:      "",
		CreatedAt:         time.Now(),
		UpdatedAt:         time.Time{},
	}
	insertUser, err := userRepo.Insert(ctx, user)
	if err != nil {
		msg.Message = "register user err"
		return
	}
	// return
	msg.Message = insertUser.Name + " register success"
	return msg, nil
}
