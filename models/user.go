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

type GetUserParam struct {
	ID    uuid.UUID
	Name  *string
	Email *string
}

type CountUserParams = GetUserParam

type IUserRepo interface {
	Get(ctx context.Context, params *GetUserParam) (*User, error)
	Count(ctx context.Context, params *CountUserParams) (int, error)
	Insert(ctx context.Context, user *User) (*User, error)
	GetEPByName(ctx context.Context, name string) (string, error)
}

var _ IUserRepo = (*UserRepo)(nil)

type UserRepo struct {
	db bun.IDB
}

func NewUserRepo(db bun.IDB) IUserRepo {
	return &UserRepo{db: db}
}

func (userRepo *UserRepo) Get(ctx context.Context, params *GetUserParam) (*User, error) {
	user := &User{}
	query := userRepo.db.NewSelect().Model(user)

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if params.Name != nil {
		query = query.Where("name = ?", params.Name)
	}

	if params.Email != nil {
		query = query.Where("email = ?", *params.Email)
	}

	return user, query.Limit(1).Scan(ctx)
}

func (userRepo *UserRepo) Count(ctx context.Context, params *GetUserParam) (int, error) {
	query := userRepo.db.NewSelect().Model((*User)(nil))

	if uuid.Nil != params.ID {
		query = query.Where("id = ?", params.ID)
	}

	if params.Name != nil {
		query = query.Where("name = ?", params.Name)
	}

	if params.Email != nil {
		query = query.Where("email = ?", *params.Email)
	}

	return query.Count(ctx)
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
	return ep, userRepo.db.NewSelect().
		Model((*User)(nil)).Column("encrypted_password").
		Where("name = ?", name).
		Scan(ctx, &ep)
}
