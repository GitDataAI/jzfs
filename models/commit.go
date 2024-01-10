package models

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/jiaozifs/jiaozifs/utils/hash"
	"github.com/uptrace/bun"
)

// Signature is used to identify who and when created a commit or tag.
type Signature struct {
	// Name represents a person name. It is an arbitrary string.
	Name string `bun:"name" json:"name"`
	// Email is an email, but it cannot be assumed to be well-formed.
	Email string `bun:"email" json:"email"`
	// When is the timestamp of the signature.
	When time.Time `bun:"when" json:"when"`
}

type Commit struct {
	bun.BaseModel `bun:"table:commits"`
	Hash          hash.Hash `bun:"hash,pk,type:bytea" json:"hash"`
	RepositoryID  uuid.UUID `bun:"repository_id,pk,type:uuid,notnull" json:"repository_id"`
	//////********commit********////////
	// Author is the original author of the commit.
	Author Signature `bun:"author,notnull,type:jsonb" json:"author"`
	// Committer is the one performing the commit, might be different from
	// Author.
	Committer Signature `bun:"committer,notnull,type:jsonb" json:"committer"`
	// MergeTag is the embedded tag object when a merge commit is created by
	// merging a signed tag.
	MergeTag string `bun:"merge_tag" json:"merge_tag"`
	// Message is the commit/tag message, contains arbitrary text.
	Message string `bun:"message" json:"message"`
	// TreeHash is the hash of the root tree of the commit.
	TreeHash hash.Hash `bun:"tree_hash,type:bytea,notnull" json:"tree_hash"`
	// ParentHashes are the hashes of the parent commits of the commit.
	ParentHashes []hash.Hash `bun:"parent_hashes,type:bytea[]" json:"parent_hashes"`

	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
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

type DeleteParams struct {
	hash hash.Hash
}

func NewDeleteParams() *DeleteParams {
	return &DeleteParams{}
}

func (params *DeleteParams) SetHash(hash hash.Hash) *DeleteParams {
	params.hash = hash
	return params
}

type ICommitRepo interface {
	RepositoryID() uuid.UUID
	Commit(ctx context.Context, hash hash.Hash) (*Commit, error)
	Insert(ctx context.Context, commit *Commit) (*Commit, error)
	Delete(ctx context.Context, params *DeleteParams) (int64, error)
}
type CommitRepo struct {
	db           bun.IDB
	repositoryID uuid.UUID
}

func (cr CommitRepo) RepositoryID() uuid.UUID {
	return cr.repositoryID
}

func NewCommitRepo(db bun.IDB, repoID uuid.UUID) ICommitRepo {
	return &CommitRepo{
		db:           db,
		repositoryID: repoID,
	}
}

func (cr CommitRepo) Commit(ctx context.Context, hash hash.Hash) (*Commit, error) {
	commit := &Commit{}
	err := cr.db.NewSelect().Model(commit).
		Where("repository_id = ?", cr.repositoryID).
		Where("hash = ?", hash).Scan(ctx)
	if err != nil {
		return nil, err
	}
	return commit, nil
}

func (cr CommitRepo) Insert(ctx context.Context, commit *Commit) (*Commit, error) {
	if commit.RepositoryID != cr.repositoryID {
		return nil, ErrRepoIDMisMatch
	}
	_, err := cr.db.NewInsert().Model(commit).Exec(ctx)
	if err != nil {
		return nil, err
	}
	return commit, nil
}

func (cr CommitRepo) Delete(ctx context.Context, params *DeleteParams) (int64, error) {
	query := cr.db.NewDelete().Model((*Commit)(nil)).Where("repository_id = ?", cr.repositoryID)
	if params.hash != nil {
		query = query.Where("hash = ?", params.hash)
	}

	sqlResult, err := query.Exec(ctx)
	if err != nil {
		return 0, err
	}
	affectedRows, err := sqlResult.RowsAffected()
	if err != nil {
		return 0, err
	}
	return affectedRows, err
}
