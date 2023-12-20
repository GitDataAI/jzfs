package versionmgr

import (
	"context"
	"strings"
	"testing"
	"time"

	"github.com/jiaozifs/jiaozifs/models/filemode"

	"github.com/jiaozifs/jiaozifs/utils"

	"github.com/jiaozifs/jiaozifs/versionmgr/merkletrie"

	"github.com/google/uuid"

	"github.com/stretchr/testify/require"

	"github.com/jiaozifs/jiaozifs/utils/hash"

	"github.com/jiaozifs/jiaozifs/models"

	"github.com/jiaozifs/jiaozifs/testhelper"
)

func TestCommitOpDiffCommit(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepo(db)

	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), "testproject")
	require.NoError(t, err)

	//base branch
	baseRef, err := makeRef(ctx, repo.RefRepo(), "feat/base", project.ID, hash.Hash("a"))
	require.NoError(t, err)

	//commit1  a.txt b/c.txt  b/e.txt
	//commit2  a.txt b/d.txt  b/e.txt
	testData1 := `
1|a.txt	|a
1|b/c.txt	|c
1|b/e.txt |e1
`

	root1, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), EmptyDirEntry, testData1)
	require.NoError(t, err)
	baseWip, err := makeWip(ctx, repo.WipRepo(), project.ID, baseRef.ID, EmptyRoot.Hash, root1.Hash)
	require.NoError(t, err)

	baseCommit, err := NewCommitOp(repo, project.ID, nil).AddCommit(ctx, user, baseWip.ID, "base commit")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), baseWip.ID))

	testData2 := `
3|a.txt	|a1
2|b/c.txt	|d
3|b/e.txt |e2
1|b/g.txt |g1
`
	root2, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(root1.Hash), testData2)
	require.NoError(t, err)

	secondWip, err := makeWip(ctx, repo.WipRepo(), project.ID, baseRef.ID, EmptyRoot.Hash, root2.Hash)
	require.NoError(t, err)
	secondCommit, err := NewCommitOp(repo, project.ID, nil).AddCommit(ctx, user, secondWip.ID, "merge commit")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), secondWip.ID))

	changes, err := baseCommit.DiffCommit(ctx, secondCommit.Commit().Hash)
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

	repo := models.NewRepo(db)

	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), "testproject")
	require.NoError(t, err)

	testData := `
1|a.txt	|h1
1|b/c.txt	|h2
`
	oriRoot, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), EmptyDirEntry, testData)
	require.NoError(t, err)
	//base branch
	baseRef, err := makeRef(ctx, repo.RefRepo(), "feat/base", project.ID, hash.Hash("a"))
	require.NoError(t, err)

	oriWip, err := makeWip(ctx, repo.WipRepo(), project.ID, baseRef.ID, hash.Hash{}, oriRoot.Hash)
	require.NoError(t, err)

	oriCommit, err := NewCommitOp(repo, project.ID, nil).AddCommit(ctx, user, oriWip.ID, "")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), oriWip.ID))
	//modify a.txt
	//CommitA
	testData = `
3|a.txt	|h5 
3|b/c.txt	|h2
`
	baseModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriRoot.Hash), testData)
	require.NoError(t, err)

	baseWip, err := makeWip(ctx, repo.WipRepo(), project.ID, baseRef.ID, oriRoot.Hash, baseModify.Hash)
	require.NoError(t, err)

	commitA, err := NewCommitOp(repo, project.ID, oriCommit.Commit()).AddCommit(ctx, user, baseWip.ID, "commit a")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), baseWip.ID))

	//toMerge branch
	mergeRef, err := makeRef(ctx, repo.RefRepo(), "feat/merge", project.ID, hash.Hash("a"))
	require.NoError(t, err)

	//modify a.txt
	//CommitB
	testData = `
3|a.txt	|h4
3|b/c.txt	|h2
`
	mergeModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriRoot.Hash), testData)
	require.NoError(t, err)
	mergeWip, err := makeWip(ctx, repo.WipRepo(), project.ID, mergeRef.ID, oriRoot.Hash, mergeModify.Hash)
	require.NoError(t, err)
	commitB, err := NewCommitOp(repo, project.ID, oriCommit.Commit()).AddCommit(ctx, user, mergeWip.ID, "commit b")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWip.ID))

	//CommitAB
	commitAB, err := commitA.Merge(ctx, user, commitB.Commit().Hash, "commit ab", LeastHashResolve)
	require.NoError(t, err)

	//CommitAS
	testData = `
1|x.txt	|h4
`
	rootF, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(commitAB.TreeHash), testData)
	require.NoError(t, err)
	mergeWipF, err := makeWip(ctx, repo.WipRepo(), project.ID, mergeRef.ID, commitAB.TreeHash, rootF.Hash)
	require.NoError(t, err)
	commitF, err := NewCommitOp(repo, project.ID, oriCommit.Commit()).AddCommit(ctx, user, mergeWipF.ID, "commit f")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWipF.ID))

	//commitC
	commitC, err := commitA.Merge(ctx, user, commitF.Commit().Hash, "commit c", LeastHashResolve)
	require.NoError(t, err)

	//commitD
	testData = `
3|a.txt	|h5
3|b/c.txt	|h6
1|g/c.txt	|h7
`
	modifyD, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(commitB.Commit().TreeHash), testData)
	require.NoError(t, err)
	mergeWipD, err := makeWip(ctx, repo.WipRepo(), project.ID, mergeRef.ID, commitB.Commit().Hash, modifyD.Hash)
	require.NoError(t, err)
	commitD, err := commitB.AddCommit(ctx, user, mergeWipD.ID, "commit d")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWipD.ID))
	//commitE
	testData = `
2|a.txt	|h4
`
	modifyE, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(commitD.Commit().TreeHash), testData)
	require.NoError(t, err)
	mergeWipE, err := makeWip(ctx, repo.WipRepo(), project.ID, mergeRef.ID, commitD.Commit().Hash, modifyE.Hash)
	require.NoError(t, err)
	commitE, err := commitD.AddCommit(ctx, user, mergeWipE.ID, "commit e")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWipE.ID))
	//test fast-ward

	fastMergeCommit, err := commitB.Merge(ctx, user, commitE.Commit().Hash, "", LeastHashResolve)
	require.NoError(t, err)
	require.Equal(t, commitE.Commit().Hash.Hex(), fastMergeCommit.Hash.Hex())

	//commitG
	commitG, err := commitE.Merge(ctx, user, commitAB.Hash, "commit c", LeastHashResolve)
	require.NoError(t, err)

	_, err = NewCommitOp(repo, project.ID, commitC).Merge(ctx, user, commitG.Hash, "commit c", LeastHashResolve)
	require.NoError(t, err)
}

