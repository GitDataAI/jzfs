package models

import (
	"context"

	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
)

type User struct {
	bun.BaseModel     `bun:"table:users"`
	ID                uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()"`
	Name              string    `bun:"name,notnull"`
	Email             string    `bun:"email,notnull"`
	EncryptedPassword string    `bun:"encrypted_password"`
	CurrentSignInAt   time.Time `bun:"current_sign_in_at"`
	LastSignInAt      time.Time `bun:"last_sign_in_at"`
	CurrentSignInIP   string    `bun:"current_sign_in_ip"`
	LastSignInIP      string    `bun:"last_sign_in_ip"`
	CreatedAt         time.Time `bun:"created_at"`
	UpdatedAt         time.Time `bun:"updated_at"`
}

type IUserRepo interface {
	Get(ctx context.Context, id uuid.UUID) (*User, error)
	GetByName(ctx context.Context, name string) (*User, error)
	Insert(ctx context.Context, user *User) (*User, error)

	GetEPByName(ctx context.Context, name string) (string, error)
	GetUserByName(ctx context.Context, name string) (*User, error)
	GetUserByEmail(ctx context.Context, email string) (*User, error)
}

var _ IUserRepo = (*UserRepo)(nil)

type UserRepo struct {
	db *bun.DB
}

func NewUserRepo(db *bun.DB) IUserRepo {
	return &UserRepo{db}
}

func (userRepo *UserRepo) GetByName(ctx context.Context, name string) (*User, error) {
	user := &User{}
	return user, userRepo.db.NewSelect().Model(user).Where("name = ?", name).Scan(ctx)
}

func (userRepo *UserRepo) Get(ctx context.Context, id uuid.UUID) (*User, error) {
	user := &User{}
	return user, userRepo.db.NewSelect().Model(user).Where("id = ?", id).Scan(ctx)
}

func (userRepo *UserRepo) Insert(ctx context.Context, user *User) (*User, error) {
	_, err := userRepo.db.NewInsert().Model(user).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return user, nil
}

func (userRepo *UserRepo) GetEPByName(ctx context.Context, name string) (string, error) {
	var ep string
	return ep, userRepo.DB.NewSelect().
		Model((*User)(nil)).Column("encrypted_password").
		Where("name = ?", name).
		Scan(ctx, &ep)
}

func (userRepo *UserRepo) GetUserByName(ctx context.Context, name string) (*User, error) {
	user := &User{}
	return user, userRepo.DB.NewSelect().Model(user).Where("name = ?", name).Scan(ctx)
}

func (userRepo *UserRepo) GetUserByEmail(ctx context.Context, email string) (*User, error) {
	user := &User{}
	return user, userRepo.DB.NewSelect().
		Model(user).Where("email = ?", email).Scan(ctx)
}
