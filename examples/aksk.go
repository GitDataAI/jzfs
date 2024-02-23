package main

import (
	"bytes"
	"context"
	"flag"
	"log"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/api"
	apiimpl "github.com/jiaozifs/jiaozifs/api/api_impl"
)

var url string
var ak string
var sk string
var repoName string

func init() {
	flag.StringVar(&url, "url", "", "jiaozifs endpoint")
	flag.StringVar(&ak, "ak", "", "jiaozifs ak")
	flag.StringVar(&sk, "sk", "", "jiaozifs sk")
	flag.StringVar(&repoName, "repo", "", "repo to create")
}
func main() {
	flag.Parse()
	log.Println(func(ctx context.Context) error {
		cli, err := api.NewClient(url+apiimpl.APIV1Prefix, api.AkSkOption(ak, sk))
		if err != nil {
			return err
		}
		resp, err := cli.GetUserInfo(ctx)
		userInfo, err := api.ParseGetUserInfoResponse(resp)
		if err != nil {
			return err
		}
		log.Println("User", userInfo.JSON200.Name, "Login")

		//create repo
		resp, err = cli.CreateRepository(ctx, api.CreateRepositoryJSONRequestBody{
			Name: repoName,
		})
		if err != nil {
			return err
		}
		repo, err := api.ParseCreateRepositoryResponse(resp)
		if err != nil {
			return err
		}
		log.Println("Create Repo", repo.JSON201.Name, "ID:", repo.JSON201.Id)

		//create branch
		branchName := "branch_test"
		resp, err = cli.CreateBranch(ctx, userInfo.JSON200.Name, repo.JSON201.Name, api.CreateBranchJSONRequestBody{
			Name:   branchName,
			Source: "main",
		})
		if err != nil {
			return err
		}
		_, err = api.ParseCreateBranchResponse(resp)
		if err != nil {
			return err
		}
		log.Println("Create Branch", repo.JSON201.Name, "ID:", repo.JSON201.Id)

		//create draft
		resp, err = cli.GetWip(ctx, userInfo.JSON200.Name, repo.JSON201.Name, &api.GetWipParams{RefName: branchName})
		if err != nil {
			return err
		}
		log.Println("create draft for main branch")
		// upload files to draft
		resp, err = cli.UploadObjectWithBody(ctx, userInfo.JSON200.Name, repo.JSON201.Name, &api.UploadObjectParams{
			RefName: branchName,
			Path:    "a.bin",
		}, "application/octet-stream", bytes.NewBufferString("hello, world!!"))
		if err != nil {
			return err
		}

		uploadResult, err := api.ParseUploadObjectResponse(resp)
		if err != nil {
			return err
		}
		log.Println("upload a file size:", uploadResult.JSON201.SizeBytes, "checksum", uploadResult.JSON201.Checksum)

		//commit draft
		resp, err = cli.CommitWip(ctx, userInfo.JSON200.Name, repo.JSON201.Name, &api.CommitWipParams{
			RefName: branchName,
			Msg:     "test",
		})
		if err != nil {
			return err
		}
		commitResult, err := api.ParseCommitWipResponse(resp)
		if err != nil {
			return err
		}
		log.Println("commit changes. hash:", commitResult.JSON201.BaseCommit)
		// merge branch

		resp, err = cli.CreateMergeRequest(ctx, userInfo.JSON200.Name, repo.JSON201.Name, api.CreateMergeRequestJSONRequestBody{
			Description:      utils.String("create merge request test"),
			SourceBranchName: branchName,
			TargetBranchName: "main",
			Title:            "Merge: test",
		})
		if err != nil {
			return err
		}
		mergeRequestResult, err := api.ParseCreateMergeRequestResponse(resp)
		if err != nil {
			return err
		}
		log.Println("create merge request merge id", mergeRequestResult.JSON201.Sequence)

		_, err = cli.Merge(ctx, userInfo.JSON200.Name, repo.JSON201.Name, mergeRequestResult.JSON201.Sequence, api.MergeJSONRequestBody{
			Msg: "merge it",
		})
		log.Println("merge success")

		//read the file from main branch
		resp, err = cli.GetObject(ctx, userInfo.JSON200.Name, repo.JSON201.Name, &api.GetObjectParams{
			Type:    api.RefTypeBranch,
			RefName: "main",
			Path:    "a.bin",
			Range:   nil,
		})
		if err != nil {
			return err
		}

		result, err := api.ParseGetObjectResponse(resp)
		log.Println("get object content ", string(result.Body))
		return nil
	}(context.Background()))
}
