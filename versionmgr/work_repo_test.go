package versionmgr

import (
	"bytes"
	"context"
	"encoding/hex"
	"fmt"
	"io"
	"os"
	"path"
	"strings"
	"testing"
	"time"

	"github.com/brianvoe/gofakeit/v6"
	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/block/mem"
	"github.com/jiaozifs/jiaozifs/config"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/models/filemode"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/jiaozifs/jiaozifs/utils"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestTreeWriteBlob(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	adapter := mem.New(ctx)
	repo := models.NewRepo(db)

	userModel := &models.User{}
	require.NoError(t, gofakeit.Struct(userModel))
	userModel, err := repo.UserRepo().Insert(ctx, userModel)
	require.NoError(t, err)

	repoModel := &models.Repository{}
	require.NoError(t, gofakeit.Struct(repoModel))
	repoModel.CreatorID = userModel.ID
	repoModel.StorageNamespace = "mem://data"
	repoModel, err = repo.RepositoryRepo().Insert(ctx, repoModel)
	require.NoError(t, err)

	workRepo := NewWorkRepositoryFromAdapter(ctx, userModel, repoModel, repo, adapter)

	binary := []byte("Build simple, secure, scalable systems with Go")
	bLen := int64(len(binary))
	r := bytes.NewReader(binary)
	blob, err := workRepo.WriteBlob(ctx, r, bLen, models.DefaultLeafProperty())
	require.NoError(t, err)
	assert.Equal(t, bLen, blob.Size)
	assert.Equal(t, "99b91d4c517d0cded9506be9298b8d02", blob.Hash.Hex())
	assert.Equal(t, "f3b39786b86a96372589aa1166966643", blob.CheckSum.Hex())

	reader, err := workRepo.ReadBlob(ctx, blob, nil)
	require.NoError(t, err)
	content, err := io.ReadAll(reader)
	require.NoError(t, err)
	require.Equal(t, binary, content)
}

func TestNewWorkRepositoryFromConfig(t *testing.T) {
	ctx := context.Background()
	tmpDir, err := os.MkdirTemp(os.TempDir(), "*")
	require.NoError(t, err)

	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepo(db)
	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	t.Run("use public", func(t *testing.T) {
		pubCfg := &config.BlockStoreConfig{
			Type: "local",
			Local: (*struct {
				Path                    string   `mapstructure:"path"`
				ImportEnabled           bool     `mapstructure:"import_enabled"`
				ImportHidden            bool     `mapstructure:"import_hidden"`
				AllowedExternalPrefixes []string `mapstructure:"allowed_external_prefixes"`
			})(&struct {
				Path                    string
				ImportEnabled           bool
				ImportHidden            bool
				AllowedExternalPrefixes []string
			}{Path: path.Join(tmpDir, "d1"), ImportEnabled: false, ImportHidden: false, AllowedExternalPrefixes: nil}),
		}

		project, err := repo.RepositoryRepo().Insert(ctx, &models.Repository{
			Name:             "testproject",
			Description:      utils.String("test"),
			UsePublicStorage: true,
			HEAD:             "main",
			CreatedAt:        time.Now(),
			UpdatedAt:        time.Now(),
			CreatorID:        user.ID,
			StorageNamespace: "mem://data",
		})
		require.NoError(t, err)
		newRepo, err := NewWorkRepositoryFromConfig(ctx, user, project, repo, pubCfg)
		require.NoError(t, err)
		require.Equal(t, "local", newRepo.adapter.BlockstoreType())
	})

	t.Run("use private", func(t *testing.T) {
		pubCfg := &config.BlockStoreConfig{
			Type: "local",
			Local: (*struct {
				Path                    string   `mapstructure:"path"`
				ImportEnabled           bool     `mapstructure:"import_enabled"`
				ImportHidden            bool     `mapstructure:"import_hidden"`
				AllowedExternalPrefixes []string `mapstructure:"allowed_external_prefixes"`
			})(&struct {
				Path                    string
				ImportEnabled           bool
				ImportHidden            bool
				AllowedExternalPrefixes []string
			}{Path: path.Join(tmpDir, "d1"), ImportEnabled: false, ImportHidden: false, AllowedExternalPrefixes: nil}),
		}
		storageCfg := fmt.Sprintf(`{"Type":"local","Local":{"Path":"%s"}}`, path.Join(tmpDir, "d2"))
		project, err := repo.RepositoryRepo().Insert(ctx, &models.Repository{
			Name:                 "testproject2",
			Description:          utils.String("test"),
			UsePublicStorage:     false,
			HEAD:                 "main",
			CreatedAt:            time.Now(),
			UpdatedAt:            time.Now(),
			CreatorID:            user.ID,
			StorageAdapterParams: storageCfg,
		})
		require.NoError(t, err)
		newRepo, err := NewWorkRepositoryFromConfig(ctx, user, project, repo, pubCfg)
		require.NoError(t, err)
		require.Equal(t, "local", newRepo.adapter.BlockstoreType())
	})

	t.Run("false storage format", func(t *testing.T) {
		pubCfg := &config.BlockStoreConfig{
			Type: "local",
			Local: (*struct {
				Path                    string   `mapstructure:"path"`
				ImportEnabled           bool     `mapstructure:"import_enabled"`
				ImportHidden            bool     `mapstructure:"import_hidden"`
				AllowedExternalPrefixes []string `mapstructure:"allowed_external_prefixes"`
			})(&struct {
				Path                    string
				ImportEnabled           bool
				ImportHidden            bool
				AllowedExternalPrefixes []string
			}{Path: path.Join(tmpDir, "d1"), ImportEnabled: false, ImportHidden: false, AllowedExternalPrefixes: nil}),
		}
		storageCfg := fmt.Sprintf(`{"Type":"local",Local":{"Path":"%s"}}`, path.Join(tmpDir, "d2"))
		project, err := repo.RepositoryRepo().Insert(ctx, &models.Repository{
			Name:                 "testproject3",
			Description:          utils.String("test"),
			UsePublicStorage:     false,
			HEAD:                 "main",
			CreatedAt:            time.Now(),
			UpdatedAt:            time.Now(),
			CreatorID:            user.ID,
			StorageAdapterParams: storageCfg,
		})
		require.NoError(t, err)
		_, err = NewWorkRepositoryFromConfig(ctx, user, project, repo, pubCfg)
		require.Error(t, err)
	})
}

func TestWorkRepository_RootTree(t *testing.T) {
	ctx := context.Background()

	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepo(db)
	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), user, "testproject")
	require.NoError(t, err)

	mainBranch, err := makeBranch(ctx, repo.BranchRepo(), user, "main", project.ID, hash.EmptyHash)
	require.NoError(t, err)

	workRepo := NewWorkRepositoryFromAdapter(ctx, user, project, repo, mem.New(ctx))
	_, err = workRepo.RootTree(ctx)
	require.NoError(t, err)

	testData := `
1|a.txt	|h1
1|b/c.txt	|h2
`
	oriRoot, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), EmptyDirEntry, testData)
	require.NoError(t, err)
	mainWip, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, mainBranch.ID, hash.Hash{}, oriRoot.Hash)
	require.NoError(t, err)

	require.Error(t, workRepo.CheckOut(ctx, WorkRepoState("mock"), "main"))

	require.NoError(t, workRepo.CheckOut(ctx, InWip, "main"))
	require.Equal(t, mainWip.ID, workRepo.CurWip().ID)
	require.Equal(t, "main", workRepo.CurBranch().Name)
	commit, err := workRepo.CommitChanges(ctx, "init commit")
	require.NoError(t, err)

	workTree, err := workRepo.RootTree(ctx)
	require.NoError(t, err)
	require.Equal(t, commit.TreeHash.Hex(), hex.EncodeToString(workTree.Root().Hash()))

	workRepo.Reset()
	workTree, err = workRepo.RootTree(ctx)
	require.NoError(t, err)
	require.Equal(t, commit.TreeHash.Hex(), hex.EncodeToString(workTree.Root().Hash()))
	require.Equal(t, "main", workRepo.CurBranch().Name)
}

