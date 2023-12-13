package auth

import (
	"context"
	"fmt"

	"github.com/jiaozifs/jiaozifs/models"
)

type contextKey string

const (
	userContextKey contextKey = "user"
)

func GetUser(ctx context.Context) (*models.User, error) {
	user, ok := ctx.Value(userContextKey).(*models.User)
	if !ok {
		return nil, fmt.Errorf("UserNotFound")
	}
	return user, nil
}

func WithUser(ctx context.Context, user *models.User) context.Context {
	return context.WithValue(ctx, userContextKey, user)
}
