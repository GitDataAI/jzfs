package auth

import (
	"context"
	"testing"

	"github.com/jiaozifs/jiaozifs/testhelper"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/stretchr/testify/require"
)

func TestLogin_Success(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint
	// repo
	mockRepo := models.NewUserRepo(db)
	// config
	mockConfig := &config.AuthConfig{SecretKey: []byte("THIS_MUST_BE_CHANGED_IN_PRODUCTION")}
	// user
	userModel := &models.User{}
	require.NoError(t, gofakeit.Struct(userModel))

	// registration
	register := &Register{
		Username: userModel.Name,
		Email:    userModel.Email,
		Password: userModel.EncryptedPassword,
	}
	err := register.Register(ctx, mockRepo)
	require.NoError(t, err)

	// login
	login := &Login{
		Username: userModel.Name,
		Password: userModel.EncryptedPassword,
	}
	token, err := login.Login(context.Background(), mockRepo, mockConfig)
	require.NoError(t, err, "Login should not return an error")
	require.NotEmpty(t, token.Token, "Token should not be empty")
	require.NotNil(t, token.TokenExpiration, "Token expiration should not be nil")
}
