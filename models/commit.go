package models

import (
	"context"
	"time"

	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/uptrace/bun"
)

// Signature is used to identify who and when created a commit or tag.
type Signature struct {
	// Name represents a person name. It is an arbitrary string.
	Name string `bun:"name"`
	// Email is an email, but it cannot be assumed to be well-formed.
	Email string `bun:"email"`
	// When is the timestamp of the signature.
	When time.Time `bun:"when"`
}

type Commit struct {
	bun.BaseModel `bun:"table:commits"`
	Hash          hash.Hash `bun:"hash,pk,type:bytea"`
	//////********commit********////////
	// Author is the original author of the commit.
	Author Signature `bun:"author,type:jsonb"`
	// Committer is the one performing the commit, might be different from
	// Author.
	Committer Signature `bun:"committer,type:jsonb"`
	// MergeTag is the embedded tag object when a merge commit is created by
	// merging a signed tag.
	MergeTag string `bun:"merge_tag"` //todo
	// Message is the commit/tag message, contains arbitrary text.
	Message string `bun:"message"`
	// TreeHash is the hash of the root tree of the commit.
	TreeHash hash.Hash `bun:"tree_hash,type:bytea,notnull"`
	// ParentHashes are the hashes of the parent commits of the commit.
	ParentHashes []hash.Hash `bun:"parent_hashes,type:bytea[]"`

	CreatedAt time.Time `bun:"created_at"`
	UpdatedAt time.Time `bun:"updated_at"`
}

func (commit *Commit) GetHash() (hash.Hash, error) {
	hasher := hash.NewHasher(hash.Md5)
	err := hasher.WriteInt8(int8(CommitObject))
	if err != nil {
		return nil, err
	}
	err = hasher.WriteString(commit.Author.Name)
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Author.Email)
	if err != nil {
		return nil, err
	}

	err = hasher.WritInt64(commit.Author.When.Unix())
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Committer.Name)
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Committer.Email)
	if err != nil {
		return nil, err
	}

	err = hasher.WritInt64(commit.Committer.When.Unix())
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.MergeTag)
	if err != nil {
		return nil, err
	}

	err = hasher.WriteString(commit.Message)
	if err != nil {
		return nil, err
	}

	_, err = hasher.Write(commit.TreeHash)
	if err != nil {
		return nil, err
	}

	for _, h := range commit.ParentHashes {
		_, err = hasher.Write(h)
		if err != nil {
			return nil, err
		}
	}

	return hasher.Md5.Sum(nil), nil
}

func (commit *Commit) NumParents() int {
	return len(commit.ParentHashes)
}

type ICommitRepo interface {
	Commit(ctx context.Context, hash hash.Hash) (*Commit, error)
	Insert(ctx context.Context, commit *Commit) (*Commit, error)
}
type CommitRepo struct {
	db bun.IDB
}

func NewCommitRepo(db bun.IDB) ICommitRepo {
	return &CommitRepo{db}
}

func (cr CommitRepo) Commit(ctx context.Context, hash hash.Hash) (*Commit, error) {
	commit := &Commit{}
	return commit, cr.db.NewSelect().Model(commit).Where("hash = ?", hash).Scan(ctx)
}

func (cr CommitRepo) Insert(ctx context.Context, commit *Commit) (*Commit, error) {
	_, err := cr.db.NewInsert().Model(commit).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return commit, nil
}
