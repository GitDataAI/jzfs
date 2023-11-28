package models

import (
	"context"
	"github.com/google/uuid"
	"github.com/uptrace/bun"
	"time"
)

var _user = (*User)(nil)

type User struct {
	bun.BaseModel     `bun:"table:users,alias:u"`
	ID                uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Name              string    `bun:"name,notnull"`
	Email             string    `bun:"email,notnull"`
	EncryptedPassword string    `bun:"encrypted_password"`
	CurrentSignInAt   int64     `bun:"current_sign_in_at"`
	LastSignInAt      int64     `bun:"last_sign_in_at"`
	CurrentSignInIP   string    `bun:"current_sign_in_ip"`
	LastSignInIP      string    `bun:"last_sign_in_ip"`
	CreatedAt         time.Time `bun:"created_at,type:timestamp"`
	UpdatedAt         time.Time `bun:"updated_at,type:timestamp"`
}

type IUserRepo interface {
	GetUser(ctx context.Context, id uuid.UUID) (*User, error)
	Insert(ctx context.Context, user *User) (*User, error)
}

var _ IUserRepo = (*UserRepo)(nil)

type UserRepo struct {
	*bun.DB
}

func NewUserRepo(db *bun.DB) IUserRepo {
	return &UserRepo{db}
}

func (userRepo *UserRepo) GetUser(ctx context.Context, id uuid.UUID) (*User, error) {
	user := &User{}
	return user, userRepo.DB.NewSelect().Model(_user).Where("id = 1").Scan(ctx, &user)
}

func (userRepo *UserRepo) Insert(ctx context.Context, user *User) (*User, error) {
	_, err := userRepo.DB.NewInsert().Model(user).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return user, nil
}
