package cmd

import (
	"io"
	"net/http"

	"github.com/GitDataAI/jiaozifs/api"
	"github.com/spf13/cobra"
)

func GetClient(cmd *cobra.Command) (*api.Client, error) {
	url := cmd.Flags().Lookup("url").Value.String()
	ak := cmd.Flags().Lookup("ak").Value.String()
	sk := cmd.Flags().Lookup("sk").Value.String()

	user := cmd.Flags().Lookup("user").Value.String()
	password := cmd.Flags().Lookup("password").Value.String()

	if len(ak) > 0 {
		return api.NewClient(url, api.AkSkOption(ak, sk))
	}
	return api.NewClient(url, api.UPOption(user, password))
}

func tryLogError(resp *http.Response) string {
	bodyContent, err := io.ReadAll(resp.Body)
	if err != nil {
		return ""
	}
	return string(bodyContent)
}
