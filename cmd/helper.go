package cmd

import (
	"github.com/jiaozifs/jiaozifs/api"
	"github.com/jiaozifs/jiaozifs/config"
)

func GetDefaultClient() (*api.Client, error) {
	swagger, err := api.GetSwagger()
	if err != nil {
		return nil, err
	}

	//get runtime version
	cfg, err := config.LoadConfig(cfgFile)
	if err != nil {
		return nil, err
	}
	basePath, err := swagger.Servers[0].BasePath()
	if err != nil {
		return nil, err
	}
	return api.NewClient(cfg.API.Listen + basePath)
}
