package utils

import (
	"fmt"
	"os"

	"github.com/mitchellh/go-homedir"
)

func RemoveLocalFiles(path, filename string) error {
	filepath := fmt.Sprintf("%s/%s", path, filename)

	expandPath, err := homedir.Expand(filepath)
	if err != nil {
		return err
	}

	fileInfo, err := os.Stat(expandPath)
	if os.IsNotExist(err) {
		return nil
	} else if err != nil {
		return err
	}

	if fileInfo.IsDir() {
		err := os.RemoveAll(expandPath)
		if err != nil {
			return err
		}
	} else {
		os.Remove(expandPath)
		if err != nil {
			return err
		}
	}

	return nil
}
