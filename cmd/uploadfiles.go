package cmd

import (
	"errors"
	"fmt"
	"io/fs"
	"net/http"
	"os"
	path2 "path"
	"path/filepath"
	"strings"

	"github.com/GitDataAI/jiaozifs/utils"

	"github.com/GitDataAI/jiaozifs/api"

	"github.com/spf13/cobra"
)

// versionCmd represents the version command
var uploadCmd = &cobra.Command{
	Use:   "upload",
	Short: "upload files to server",
	RunE: func(cmd *cobra.Command, _ []string) error {
		client, err := GetClient(cmd)
		if err != nil {
			return err
		}

		path, err := cmd.Flags().GetString("path")
		if err != nil {
			return err
		}
		path = path2.Clean(path)
		if len(path) == 0 {
			return errors.New("path must be set")
		}

		owner, err := cmd.Flags().GetString("owner")
		if err != nil {
			return err
		}
		if len(owner) == 0 {
			return errors.New("owner must be set")
		}

		repo, err := cmd.Flags().GetString("repo")
		if err != nil {
			return err
		}
		if len(owner) == 0 || len(repo) == 0 {
			return errors.New("owner and repo must be set")
		}

		refName, err := cmd.Flags().GetString("refName")
		if err != nil {
			return err
		}
		if len(refName) == 0 {
			return errors.New("refName must be set")
		}

		uploadPath, err := cmd.Flags().GetString("uploadPath")
		if err != nil {
			return err
		}
		uploadPath = path2.Clean(uploadPath)
		if len(uploadPath) == 0 {
			uploadPath = "/"
		}

		if len(path) == 0 {
			return errors.New("path not set")
		}

		st, err := os.Stat(path)
		if err != nil {
			return err
		}

		var files []string
		if st.IsDir() {
			err = filepath.Walk(path, func(path string, info fs.FileInfo, _ error) error {
				if info.IsDir() {
					return nil
				}
				files = append(files, path)
				return nil
			})
			if err != nil {
				return err
			}
		}

		fmt.Printf("Files dected, %d files need to be uploaded\n", len(files))
		_, err = client.GetWip(cmd.Context(), owner, repo, &api.GetWipParams{RefName: refName})
		if err != nil {
			return err
		}

		basename := filepath.Base(path)
		for _, file := range files {
			fs, err := os.Open(file)
			if err != nil {
				return err
			}
			relativePath := strings.Replace(file, path, "", 1)
			destPath := path2.Join(uploadPath, basename, relativePath)

			resp, err := client.UploadObjectWithBody(cmd.Context(), owner, repo, &api.UploadObjectParams{
				RefName: refName,
				// Path relative to the ref
				Path:      destPath,
				IsReplace: utils.Bool(true),
			}, "application/json", fs)
			if err != nil {
				return err
			}

			if resp.StatusCode == http.StatusCreated || resp.StatusCode == http.StatusOK {
				fmt.Println("Upload file success ", file, " dest path", destPath)
				continue
			}
			return fmt.Errorf("upload file failed %d, %s", resp.StatusCode, tryLogError(resp))
		}
		return nil
	},
}

func init() {
	rootCmd.AddCommand(uploadCmd)

	uploadCmd.Flags().String("path", "", "path of files to upload")
	uploadCmd.Flags().String("owner", "", "owner")
	uploadCmd.Flags().String("repo", "", "repo")
	uploadCmd.Flags().String("refName", "main", "branch name")
	uploadCmd.Flags().String("uploadPath", "", "path to save in server")
	uploadCmd.Flags().Bool("replace", true, "path to save in server")
}
