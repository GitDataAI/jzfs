package versionmgr

import (
	"context"
	"testing"

	"github.com/jiaozifs/jiaozifs/block/mem"
	"github.com/jiaozifs/jiaozifs/models"
	"github.com/jiaozifs/jiaozifs/testhelper"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/stretchr/testify/require"
)

//  TestCommitOpMerge
//example
//       A -----C
//       |      | \
//       |      |  \
//		/ \    F    \
//    /    \  /      \
//  root    AB       CG
//    \    /  \     /
//     \ /     \   /
//      B-D-E--- G

func TestCommitOpMerge(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	adapter := mem.New(ctx)
	repo := models.NewRepo(db)

	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), user, "testproject")
	require.NoError(t, err)

	testData := `
1|a.txt	|h1
1|b/c.txt	|h2
`
	oriRoot, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), EmptyDirEntry, testData)
	require.NoError(t, err)
	//base branch
	baseBranch, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/base", project.ID, hash.EmptyHash)
	require.NoError(t, err)

	oriWip, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, baseBranch.ID, hash.Hash{}, oriRoot.Hash)
	require.NoError(t, err)

	workRepo := NewWorkRepositoryFromAdapter(ctx, user, project, repo, adapter)

	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/base"))
	oriCommit, err := workRepo.CommitChanges(ctx, "")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), oriWip.ID))
	//modify a.txt
	//---------------CommitA
	testData = `
3|a.txt	|h5 
3|b/c.txt	|h2
`
	branchA, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/branchA", project.ID, oriCommit.Hash)
	require.NoError(t, err)

	baseModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriCommit.TreeHash), testData)
	require.NoError(t, err)

	baseWip, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, branchA.ID, oriCommit.Hash, baseModify.Hash)
	require.NoError(t, err)

	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/branchA"))
	commitA, err := workRepo.CommitChanges(ctx, "commit a")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), baseWip.ID))

	//modify a.txt
	//---------------CommitB
	testData = `
3|a.txt	|h4
3|b/c.txt	|h2
`
	branchB, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/branchB", project.ID, oriCommit.Hash)
	require.NoError(t, err)
	mergeModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriCommit.TreeHash), testData)
	require.NoError(t, err)
	mergeWip, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, branchB.ID, oriCommit.Hash, mergeModify.Hash)
	require.NoError(t, err)

	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/branchB"))
	commitB, err := workRepo.CommitChanges(ctx, "commit b")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWip.ID))

	//--------------CommitAB
	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchA"))
	commitAB, err := workRepo.Merge(ctx, user, commitB.Hash, "commit ab", LeastHashResolve)
	require.NoError(t, err)

	//--------------CommitF
	testData = `
1|x.txt	|h4
`
	branchF, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/branchF", project.ID, commitAB.Hash)
	require.NoError(t, err)
	rootF, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(commitAB.TreeHash), testData)
	require.NoError(t, err)
	mergeWipF, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, branchF.ID, commitAB.Hash, rootF.Hash)
	require.NoError(t, err)
	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/branchF"))
	_, err = workRepo.CommitChanges(ctx, "commit f")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWipF.ID))

	//commitC
	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchF"))
	commitC, err := workRepo.Merge(ctx, user, commitA.Hash, "commit c", LeastHashResolve)
	require.NoError(t, err)

	//commitD
	testData = `
3|a.txt	|h5
3|b/c.txt	|h6
1|g/c.txt	|h7
`
	branchDE, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/branchD_E", project.ID, commitB.Hash)
	require.NoError(t, err)
	modifyD, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(commitB.TreeHash), testData)
	require.NoError(t, err)
	mergeWipD, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, branchDE.ID, commitB.Hash, modifyD.Hash)
	require.NoError(t, err)
	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/branchD_E"))
	commitD, err := workRepo.CommitChanges(ctx, "commit d")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWipD.ID))

	//commitE
	testData = `
2|a.txt	|h4
`
	modifyE, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(commitD.TreeHash), testData)
	require.NoError(t, err)
	mergeWipE, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, branchDE.ID, commitD.Hash, modifyE.Hash)
	require.NoError(t, err)
	//require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/branchD_E"))
	commitE, err := workRepo.CommitChanges(ctx, "commit e")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWipE.ID))

	//test fast-ward
	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchB"))
	commitBE, err := workRepo.Merge(ctx, user, commitE.Hash, "commit ab", LeastHashResolve)
	require.NoError(t, err)
	require.Equal(t, commitE.Hash.Hex(), commitBE.Hash.Hex())

	//commitG
	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchD_E"))
	commitG, err := workRepo.Merge(ctx, user, commitAB.Hash, "commit g", LeastHashResolve)
	require.NoError(t, err)

	_, err = makeBranch(ctx, repo.BranchRepo(), user, "feat/branchG", project.ID, commitG.Hash)
	require.NoError(t, err)

	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchG"))
	_, err = workRepo.Merge(ctx, user, commitC.Hash, "commit cg", LeastHashResolve)
	require.NoError(t, err)
}

