package auth

import (
	"github.com/gorilla/sessions"
	"github.com/jiaozifs/jiaozifs/auth/crypt"
	"github.com/jiaozifs/jiaozifs/config"
)

func NewSessionStore(authConfig *config.AuthConfig) sessions.Store {
	sstore := crypt.NewSecretStore(authConfig.SecretKey)
	return sessions.NewCookieStore(sstore.SharedSecret())
}
