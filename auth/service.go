package auth

import (
	"context"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
)

type Service interface {
	// user
	Insert(ctx context.Context, user *models.User) (*models.User, error)
	GetEPByName(ctx context.Context, name string) (string, error)
	GetUserByName(ctx context.Context, name string) (*models.User, error)
	CheckUserByNameEmail(ctx context.Context, name, email string) bool
	// config
	GetSecretKey() []byte
}

var _ Service = (*AuthService)(nil)

type AuthService struct {
	UserRepo *models.IUserRepo
	Config   *config.Config
}

func (a AuthService) GetUserByName(ctx context.Context, name string) (*models.User, error) {
	return (*a.UserRepo).GetUserByName(ctx, name)
}

func (a AuthService) Insert(ctx context.Context, user *models.User) (*models.User, error) {
	return (*a.UserRepo).Insert(ctx, user)
}

func (a AuthService) CheckUserByNameEmail(ctx context.Context, name, email string) bool {
	return (*a.UserRepo).CheckUserByNameEmail(ctx, name, email)
}

func (a AuthService) GetSecretKey() []byte {
	return (*a.Config).Auth.SecretKey
}

func (a AuthService) GetEPByName(ctx context.Context, name string) (string, error) {
	ep, err := (*a.UserRepo).GetEPByName(ctx, name)
	if err != nil {
		return "", err
	}
	return ep, nil
}

func NewAuthService(userRepo *models.IUserRepo, config *config.Config) *AuthService {
	log.Info("initialized Auth service")
	res := &AuthService{
		UserRepo: userRepo,
		Config:   config,
	}
	return res
}
