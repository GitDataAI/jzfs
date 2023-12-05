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

var _ Service = (*ServiceAuth)(nil)

type ServiceAuth struct {
	UserRepo *models.IUserRepo
	Config   *config.Config
}

func (a ServiceAuth) GetUserByName(ctx context.Context, name string) (*models.User, error) {
	return (*a.UserRepo).GetUserByName(ctx, name)
}

func (a ServiceAuth) Insert(ctx context.Context, user *models.User) (*models.User, error) {
	return (*a.UserRepo).Insert(ctx, user)
}

func (a ServiceAuth) CheckUserByNameEmail(ctx context.Context, name, email string) bool {
	return (*a.UserRepo).CheckUserByNameEmail(ctx, name, email)
}

func (a ServiceAuth) GetSecretKey() []byte {
	return (*a.Config).Auth.SecretKey
}

func (a ServiceAuth) GetEPByName(ctx context.Context, name string) (string, error) {
	ep, err := (*a.UserRepo).GetEPByName(ctx, name)
	if err != nil {
		return "", err
	}
	return ep, nil
}

func NewAuthService(userRepo *models.IUserRepo, config *config.Config) *ServiceAuth {
	log.Info("initialized Auth service")
	res := &ServiceAuth{
		UserRepo: userRepo,
		Config:   config,
	}
	return res
}