func makeUser(ctx context.Context, userRepo models.IUserRepo, name string) (*models.User, error) {
	user := &models.User{
		Name:              name,
		Email:             "xxx@gg.com",
		EncryptedPassword: "123",
		CurrentSignInAt:   time.Time{},
		LastSignInAt:      time.Time{},
		CurrentSignInIP:   "",
		LastSignInIP:      "",
		CreatedAt:         time.Time{},
		UpdatedAt:         time.Time{},
	}
	return userRepo.Insert(ctx, user)
}

func makeRepository(ctx context.Context, repoRepo models.IRepositoryRepo, user *models.User, name string) (*models.Repository, error) {
	return repoRepo.Insert(ctx, &models.Repository{
		Name:             name,
		Description:      utils.String("test"),
		HEAD:             "main",
		CreatedAt:        time.Now(),
		UpdatedAt:        time.Now(),
		CreatorID:        user.ID,
		StorageNamespace: "mem://data",
	})
}

// nolint
func makeCommit(ctx context.Context, commitRepo models.ICommitRepo, treeHash hash.Hash, msg string, parentsHash ...hash.Hash) (*models.Commit, error) {
	commit := &models.Commit{
		RepositoryID: commitRepo.RepositoryID(),
		Author: models.Signature{
			Name:  "admin",
			Email: "xxx@gg.com",
			When:  time.Now(),
		},
		Committer: models.Signature{
			Name:  "admin",
			Email: "xxx@gg.com",
			When:  time.Now(),
		},
		TreeHash:     treeHash,
		ParentHashes: parentsHash,
		Message:      msg,
	}
	hash, err := commit.GetHash()
	if err != nil {
		return nil, err
	}
	commit.Hash = hash
	obj, err := commitRepo.Insert(ctx, commit)
	if err != nil {
		return nil, err
	}
	return obj, nil
}

