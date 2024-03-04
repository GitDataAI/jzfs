package auth

import (
	"time"

	"golang.org/x/crypto/bcrypt"
)

const (
	ExpirationDuration = time.Hour
	PasswordCost       = 12
)

func HashPassword(password string) ([]byte, error) {
	return bcrypt.GenerateFromPassword([]byte(password), PasswordCost)
}
