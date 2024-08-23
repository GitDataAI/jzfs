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

		refName, err := cmd.Flags().GetString("ref-name")
		if err != nil {
			return err
		}
		if len(refName) == 0 {
			return errors.New("refName must be set")
		}

		uploadPath, err := cmd.Flags().GetString("upload-path")
		if err != nil {
			return err
		}
		uploadPath = path2.Clean(uploadPath)
		if len(uploadPath) == 0 {
			uploadPath = "/"
		}

		ignoreRootName, err := cmd.Flags().GetBool("ignore-root-name")
		if err != nil {
			return err
		}

		replace, err := cmd.Flags().GetBool("replace")
		if err != nil {
			return err
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

			var destPath string
			if !ignoreRootName {
				destPath = path2.Join(uploadPath, basename, relativePath)
			} else {
				destPath = path2.Join(uploadPath, relativePath)
			}

			resp, err := client.UploadObjectWithBody(cmd.Context(), owner, repo, &api.UploadObjectParams{
				RefName: refName,
				// Path relative to the ref
				Path:      destPath,
				IsReplace: utils.Bool(replace),
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

// versionCmd represents the version command
var downloadCmd = &cobra.Command{
	Use:   "download",
	Short: "download files from server",
	RunE: func(cmd *cobra.Command, _ []string) error {
		ctx := cmd.Context()
		client, err := GetClient(cmd)
		if err != nil {
			return err
		}

		path, err := cmd.Flags().GetString("path")
		if err != nil {
			return err
		}
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

		refName, err := cmd.Flags().GetString("ref-name")
		if err != nil {
			return err
		}
		if len(refName) == 0 {
			return errors.New("ref-name must be set")
		}

		refType, err := cmd.Flags().GetString("ref-type")
		if err != nil {
			return err
		}
		if len(refType) == 0 {
			return errors.New("ref-type must be set")
		}

		fileName := filepath.Base(path)
		output, err := cmd.Flags().GetString("output")
		if err != nil {
			return err
		}

		if len(output) == 0 {
			fileName = output
		}

		opjResp, err := client.GetObject(ctx, owner, repo, &api.GetObjectParams{
			// Type type indicate to retrieve from wip/branch/tag, default branch
			Type:    api.RefType(refType),
			RefName: refName,
			Path:    path,
		})
		if err != nil {
			return err
		}

		headObj, err := api.ParseGetObjectResponse(opjResp)
		if err != nil {
			return err
		}

		return os.WriteFile(fileName, headObj.Body, 0666)
	},
}

func init() {
	rootCmd.AddCommand(uploadCmd)

	uploadCmd.Flags().String("path", "", "path of files to upload")
	uploadCmd.Flags().String("owner", "", "owner")
	uploadCmd.Flags().String("repo", "", "repo")
	uploadCmd.Flags().String("ref-name", "main", "branch name")
	uploadCmd.Flags().String("upload-path", "", "path to save in server")
	uploadCmd.Flags().Bool("replace", true, "path to save in server")
	uploadCmd.Flags().Bool("ignore-root-name", false, "ignore root name")

	rootCmd.AddCommand(downloadCmd)
	downloadCmd.Flags().String("path", "", "path of files to upload")
	downloadCmd.Flags().String("owner", "", "owner")
	downloadCmd.Flags().String("repo", "", "repo")
	downloadCmd.Flags().String("ref-name", "main", "branch name")
	downloadCmd.Flags().String("ref-type", "branch", "refrence type")
	downloadCmd.Flags().String("output", "branch", "refrence type")
}