func makeBranch(ctx context.Context, branchRepo models.IBranchRepo, user *models.User, name string, repoID uuid.UUID, commitHash hash.Hash) (*models.Branches, error) {
	branch := &models.Branches{
		RepositoryID: repoID,
		CommitHash:   commitHash,
		Name:         name,
		Description:  utils.String("test"),
		CreatorID:    user.ID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	return branchRepo.Insert(ctx, branch)
}

func makeWip(ctx context.Context, wipRepo models.IWipRepo, creatorID, repoID, branchID uuid.UUID, parentHash, curHash hash.Hash) (*models.WorkingInProcess, error) {
	wip := &models.WorkingInProcess{
		CurrentTree:  curHash,
		BaseCommit:   parentHash,
		RefID:        branchID,
		RepositoryID: repoID,
		CreatorID:    creatorID,
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}
	return wipRepo.Insert(ctx, wip)
}

func rmWip(ctx context.Context, wipRepo models.IWipRepo, wipID uuid.UUID) error {
	_, err := wipRepo.Delete(ctx, models.NewDeleteWipParams().SetID(wipID))
	return err
}

func makeRoot(ctx context.Context, objRepo models.IFileTreeRepo, treeEntry models.TreeEntry, testData string) (*models.TreeNode, error) {
	lines := strings.Split(testData, "\n")
	treeOp, err := NewWorkTree(ctx, objRepo, treeEntry)
	if err != nil {
		return nil, err
	}
	for _, line := range lines {
		if len(strings.TrimSpace(line)) == 0 {
			continue
		}
		commitData := strings.Split(strings.TrimSpace(line), "|")
		fullPath := strings.TrimSpace(commitData[1])
		fileHash := strings.TrimSpace(commitData[2])
		blob := &models.Blob{
			Hash:         hash.Hash(fileHash),
			RepositoryID: objRepo.RepositoryID(),
			Type:         models.BlobObject,
			Size:         10,
			Properties:   models.Property{Mode: filemode.Regular},
			CreatedAt:    time.Now(),
			UpdatedAt:    time.Now(),
		}

		if commitData[0] == "1" {
			err = treeOp.AddLeaf(ctx, fullPath, blob)
			if err != nil {
				return nil, err
			}
		} else if commitData[0] == "3" {
			err = treeOp.ReplaceLeaf(ctx, fullPath, blob)
			if err != nil {
				return nil, err
			}
		} else {
			//2
			err = treeOp.RemoveEntry(ctx, fullPath)
			if err != nil {
				return nil, err
			}
		}

	}
	return treeOp.Root().TreeNode(), nil
}
