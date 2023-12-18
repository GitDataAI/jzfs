package auth

import (
	"github.com/gorilla/sessions"
	"github.com/jiaozifs/jiaozifs/auth/crypt"
	"github.com/jiaozifs/jiaozifs/config"
)

func NewSessionStore(secretStrore crypt.SecretStore) sessions.Store {
	return sessions.NewCookieStore(secretStrore.SharedSecret())
}

func NewSectetStore(authConfig *config.AuthConfig) crypt.SecretStore {
	return crypt.NewSecretStore(authConfig.SecretKey)
}
