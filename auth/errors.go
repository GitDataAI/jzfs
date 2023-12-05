package auth

import "errors"

var (
	ErrComparePassword  = errors.New("compare password error")
	ErrParseToken       = errors.New("parse token error")
	ErrInvalidToken     = errors.New("invalid token")
	ErrInvalidNameEmail = errors.New("invalid name or email")
	ErrExtractClaims    = errors.New("failed to extract claims from JWT token")
)
