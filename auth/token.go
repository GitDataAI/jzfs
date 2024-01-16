package auth

import (
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"

	"github.com/golang-jwt/jwt/v5"
)

var (
	ErrUnexpectedSigningMethod = errors.New("unexpected signing method")
)

const (
	LoginAudience = "login"
)

// GenerateJWTLogin creates a jwt token which can be used for authentication during login only, i.e. it will not work for password reset.
// It supports backward compatibility for creating a login jwt. The audience is not set for login token. Any audience will make the token
// invalid for login. No email is passed to support the ability of login for users via user/access keys which don't have an email yet
func GenerateJWTLogin(secret []byte, userID string, issuedAt, expiresAt time.Time) (string, error) {
	claims := jwt.MapClaims{
		"id":  uuid.NewString(),
		"aud": LoginAudience,
		"sub": userID,
		"iat": issuedAt.Unix(),
		"exp": expiresAt.Unix(),
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(secret)
}

// VerifyToken verifies the authenticity of a token using a secret key.
//
// It takes in the following parameters:
// - secret []byte: the secret key used to sign the token
// - tokenString string: the token string to be verified
//
// It returns the following:
// - jwt.Claims: the claims extracted from the token
// - error: any error encountered during token verification
func VerifyToken(secret []byte, tokenString string) (jwt.Claims, error) {
	claims := &jwt.MapClaims{}
	token, err := jwt.ParseWithClaims(tokenString, claims, func(token *jwt.Token) (interface{}, error) {
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("%w: %s", ErrUnexpectedSigningMethod, token.Header["alg"])
		}
		return secret, nil
	})
	if err != nil || !token.Valid {
		return nil, ErrInvalidToken
	}

	return claims, nil
}
