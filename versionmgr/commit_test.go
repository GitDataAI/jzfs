package versionmgr

import (
	"context"
	"strings"
	"testing"
	"time"

	"github.com/google/uuid"

	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/testhelper"
)

func TestCommitOp_DiffCommit(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepo(db)
	//commit1  a.txt b/c.txt  b/e.txt
	//commit2  a.txt b/d.txt  b/e.txt
	testData1 := `
a.txt	|a
b/c.txt	|c
b/e.txt |e1
`
	testData2 := `
a.txt	|a
b/d.txt	|d
b/e.txt |e2
`
	root1, err := makeRoot(ctx, repo.ObjectRepo(), testData1)
	require.NoError(t, err)
	root2, err := makeRoot(ctx, repo.ObjectRepo(), testData2)
	require.NoError(t, err)

	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), "testproject")
	require.NoError(t, err)

	//base branch
	baseRef, err := makeRef(ctx, repo.RefRepo(), "feat/base", project.ID, hash.Hash("a"))
	require.NoError(t, err)
	baseWip, err := makeWip(ctx, repo.WipRepo(), project.ID, baseRef.ID, root1.Hash)
	require.NoError(t, err)

	baseCommit, err := NewCommitOp(repo, nil).AddCommit(ctx, user.ID, baseWip.ID, "base commit")
	require.NoError(t, err)

	//toMerge branch
	mergeRef, err := makeRef(ctx, repo.RefRepo(), "feat/merge", project.ID, hash.Hash("a"))
	require.NoError(t, err)
	mergeWip, err := makeWip(ctx, repo.WipRepo(), project.ID, mergeRef.ID, root2.Hash)
	require.NoError(t, err)
	mergeCommit, err := NewCommitOp(repo, nil).AddCommit(ctx, user.ID, mergeWip.ID, "merge commit")
	require.NoError(t, err)

	changes, err := baseCommit.DiffCommit(ctx, mergeCommit.Commit().Hash)
	require.NoError(t, err)
	require.Len(t, changes.Num(), 3)
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

func makeRepository(ctx context.Context, repoRepo models.IRepositoryRepo, name string) (*models.Repository, error) {
	user := &models.Repository{
		Name:        name,
		Description: "",
		HEAD:        "main",
		CreateID:    uuid.UUID{},
		CreatedAt:   time.Time{},
		UpdatedAt:   time.Time{},
	}
	return repoRepo.Insert(ctx, user)
}

func makeCommit(ctx context.Context, commitRepo models.IObjectRepo, treeHash hash.Hash, msg string, parentsHash ...hash.Hash) (*models.Commit, error) {
	commit := &models.Commit{
		Hash: hash.Hash("mock"),
		Type: models.CommitObject,
		Author: models.Signature{
			Name:  "admin",
			Email: "xxx@gg.com",
			When:  time.Time{},
		},
		Committer: models.Signature{
			Name:  "admin",
			Email: "xxx@gg.com",
			When:  time.Time{},
		},
		TreeHash:     treeHash,
		ParentHashes: parentsHash,
		Message:      msg,
	}
	obj, err := commitRepo.Insert(ctx, commit.Object())
	if err != nil {
		return nil, err
	}
	return obj.Commit(), nil
}
func makeRef(ctx context.Context, refRepo models.IRefRepo, name string, repoID uuid.UUID, commitHash hash.Hash) (*models.Ref, error) {
	ref := &models.Ref{
		RepositoryID: repoID,
		CommitHash:   commitHash,
		Name:         name,
		Description:  "",
		CreateID:     uuid.UUID{},
		CreatedAt:    time.Time{},
		UpdatedAt:    time.Time{},
	}
	return refRepo.Insert(ctx, ref)
}

func makeWip(ctx context.Context, wipRepo models.IWipRepo, repoID, refID uuid.UUID, curHash hash.Hash) (*models.WorkingInProcess, error) {
	wip := &models.WorkingInProcess{
		CurrentTree:  curHash,
		ParentTree:   hash.Hash("mock"),
		RefID:        refID,
		RepositoryID: repoID,
		CreateID:     uuid.UUID{},
		CreatedAt:    time.Time{},
		UpdatedAt:    time.Time{},
	}
	return wipRepo.Insert(ctx, wip)
}
func makeRoot(ctx context.Context, objRepo models.IObjectRepo, testData string) (*models.TreeNode, error) {
	lines := strings.Split(testData, "\n")
	treeOp, err := NewWorkTree(ctx, objRepo, EmptyDirEntry)
	if err != nil {
		return nil, err
	}
	for _, line := range lines {
		if len(strings.TrimSpace(line)) == 0 {
			continue
		}
		commitData := strings.Split(strings.TrimSpace(line), "|")
		fullPath := strings.TrimSpace(commitData[0])
		fileHash := strings.TrimSpace(commitData[1])
		blob := &models.Blob{
			Hash:      hash.Hash(fileHash),
			Type:      models.BlobObject,
			Size:      10,
			CreatedAt: time.Now(),
			UpdatedAt: time.Now(),
		}

		err = treeOp.AddLeaf(ctx, fullPath, blob)
		if err != nil {
			return nil, err
		}
	}
	return treeOp.Root().TreeNode(), nil
}