//	TestCrissCrossMerge
//
// example
//
//	          C--------D
//	        /   \    /  \
//		   /     \ /     \
//		 root     *      CG
//		   \    /  \    /
//		    \ /     \ /
//		     B-------G
func TestCrissCrossMerge(t *testing.T) {
	ctx := context.Background()
	postgres, _, db := testhelper.SetupDatabase(ctx, t)
	defer postgres.Stop() //nolint

	repo := models.NewRepo(db)

	user, err := makeUser(ctx, repo.UserRepo(), "admin")
	require.NoError(t, err)

	project, err := makeRepository(ctx, repo.RepositoryRepo(), "testproject")
	require.NoError(t, err)

	testData := `
1|a.txt	|h1
1|b.txt	|h2
`
	oriRoot, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), EmptyDirEntry, testData)
	require.NoError(t, err)
	//base branch
	baseRef, err := makeRef(ctx, repo.RefRepo(), "feat/base", project.ID, hash.Hash("a"))
	require.NoError(t, err)

	oriWip, err := makeWip(ctx, repo.WipRepo(), project.ID, baseRef.ID, hash.Hash{}, oriRoot.Hash)
	require.NoError(t, err)

	oriCommit, err := NewCommitOp(repo, project.ID, nil).AddCommit(ctx, user, oriWip.ID, "")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), oriWip.ID))

	//CommitA
	testData = `
3|a.txt	|h1 
3|b.txt	|h3
`
	baseModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriRoot.Hash), testData)
	require.NoError(t, err)

	baseWip, err := makeWip(ctx, repo.WipRepo(), project.ID, baseRef.ID, oriRoot.Hash, baseModify.Hash)
	require.NoError(t, err)

	commitA, err := NewCommitOp(repo, project.ID, oriCommit.Commit()).AddCommit(ctx, user, baseWip.ID, "commit a")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), baseWip.ID))

	//toMerge branch
	mergeRef, err := makeRef(ctx, repo.RefRepo(), "feat/merge", project.ID, hash.Hash("a"))
	require.NoError(t, err)

	//modify a.txt
	//CommitB
	testData = `
3|a.txt	|h4 
3|b.txt	|h2
`
	mergeModify, err := makeRoot(ctx, repo.FileTreeRepo(project.ID), models.NewRootTreeEntry(oriRoot.Hash), testData)
	require.NoError(t, err)
	mergeWip, err := makeWip(ctx, repo.WipRepo(), project.ID, mergeRef.ID, oriRoot.Hash, mergeModify.Hash)
	require.NoError(t, err)
	commitB, err := NewCommitOp(repo, project.ID, oriCommit.Commit()).AddCommit(ctx, user, mergeWip.ID, "commit b")
	require.NoError(t, err)
	require.NoError(t, rmWip(ctx, repo.WipRepo(), mergeWip.ID))

	commitAB, err := commitA.Merge(ctx, user, commitB.Commit().Hash, "commit ab", LeastHashResolve)
	require.NoError(t, err)

	commitBA, err := commitB.Merge(ctx, user, commitA.Commit().Hash, "commit ba", LeastHashResolve)
	require.NoError(t, err)

	_, err = NewCommitOp(repo, project.ID, commitAB).Merge(ctx, user, commitBA.Hash, "cross commit", LeastHashResolve)
	require.NoError(t, err)
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
		Description: utils.String("test"),
		HEAD:        "main",
		CreatorID:   uuid.UUID{},
		CreatedAt:   time.Time{},
		UpdatedAt:   time.Time{},
	}
	return repoRepo.Insert(ctx, user)
}

