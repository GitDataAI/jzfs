package auth

import (
	"context"
	"fmt"
	"time"

	logging "github.com/ipfs/go-log/v2"
	"github.com/jiaozifs/jiaozifs/models"
)

var log = logging.Logger("auth")

type Register struct {
	Username string `json:"username"`
	Email    string `json:"email"`
	Password string `json:"password"`
}

func (r *Register) Register(ctx context.Context, repo models.IUserRepo) error {
	// check username, email
	count1, err := repo.Count(ctx, models.NewCountUserParam().SetName(r.Username))
	if err != nil {
		return err
	}
	count2, err := repo.Count(ctx, models.NewCountUserParam().SetName(r.Email))
	if err != nil {
		return err
	}

	if count1+count2 > 0 {
		return fmt.Errorf("username %s or email %s not found %w ", r.Username, r.Email, ErrInvalidNameEmail)
	}

	// reserve temporarily
	password, err := HashPassword(r.Password)
	if err != nil {
		return fmt.Errorf("invalid password %w", err)
	}

	// insert db
	user := &models.User{
		Name:              r.Username,
		Email:             r.Email,
		EncryptedPassword: string(password),
		CurrentSignInAt:   time.Time{},
		LastSignInAt:      time.Time{},
		CurrentSignInIP:   "",
		LastSignInIP:      "",
		CreatedAt:         time.Now(),
		UpdatedAt:         time.Now(),
	}
	insertUser, err := repo.Insert(ctx, user)
	if err != nil {
		return fmt.Errorf("inser user %s user error %w", r.Username, err)
	}

	log.Infof("%s registration success", insertUser.Name)
	return nil
}
