package auth

import "errors"

var (
	ErrInvalidToken     = errors.New("invalid token")
	ErrInvalidNameEmail = errors.New("invalid name or email")
	ErrExtractClaims    = errors.New("failed to extract claims from JWT token")
)
