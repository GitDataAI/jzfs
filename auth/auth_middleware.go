package auth

import (
	"context"
	"errors"
	"github.com/jiaozifs/jiaozifs/models"
)

var ErrAuthenticatingRequest = errors.New("error authenticating request")

// userByAuth Return the user object by accessKey and secretKey
func UserByAuth(ctx context.Context, authService models.IUserRepo, accessKey string, secretKey string) (*models.User, error) {
	id, err := authService.AuthenticateUser(ctx, accessKey, secretKey)
	if err != nil {
		log.Errorf("authenticator user: %s, err: %s", accessKey, err)
		return nil, ErrAuthenticatingRequest
	}
	user, err := authService.GetUser(ctx, id)
	if err != nil {
		log.Errorf("could not find user id: %s, err: %s", id, err)
		return nil, ErrAuthenticatingRequest
	}
	return user, nil
}
