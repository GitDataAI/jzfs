package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/uptrace/bun"
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
	AuthenticateUser(ctx context.Context, name, encryptedPassword string) (uuid.UUID, error)
	GetUserByName(ctx context.Context, name string) (*User, error)
	GetUserEPByName(ctx context.Context, name string) (string, error)
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
	return user, userRepo.DB.NewSelect().Model(_user).Where("id = :id", id).Scan(ctx, &user)
}

func (userRepo *UserRepo) Insert(ctx context.Context, user *User) (*User, error) {
	_, err := userRepo.DB.NewInsert().Model(user).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return user, nil
}

func (userRepo *UserRepo) AuthenticateUser(ctx context.Context, name, encryptedPassword string) (uuid.UUID, error) {
	user := &User{}
	return user.ID, userRepo.DB.NewSelect().
		Model(_user).
		Where("name = ?", name).
		Where("encrypted_password = ?", encryptedPassword).
		Scan(ctx, &user)
}

func (userRepo *UserRepo) GetUserEPByName(ctx context.Context, name string) (string, error) {
	var ep string
	return ep, userRepo.DB.NewSelect().
		Model(_user).Column("encrypted_password").Where("name = ?", name).Scan(ctx, &ep)
}

func (userRepo *UserRepo) GetUserByName(ctx context.Context, name string) (*User, error) {
	user := &User{}
	return user, userRepo.DB.NewSelect().
		Model(user).Where("name = ?", name).Scan(ctx)
}
