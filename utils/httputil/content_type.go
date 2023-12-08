package httputil

import (
	"mime"
	"path/filepath"
	"strings"
)

func ExtensionsByType(fileName string) string {
	ext := filepath.Ext(fileName)
	m := mime.TypeByExtension(ext)
	if len(m) == 0 {
		return "application/octet-stream"
	}
	return strings.Split(m, ";")[0] //remove charset part
}