//	TestCrissCrossMerge
//
// example
//
//	          C--------D
//	        /   \    /  \
//		   /     \ /     \
//		 root     *AB     CG
//		   \    /  \    /
//		    \ /     \ /
//		     B-------G
func TestCrissCrossMerge(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepo(db)
	adapter := mem.New(ctx)

	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), user, "testproject")
	require.NoError(t, err)
	workRepo := NewWorkRepositoryFromAdapter(ctx, user, project, repo, adapter)

	testData := `
1|a.txt	|h1
1|b.txt	|h2
`
	oriRoot, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), EmptyDirEntry, testData)
	require.NoError(t, err)
	//base branch
	baseBranch, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/base", project.ID, hash.EmptyHash)
	require.NoError(t, err)

	oriWip, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, baseBranch.ID, hash.Hash{}, oriRoot.Hash)
	require.NoError(t, err)
	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/base"))
	oriCommit, err := workRepo.CommitChanges(ctx, "base commit")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), oriWip.ID))

	//------------------CommitC
	testData = `
3|a.txt	|h1 
3|b.txt	|h3
`
	branchC, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/branchC", project.ID, oriCommit.Hash)
	require.NoError(t, err)
	baseModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriCommit.TreeHash), testData)
	require.NoError(t, err)
	wipC, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, branchC.ID, oriCommit.Hash, baseModify.Hash)
	require.NoError(t, err)

	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/branchC"))
	commitC, err := workRepo.CommitChanges(ctx, "commit c")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), wipC.ID))
	//modify a.txt
	//-----------------CommitB
	testData = `
3|a.txt	|h4 
3|b.txt	|h2
`
	branchB, err := makeBranch(ctx, repo.BranchRepo(), user, "feat/branchB", project.ID, oriCommit.Hash)
	require.NoError(t, err)
	mergeModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriCommit.TreeHash), testData)
	require.NoError(t, err)
	wipB, err := makeWip(ctx, repo.WipRepo(), user.ID, project.ID, branchB.ID, oriCommit.Hash, mergeModify.Hash)
	require.NoError(t, err)
	require.NoError(t, workRepo.CheckOut(ctx, InWip, "feat/branchB"))
	commitB, err := workRepo.CommitChanges(ctx, "commit b")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), wipB.ID))

	//-----------------CommitAB
	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchB"))
	commiyBC, err := workRepo.Merge(ctx, user, commitC.Hash, "commit bc", LeastHashResolve)
	require.NoError(t, err)

	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchC"))
	commitCB, err := workRepo.Merge(ctx, user, commitB.Hash, "commit cb", LeastHashResolve)
	require.NoError(t, err)

	_, err = makeBranch(ctx, repo.BranchRepo(), user, "feat/branchBC", project.ID, commiyBC.Hash)
	require.NoError(t, err)

	require.NoError(t, workRepo.CheckOut(ctx, InBranch, "feat/branchBC"))
	_, err = workRepo.Merge(ctx, user, commitCB.Hash, "cross commit", LeastHashResolve)
	require.NoError(t, err)
}
