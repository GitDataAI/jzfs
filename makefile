SHELL=/usr/bin/env bash
GOCMD=$(or $(shell which go), $(error "Missing dependency - no go in PATH"))

GOGENERATE=$(GOCMD) generate

all: build
.PHONY: all

ldflags=-X=github.com/jiaozifs/jiaozifs/version.CurrentCommit=+git.$(subst -,.,$(shell git describe --always --match=NeVeRmAtCh --dirty 2>/dev/null || git rev-parse --short HEAD 2>/dev/null))
ifneq ($(strip $(LDFLAGS)),)
	ldflags+=-extldflags=$(LDFLAGS)
endif

GOFLAGS+=-ldflags="$(ldflags)"

gen-api: ./api/swagger.yml ./api/tmpls/chi
	$(GOGENERATE) ./api

install-go-swagger:
	go install github.com/go-swagger/go-swagger/cmd/swagger@latest

SWAGGER_ARG=
swagger-srv:
	 swagger serve $(SWAGGER_ARG) -F swagger  ./api/swagger.yml

test: gen-api
	go test -timeout=30m -parallel=4  -v ./...
build:gen-api
	go build $(GOFLAGS) -o jzfs
