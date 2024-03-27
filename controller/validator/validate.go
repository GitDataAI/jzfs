package validator

import (
	"errors"
	"regexp"
	"strings"
)

var (
	MaxBranchNameLength = 40

	ReValidRef  = regexp.MustCompile(`^\w+/?\w+$`)
	ReValidRepo = regexp.MustCompile(`^[a-zA-Z0-9][a-zA-Z0-9_\-]{1,61}[a-zA-Z0-9]$`)
	ReValidTag  = regexp.MustCompile(`^[a-zA-Z0-9][a-zA-Z0-9_.\-]{1,61}[a-zA-Z0-9]$`)
	ReValidUser = regexp.MustCompile(`^[a-zA-Z0-9][a-zA-Z0-9_-]{1,28}[a-zA-Z0-9]$`)
	ReValidPath = regexp.MustCompile(`^[^\x00/:*?"<>|]*/?([^/\s\x00:*?"<>|]+/)*[^/\s\x00:*?"<>|]+(?:\.[a-zA-Z0-9]+)?$`)

	// RepoNameBlackList forbid repo name, reserve for routes
	RepoNameBlackList = []string{"repository", "repositories", "wip", "wips", "object", "objects", "tags", "tag", "commit", "commits", "ref", "refs", "repo", "repos", "user", "users"}
)

var (
	ErrNameBlackList     = errors.New("repository name is black list")
	ErrNameTooLong       = errors.New("name too long")
	ErrBranchFormat      = errors.New("branch format must be <name> or <name>/<name>")
	ErrInvalidBranchName = errors.New("invalid branch name: must start with a number or letter and can only contain numbers, letters, hyphens or underscores")
	ErrInvalidRepoName   = errors.New("repository name must start with a number or letter, can only contain numbers, letters, or hyphens, and must be between 3 and 63 characters in length")
	ErrInvalidTagName    = errors.New("tag name must start with a number or letter, can only contain numbers, letters, dot, or hyphens, and must be between 3 and 63 characters in length")
	ErrInvalidUsername   = errors.New("invalid username: it must start and end with a letter or digit, can contain letters, digits, hyphens, and cannot start or end with a hyphen; the length must be between 3 and 30 characters")
	ErrInvalidObjectPath = errors.New("invalid object path: it must not contain null characters or NTFS forbidden characters")
)

func ValidateBranchName(name string) error {
	for _, blackName := range RepoNameBlackList {
		if name == blackName {
			return ErrNameBlackList
		}
	}

	if len(name) > MaxBranchNameLength {
		return ErrNameTooLong
	}

	seg := strings.Split(name, "/")
	if len(seg) > 2 {
		return ErrBranchFormat
	}

	if !ReValidRef.Match([]byte(seg[0])) {
		return ErrInvalidBranchName
	}
	if len(seg) > 1 {
		if !ReValidRef.Match([]byte(seg[1])) {
			return ErrInvalidBranchName
		}
	}
	return nil
}

func ValidateRepoName(name string) error {
	for _, blackName := range RepoNameBlackList {
		if name == blackName {
			return ErrNameBlackList
		}
	}

	if !ReValidRepo.MatchString(name) {
		return ErrInvalidRepoName
	}
	return nil
}

func ValidateTagName(name string) error {
	if !ReValidTag.MatchString(name) {
		return ErrInvalidTagName
	}
	return nil
}

func ValidateUsername(name string) error {
	if !ReValidUser.MatchString(name) {
		return ErrInvalidUsername
	}
	return nil
}

func ValidateObjectPath(path string) error {
	if !ReValidPath.MatchString(path) {
		return ErrInvalidObjectPath
	}
	return nil
}
