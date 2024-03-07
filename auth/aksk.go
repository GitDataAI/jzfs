package auth

import (
	"context"

	"github.com/jiaozifs/jiaozifs/auth/aksk"
	"github.com/jiaozifs/jiaozifs/models"
)

var _ aksk.SkGetter = (*SkGetter)(nil)

type SkGetter struct {
	akskRepo models.IAkskRepo
}

func (s SkGetter) Get(ak string) (string, error) {
	aksk, err := s.akskRepo.Get(context.Background(), models.NewGetAkSkParams().SetAccessKey(ak))
	if err != nil {
		return "", err
	}
	return aksk.SecretKey, nil
}

func NewAkskVerifier(repo models.IRepo) aksk.Verifier {
	return aksk.NewV0Verier(SkGetter{repo.AkskRepo()})
}
