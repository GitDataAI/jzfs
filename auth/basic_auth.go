package auth

import (
	"context"
	"fmt"
	"time"

	"github.com/jiaozifs/jiaozifs/config"

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

func (l *Login) Login(ctx context.Context, repo models.IUserRepo, config *config.AuthConfig) (token api.AuthenticationToken, err error) {
	// get user encryptedPassword by username
	ep, err := repo.GetEPByName(ctx, l.Username)
	if err != nil {
		return token, fmt.Errorf("cannt get user %s encrypt password %w", l.Username, err)
	}

	// Compare ep and password
	err = bcrypt.CompareHashAndPassword([]byte(ep), []byte(l.Password))
	if err != nil {
		log.Errorf("password err: %s", err)
		return token, fmt.Errorf("user %s password not match %w", l.Username, err)
	}
	// Generate user token
	loginTime := time.Now()
	expires := loginTime.Add(expirationDuration)
	secretKey := config.SecretKey

	tokenString, err := GenerateJWTLogin(secretKey, l.Username, loginTime, expires)
	if err != nil {
		return token, fmt.Errorf("generate token err: %w", err)
	}

	log.Infof("usert %s login successful", l.Username)

	token.Token = tokenString
	token.TokenExpiration = swag.Int64(expires.Unix())
	return token, nil
}

type Register struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

func (r *Register) Register(ctx context.Context, repo models.IUserRepo) error {
	// check username, email
	count1, err := repo.Count(ctx, models.NewCountUserParam().SetName(r.Username))
	if err != nil {
		return err
	}
	count2, err := repo.Count(ctx, models.NewCountUserParam().SetName(r.Email))
	if err != nil {
		return err
	}

	if count1+count2 > 0 {
		return fmt.Errorf("username %s or email %s not found %w ", r.Username, r.Email, ErrInvalidNameEmail)
	}

	// reserve temporarily
	password, err := bcrypt.GenerateFromPassword([]byte(r.Password), passwordCost)
	if err != nil {
		return fmt.Errorf("invalid password %w", err)
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
		UpdatedAt:         time.Now(),
	}
	insertUser, err := repo.Insert(ctx, user)
	if err != nil {
		return fmt.Errorf("inser user %s user error %w", r.Username, err)
	}

	log.Infof("%s registration success", insertUser.Name)
	return nil
}