// nolint
func makeCommit(ctx context.Context, commitRepo models.ICommitRepo, treeHash hash.Hash, msg string, parentsHash ...hash.Hash) (*models.Commit, error) {
	commit := &models.Commit{
		Hash: hash.Hash("mock"),
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
	obj, err := commitRepo.Insert(ctx, commit)
	if err != nil {
		return nil, err
	}
	return obj, nil
}

func makeRef(ctx context.Context, refRepo models.IRefRepo, name string, repoID uuid.UUID, commitHash hash.Hash) (*models.Ref, error) {
	ref := &models.Ref{
		RepositoryID: repoID,
		CommitHash:   commitHash,
		Name:         name,
		Description:  utils.String("test"),
		CreatorID:    uuid.UUID{},
		CreatedAt:    time.Time{},
		UpdatedAt:    time.Time{},
	}
	return refRepo.Insert(ctx, ref)
}

func makeWip(ctx context.Context, wipRepo models.IWipRepo, repoID, refID uuid.UUID, parentHash, curHash hash.Hash) (*models.WorkingInProcess, error) {
	wip := &models.WorkingInProcess{
		CurrentTree:  curHash,
		BaseCommit:   parentHash,
		RefID:        refID,
		RepositoryID: repoID,
		CreatorID:    uuid.UUID{},
		CreatedAt:    time.Time{},
		UpdatedAt:    time.Time{},
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
