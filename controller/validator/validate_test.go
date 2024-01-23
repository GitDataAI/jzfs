package validator

import (
	"testing"
)

func TestValidateBranchName(t *testing.T) {
	//Validate branch names
	validBranchNames := []string{"main", "feat/branch", "fix/bugfix"}
	for _, name := range validBranchNames {
		err := ValidateBranchName(name)
		if err != nil {
			t.Errorf("Expected no error for branch name '%s', but got: %s", name, err)
		}
	}

	//Invalidate branch names
	invalidBranchNames := []struct {
		name  string
		error string
	}{
		{"repository", "repository name is black list"},
		{"wip", "repository name is black list"},
		{"too_long_branch_name_that_exceeds_max_length_limit", "name too long"},
		{"invalid/name\x00", "invalid branch name: must start with a number or letter and can only contain numbers, letters, hyphens or underscores"},
		{"invalid/branch/name", "branch format must be <name> or <name>/<name>"},
	}

	for _, testCase := range invalidBranchNames {
		err := ValidateBranchName(testCase.name)
		if err == nil || err.Error() != testCase.error {
			t.Errorf("Expected error '%s' for invalid branch name '%s', but got: %v", testCase.error, testCase.name, err)
		}
	}
}

func TestValidateRepoName(t *testing.T) {
	//Validate Repo names
	validRepoNames := []string{"myrepo", "user123", "project-name", "repo123_name"}
	for _, name := range validRepoNames {
		err := ValidateRepoName(name)
		if err != nil {
			t.Errorf("Expected no error for repo name '%s', but got: %s", name, err)
		}
	}

	//Invalidate Repo names
	invalidRepoNames := []struct {
		name  string
		error string
	}{
		{"repository", "repository name is black list"},
		{"wip", "repository name is black list"},
		{"invalid/name", "repository name must start with a number or letter, can only contain numbers, letters, or hyphens, and must be between 3 and 63 characters in length"},
	}

	for _, testCase := range invalidRepoNames {
		err := ValidateRepoName(testCase.name)
		if err == nil || err.Error() != testCase.error {
			t.Errorf("Expected error '%s' for invalid repo name '%s', but got: %v", testCase.error, testCase.name, err)
		}
	}
}

func TestValidateUsername(t *testing.T) {
	//Validate Username
	validUsernames := []string{"user123", "username", "user_name", "user-123"}
	for _, name := range validUsernames {
		err := ValidateUsername(name)
		if err != nil {
			t.Errorf("Expected no error for username '%s', but got: %s", name, err)
		}
	}

	//Invalidate Username
	invalidUsernames := []struct {
		name  string
		error string
	}{
		{"user name", "invalid username: it must start and end with a letter or digit, can contain letters, digits, hyphens, and cannot start or end with a hyphen; the length must be between 3 and 30 characters"},
		{"user-with-hyphen-", "invalid username: it must start and end with a letter or digit, can contain letters, digits, hyphens, and cannot start or end with a hyphen; the length must be between 3 and 30 characters"},
		{"invalid/username", "invalid username: it must start and end with a letter or digit, can contain letters, digits, hyphens, and cannot start or end with a hyphen; the length must be between 3 and 30 characters"},
	}

	for _, testCase := range invalidUsernames {
		err := ValidateUsername(testCase.name)
		if err == nil || err.Error() != testCase.error {
			t.Errorf("Expected error '%s' for invalid username '%s', but got: %v", testCase.error, testCase.name, err)
		}
	}
}

func TestValidateObjectPath(t *testing.T) {
	//Validate Obj Path
	validObjectPaths := []string{"path/to/object", "file.txt", "folder/file.txt", "我的图片.png", "我的文件/我的应用.exe"}
	for _, path := range validObjectPaths {
		err := ValidateObjectPath(path)
		if err != nil {
			t.Errorf("Expected no error for object path '%s', but got: %s", path, err)
		}
	}

	//Invalidate Obj Path
	invalidObjectPaths := []struct {
		path  string
		error string
	}{
		{"path/with/null\x00character", "invalid object path: it must not contain null characters or NTFS forbidden characters"},
		{"path/with/invalid/characters/:", "invalid object path: it must not contain null characters or NTFS forbidden characters"},
		{"path/with/invalid/characters/*", "invalid object path: it must not contain null characters or NTFS forbidden characters"},
		{"path/with/invalid/characters/\"", "invalid object path: it must not contain null characters or NTFS forbidden characters"},
		{"path/with/invalid/characters/<?", "invalid object path: it must not contain null characters or NTFS forbidden characters"},
	}

	for _, testCase := range invalidObjectPaths {
		err := ValidateObjectPath(testCase.path)
		if err == nil || err.Error() != testCase.error {
			t.Errorf("Expected error '%s' for invalid object path '%s', but got: %v", testCase.error, testCase.path, err)
		}
	}
}
