package cmd

import (
	"fmt"

	"github.com/GitDataAI/jiaozifs/api"
	"github.com/GitDataAI/jiaozifs/utils"
	"github.com/spf13/cobra"
)

// versionCmd represents the version command
var akskCmd = &cobra.Command{
	Use:   "aksk",
	Short: "ak sk command",
}

var createAkskCmd = &cobra.Command{
	Use:   "create",
	Short: "create ak/sk",
	RunE: func(cmd *cobra.Command, _ []string) error {
		client, err := GetClient(cmd)
		if err != nil {
			return err
		}

		desc := cmd.Flags().Lookup("description").Value.String()
		resp, err := client.CreateAksk(cmd.Context(), &api.CreateAkskParams{
			Description: utils.String(desc),
		})
		if err != nil {
			return fmt.Errorf("request aksk %w", err)
		}

		result, err := api.ParseCreateAkskResponse(resp)
		if err != nil {
			return fmt.Errorf("parser aksk response %w", err)
		}

		fmt.Printf("ak %s sk %s \n", result.JSON201.AccessKey, result.JSON201.SecretKey)
		return nil
	},
}

func init() {
	rootCmd.AddCommand(akskCmd)

	akskCmd.AddCommand(createAkskCmd)
	createAkskCmd.Flags().String("description", "", "description")
}
