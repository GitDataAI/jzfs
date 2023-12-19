package auth

import (
	"encoding/hex"

	"github.com/gorilla/sessions"
	"github.com/jiaozifs/jiaozifs/auth/crypt"
	"github.com/jiaozifs/jiaozifs/config"
)

func NewSessionStore(secretStrore crypt.SecretStore) sessions.Store {
	return sessions.NewCookieStore(secretStrore.SharedSecret())
}

func NewSectetStore(authConfig *config.AuthConfig) (crypt.SecretStore, error) {
	secretKey, err := hex.DecodeString(authConfig.SecretKey)
	if err != nil {
		return nil, err
	}
	return crypt.NewSecretStore(secretKey), nil
}
