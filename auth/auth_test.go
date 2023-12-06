package auth

import (
	"context"
	"fmt"
	"github.com/brianvoe/gofakeit/v6"
	embeddedpostgres "github.com/fergusstrange/embedded-postgres"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/migrations"
	"github.com/phayes/freeport"
	"github.com/stretchr/testify/require"
	"github.com/uptrace/bun"
	"go.uber.org/fx/fxtest"
	"testing"
)

var testConnTmpl = "postgres://postgres:postgres@localhost:%d/jiaozifs?sslmode=disable"

func setup(ctx context.Context, t *testing.T) (*embeddedpostgres.EmbeddedPostgres, *bun.DB) {
	port, err := freeport.GetFreePort()
	require.NoError(t, err)
	postgres := embeddedpostgres.NewDatabase(embeddedpostgres.DefaultConfig().Port(uint32(port)).Database("jiaozifs"))
	err = postgres.Start()
	require.NoError(t, err)

	db, err := models.SetupDatabase(ctx, fxtest.NewLifecycle(t), &config.DatabaseConfig{Debug: true, Connection: fmt.Sprintf(testConnTmpl, port)})
	require.NoError(t, err)

	err = migrations.MigrateDatabase(ctx, db)
	require.NoError(t, err)
	return postgres, db
}

func TestLogin_Success(t *testing.T) {
	ctx := context.Background()
	postgres, db := setup(ctx, t)
	defer postgres.Stop() //nolint
	// repo
	mockRepo := models.NewUserRepo(db)
	// config
	mockConfig := &config.Config{Auth: config.AuthConfig{SecretKey: []byte("THIS_MUST_BE_CHANGED_IN_PRODUCTION")}}
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
	// profile
	userInfo := &UserInfo{Token: token.Token}
	profile, err := userInfo.UserProfile(ctx, mockRepo, mockConfig)
	require.NoError(t, err)
	require.NotEmpty(t, profile)
}
