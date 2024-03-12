/*
Copyright © 2023 githun.com/GitDataAI/jiaozifs
*/
package main

import (
	"github.com/GitDataAI/jiaozifs/cmd"
	_ "github.com/deepmap/oapi-codegen/v2/pkg/codegen"
	_ "gopkg.in/yaml.v2"
)

func main() {
	cmd.Execute()
}
