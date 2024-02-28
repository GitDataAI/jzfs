package rbacModel

import (
	"fmt"
	"strings"
)

type ResourceType string

const (
	RepoRT ResourceType = "repo"
	UserRT ResourceType = "user"
	AuthRT ResourceType = "auth"
)

type Resource string

const (
	// this is arn with s3, maybe we dont need this https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html
	repoArnPrefix Resource = "arn:gitdata:jiaozifs:::"
	authArnPrefix Resource = "arn:gitdata:jiaozifs:::"
	userArnPrefix Resource = "arn:gitdata:jiaozifs:::"
	All           Resource = "*"

	UserIDCapture = "{user_id}"
	RepoIDCapture = "{repo_id}"
)

func (r Resource) WithRepoID(repoID string) Resource {
	return Resource(strings.ReplaceAll(string(r), RepoIDCapture, repoID))
}

func (r Resource) WithUserID(userID string) Resource {
	return Resource(strings.ReplaceAll(string(r), UserIDCapture, userID))
}

func (r Resource) String() string {
	return string(r)
}
func RepoURArn(userID string, repoID string) Resource {
	return Resource(fmt.Sprintf("%srepository/%s/%s", repoArnPrefix, userID, repoID))
}

// RepoUArn anything in this repo
func RepoUArn(userID string) Resource {
	return Resource(fmt.Sprintf("%srepository/%s/*", repoArnPrefix, userID))
}

func UserArn(userID string) Resource {
	return Resource(fmt.Sprintf("%suser/%s", userArnPrefix, userID))
}

func UserAkskArn(userID string) Resource {
	return Resource(fmt.Sprintf("%suser/aksk/%s", userArnPrefix, userID))
}

func GroupArn(groupID string) Resource {
	return Resource(fmt.Sprintf("%sgroup/%s", authArnPrefix, groupID))
}

func PolicyArn(policyID string) Resource {
	return Resource(fmt.Sprintf("%spolicy/%s", authArnPrefix, policyID))
}
