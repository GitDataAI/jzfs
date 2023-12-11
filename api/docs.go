// Package api provides generated code for our OpenAPI
package api

//go:generate go run github.com/deepmap/oapi-codegen/v2/cmd/oapi-codegen -package api -templates ./tmpls -generate "types,client,chi-server,spec" -o jiaozifs.gen.go ./swagger.yml
//go:generate  go run go.uber.org/mock/mockgen@latest --package=api --destination=resp.gen.go net/http ResponseWriter
