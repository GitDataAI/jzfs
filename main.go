/*
Copyright © 2023 githun.com/jiaozifs/jiaozifs
*/
package main

import (
	_ "github.com/deepmap/oapi-codegen/v2/pkg/codegen"
	"github.com/jiaozifs/jiaozifs/cmd"
	_ "gopkg.in/yaml.v2"
)

func main() {
	cmd.Execute()
}
