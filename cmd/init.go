package cmd

import (
	"fmt"
	"os"
	"time"

	"github.com/jiaozifs/jiaozifs/models/migrations"

	"github.com/jiaozifs/jiaozifs/auth/rbac"
	"github.com/jiaozifs/jiaozifs/controller/validator"

	"github.com/jiaozifs/jiaozifs/auth"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/m1/go-generate-password/generator"

	"github.com/jiaozifs/jiaozifs/config"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

// initCmd represents the init command
var initCmd = &cobra.Command{
	Use:   "init",
	Short: "init jiaozifs ",
	Long:  `create default config file for jiaoozifs`,
	PreRunE: func(cmd *cobra.Command, args []string) error {
		//protect duplicate bind flag with daemon
		err := viper.BindPFlag("blockstore.local.path", cmd.Flags().Lookup("bs_path"))
		if err != nil {
			return err
		}

		err = viper.BindPFlag("database.debug", cmd.Flags().Lookup("db_debug"))
		if err != nil {
			return err
		}

		return viper.BindPFlag("database.connection", cmd.Flags().Lookup("db"))
	},
	RunE: func(cmd *cobra.Command, args []string) error {
		err := config.InitConfig(cfgFile)
		if err != nil {
			return err
		}

		cfg, err := config.LoadConfig(cfgFile)
		if err != nil {
			return err
		}

		if cfg.Blockstore.Type == "local" {
			_, err = os.Stat(cfg.Blockstore.Local.Path)
			if os.IsNotExist(err) {
				err = os.MkdirAll(cfg.Blockstore.Local.Path, 0755)
				if err != nil {
					return err
				}
			}
		}

		return initRbac(cmd, &cfg.Database)
	},
}

func initRbac(cmd *cobra.Command, cfg *config.DatabaseConfig) error {
	bunDB, err := models.NewBunDBFromConfig(cmd.Context(), cfg)
	if err != nil {
		return err
	}

	err = migrations.MigrateDatabase(cmd.Context(), bunDB)
	if err != nil {
		return err
	}

	repo := models.NewRepo(bunDB)
	userName, err := cmd.Flags().GetString("super_username")
	if err != nil {
		return err
	}

	err = validator.ValidateUsername(userName)
	if err != nil {
		return err
	}

	password, err := cmd.Flags().GetString("super_password")
	if err != nil {
		return err
	}

	if len(password) == 0 {
		config := generator.Config{
			Length:                     16,
			IncludeSymbols:             false,
			IncludeNumbers:             true,
			IncludeLowercaseLetters:    true,
			IncludeUppercaseLetters:    true,
			ExcludeSimilarCharacters:   true,
			ExcludeAmbiguousCharacters: true,
		}
		g, err := generator.New(&config)
		if err != nil {
			return err
		}

		pwd, err := g.Generate()
		if err != nil {
			return err
		}
		password = utils.StringValue(pwd)
	}
	fmt.Println("super user:", userName, password)
	passwordHash, err := auth.HashPassword(password)
	password = ""
	if err != nil {
		return err
	}
	return rbac.NewRbacAuth(repo).InitRbac(cmd.Context(), &models.User{
		Name:              userName,
		Email:             "",
		EncryptedPassword: string(passwordHash),
		CurrentSignInAt:   time.Now(),
		LastSignInAt:      time.Now(),
		CurrentSignInIP:   "",
		LastSignInIP:      "",
		CreatedAt:         time.Now(),
		UpdatedAt:         time.Now(),
	})
}
func init() {
	rootCmd.AddCommand(initCmd)
	initCmd.Flags().String("bs_path", config.DefaultLocalBSPath, "config blockstore path")
	initCmd.Flags().Bool("db_debug", false, "enable database debug")
	initCmd.Flags().String("super_username", "admin", "super name")
	initCmd.Flags().String("super_password", "", "super user name if not specific, random generate one")
	initCmd.Flags().String("db", "", "pg connection string eg. postgres://user:pass@localhost:5432/jiaozifs?sslmode=disable")
}
