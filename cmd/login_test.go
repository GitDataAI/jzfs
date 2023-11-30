package cmd

import (
	"context"
	"fmt"
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/config"
	"testing"
)

func TestLogin(t *testing.T) {
	ctx := context.Background()
	cfg, err := config.LoadConfig(cfgFile)
	if err != nil {
		t.Errorf("load config err: %s", err)
	}
	client, err := api.NewClient(cfg.API.Listen)
	if err != nil {
		t.Errorf("api new client err: %s", err)
	}
	params := &api.LoginParams{
		AccessKeyId:     "123456",
		SecretAccessKey: "abc123",
	}
	loginResp, err := client.Login(ctx, params)
	if err != nil {
		t.Errorf("client login err: %s", err)
	}
	//fmt.Println(loginResp)
	okResp, err := api.ParseLoginResponse(loginResp)
	if err != nil {
		t.Errorf("parse login response err: %s", err)
	}
	if okResp.JSON200 == nil {
		t.Errorf("response json200 err: %s", err)
	}
	fmt.Println("Token ", okResp.JSON200.Token)
	fmt.Println("Token Expiration ", okResp.JSON200.TokenExpiration)
	return
}
