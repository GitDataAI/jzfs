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
	return api.NewClient(url, api.AkSkOption(ak, sk))
}

func tryLogError(resp *http.Response) string {
	bodyContent, err := io.ReadAll(resp.Body)
	if err != nil {
		return ""
	}
	return string(bodyContent)
}
