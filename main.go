/*
Copyright © 2023 githun.com/GitDataAI/jiaozifs
*/
package main

import (
	_ "github.com/deepmap/oapi-codegen/v2/pkg/codegen"
	"github.com/GitDataAI/jiaozifs/cmd"
	_ "gopkg.in/yaml.v2"
)

func main() {
	cmd.Execute()
}
