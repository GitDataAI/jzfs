package rbac

import "fmt"

type ResourceType string

const (
	RepoRT ResourceType = "repo"
	UserRT ResourceType = "user"
	AuthRT ResourceType = "auth"
)

type Resource string

const (
	//todo this is arn with s3, maybe we dont need this https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html
	repoArnPrefix Resource = "arn:iaozifs:repo:::"
	authArnPrefix Resource = "arn:jiaozifs:auth:::"
	userArnPrefix Resource = "arn:jiaozifs:user:::"
	All           Resource = "*"
)

func RepoArn(repoID string) Resource {
	return Resource(fmt.Sprintf("%srepository/%s", repoArnPrefix, repoID))
}

func StorageNamespace(namespace string) Resource {
	return Resource(fmt.Sprintf("%samespace/%s", repoArnPrefix, namespace))
}

func ObjectArn(repoID, key string) Resource {
	return Resource(fmt.Sprintf("%sepository/%s/object/%s", repoArnPrefix, repoID, key))
}

func BranchArn(repoID, branchID string) Resource {
	return Resource(fmt.Sprintf("%srepository/%s/branch/%s", repoArnPrefix, repoID, branchID))
}

func UserArn(userID string) Resource {
	return Resource(fmt.Sprintf("%suser/%s", userArnPrefix, userID))
}

func GroupArn(groupID string) Resource {
	return Resource(fmt.Sprintf("%sgroup/%s", authArnPrefix, groupID))
}

func PolicyArn(policyID string) Resource {
	return Resource(fmt.Sprintf("%spolicy/%s", authArnPrefix, policyID))
}
