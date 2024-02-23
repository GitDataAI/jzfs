package auth

import (
	"context"

	"github.com/jiaozifs/jiaozifs/models"
	"golang.org/x/crypto/bcrypt"
)

type BasicAuthenticator struct {
	userRepo models.IUserRepo
}

func NewBasicAuthenticator(userRepo models.IUserRepo) *BasicAuthenticator {
	return &BasicAuthenticator{userRepo: userRepo}
}

func (b BasicAuthenticator) AuthenticateUser(ctx context.Context, user, password string) (*models.User, error) {
	// get user encryptedPassword by username
	ep, err := b.userRepo.GetEPByName(ctx, user)
	if err != nil {
		return nil, err
	}

	// Compare ep and password
	err = bcrypt.CompareHashAndPassword([]byte(ep), []byte(password))
	if err != nil {
		return nil, err
	}

	return b.userRepo.Get(ctx, models.NewGetUserParams().SetName(user))
}
