SHELL=/usr/bin/env bash

all: build
.PHONY: all

ldflags=-X=github.com/jiaozofs/jiaozifs/version.CurrentCommit=+git.$(subst -,.,$(shell git describe --always --match=NeVeRmAtCh --dirty 2>/dev/null || git rev-parse --short HEAD 2>/dev/null))
ifneq ($(strip $(LDFLAGS)),)
	ldflags+=-extldflags=$(LDFLAGS)
endif

GOFLAGS+=-ldflags="$(ldflags)"

install-go-swagger:
	go install github.com/go-swagger/go-swagger/cmd/swagger@latest

SWAGGER_ARG=
swagger-srv:
	 swagger serve $(SWAGGER_ARG) -F swagger  ./api/swagger.yml

build
	go build $(GOFLAGS) -o jiaozifs
