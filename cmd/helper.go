package cmd

import (
	"github.com/GitDataAI/jiaozifs/api"
	"github.com/GitDataAI/jiaozifs/config"
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
