package versionmgr

import (
	"context"
	"testing"

	"github.com/jiaozifs/jiaozifs/block/mem"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/jiaozifs/jiaozifs/versionmgr/merkletrie"
	"github.com/stretchr/testify/require"
)

func TestWorkRepositoryDiffCommit(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepo(db)
	adapter := mem.New(ctx)

	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), user, "testproject")
	require.NoError(t, err)
	//commit1  a.txt b/c.txt  b/e.txt
	//commit2  a.txt b/d.txt  b/e.txt
	testData1 := `
1|a.txt	|a
1|b/c.txt	|c
1|b/e.txt |e1
`

	workRepo := NewWorkRepositoryFromAdapter(ctx, user, project, repo, adapter)
	//base branch

	err = workRepo.CheckOut(ctx, InCommit, hash.EmptyHash.Hex())
	require.NoError(t, err)
	baseBranch, err := workRepo.CreateBranch(ctx, "feat/base")
	require.NoError(t, err)

	root1, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), EmptyDirEntry, testData1)
	require.NoError(t, err)
	baseWip, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, baseBranch.ID, EmptyRoot.Hash, root1.Hash)
	require.NoError(t, err)

	_, err = workRepo.CommitChanges(ctx, "base commit") //asset not correct state
	require.Error(t, err)
	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/base"))

	baseCommit, err := workRepo.CommitChanges(ctx, "base commit")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), baseWip.ID))

	testData2 := `
3|a.txt	|a1
2|b/c.txt	|d
3|b/e.txt |e2
1|b/g.txt |g1
`
	diffBranch, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/diff", project.ID, hash.EmptyHash)
	require.NoError(t, err)

	root2, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(root1.Hash), testData2)
	require.NoError(t, err)
	secondWip, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, diffBranch.ID, EmptyRoot.Hash, root2.Hash)
	require.NoError(t, err)

	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/diff"))
	secondCommit, err := workRepo.CommitChanges(ctx, "merge commit")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), secondWip.ID))

	require.NoError(t, workRepo.CheckOut(ctx, InCommit, baseCommit.Hash.Hex()))
	changes, err := workRepo.DiffCommit(ctx, secondCommit.Hash)
	require.NoError(t, err)
	require.Equal(t, 4, changes.Num())
	require.Equal(t, "a.txt", changes.Index(0).Path())
	action, err := changes.Index(0).Action()
	require.NoError(t, err)
	require.Equal(t, merkletrie.Modify, action)

	require.Equal(t, "b/c.txt", changes.Index(1).Path())
	action, err = changes.Index(1).Action()
	require.NoError(t, err)
	require.Equal(t, merkletrie.Delete, action)
	require.Equal(t, "b/e.txt", changes.Index(2).Path())
	action, err = changes.Index(2).Action()
	require.NoError(t, err)
	require.Equal(t, merkletrie.Modify, action)

	require.Equal(t, "b/g.txt", changes.Index(3).Path())
	action, err = changes.Index(3).Action()
	require.NoError(t, err)
	require.Equal(t, merkletrie.Insert, action)
}
