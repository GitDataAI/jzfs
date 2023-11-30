package auth

import (
	"context"
	"errors"
	"github.com/jiaozifs/jiaozifs/models"
	"golang.org/x/crypto/bcrypt"
)

var ErrAuthenticatingRequest = errors.New("error authenticating request")

// userByAuth Return the user object by accessKey and secretKey
func UserByAuth(ctx context.Context, authService models.IUserRepo, accessKey, secretKey string) (user *models.User, err error) {
	// Check accessKey and secretKey
	// Get user encryptedPassword by name
	ep, err := authService.GetUserEPByName(ctx, accessKey)
	if err != nil {
		log.Errorf("get user: %s  ep by user name err, err: %s", accessKey, err)
		return nil, ErrAuthenticatingRequest
	}
	// Compare ep and password
	err = bcrypt.CompareHashAndPassword([]byte(ep), []byte(secretKey))
	if err == nil {
		log.Infof("password is correct")
	} else if err == bcrypt.ErrMismatchedHashAndPassword {
		log.Errorf("password is incorrect")
		return
	} else {
		log.Errorf("Else error: ", err)
		return
	}

	// Get user
	user, err = authService.GetUserByName(ctx, accessKey)
	if err != nil {
		log.Errorf("could not find user name: %s, err: %s", accessKey, err)
		return nil, ErrAuthenticatingRequest
	}
	return user, nil
}
