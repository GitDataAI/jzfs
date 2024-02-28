# rbac设计方案

rbac中引入resource_type, resource, statement, policy, group，user,的概念他们之间的关系如下图

![image](https://github.com/jiaozifs/jiaozifs/assets/41407352/672a6d22-809c-49b0-9e04-7dd32700b551)

## 类型组织
整体上对于权限的组织情况如下
- resource_type: 资源类型
- resource: 资源种类
- statement: 一组预定义的权限集合
- poly: 结合了资源的权限
- group： 用户可直接理解的权限组
- user: 用户

### 资源类型

- repo 用户仓库相关功能
- user 用户相关接口
- auth 授权相关功能

### action

### statement
- FSFullAccess     全访问权限
- RepoRead         仓库读取权限
- RepoReadWrite    仓库读写权限
- RepoReadConfig   仓库读取配置权限
- RepoWriteConfig  仓库写入配置权限
- UserAccess       用户配置自己信息的权限

### policy

- FSFullAccess  全访问权限
- RepoRead      仓库读取权限
- RepoReadWrite 仓库读写权限
- RepoConfig    仓库配置权限
- UserAccess    用户配置自己信息的权限

### group

- SuperUsers 超级用户组
- RepoAdmins 仓库管理权限
- RepoWrite 写入权限
- RepoRead 读取权限


## 表设计

资源类型
```go
const (
	RepoRT ResourceType = "repo"
	UserRT ResourceType = "user"
	AuthRT ResourceType = "auth"
)
```

action 列表
```go
const (
	ReadRepositoryAction    = "repo:ReadRepository"
	CreateRepositoryAction  = "repo:CreateRepository"
	UpdateRepositoryAction  = "repo:UpdateRepository"
	DeleteRepositoryAction  = "repo:DeleteRepository"
	ListRepositoriesAction  = "repo:ListRepositories"
	ReadObjectAction        = "repo:ReadObject"
	WriteObjectAction       = "repo:WriteObject"
	DeleteObjectAction      = "repo:DeleteObject"
	ListObjectsAction       = "repo:ListObjects"
	CreateCommitAction      = "repo:CreateCommit"
	ReadCommitAction        = "repo:ReadCommit"
	ListCommitsAction       = "repo:ListCommits"
	CreateBranchAction      = "repo:CreateBranch"
	DeleteBranchAction      = "repo:DeleteBranch"
	ReadBranchAction        = "repo:ReadBranch"
	ListBranchesAction      = "repo:ListBranches"
	ReadConfigAction        = "repo:ReadConfig"
	UpdateConfigAction      = "repo:UpdateConfig"
	AddGroupMemberAction    = "repo:AddGroupMember"
	RemoveGroupMemberAction = "repo:RemoveGroupMember"

	ListUsersAction    = "auth:ListUsers"
	ReadGroupAction    = "auth:ReadGroup"
	CreateGroupAction  = "auth:CreateGroup"
	DeleteGroupAction  = "auth:DeleteGroup"
	ListGroupsAction   = "auth:ListGroups"
	ReadPolicyAction   = "auth:ReadPolicy"
	CreatePolicyAction = "auth:CreatePolicy"
	UpdatePolicyAction = "auth:UpdatePolicy"
	DeletePolicyAction = "auth:DeletePolicy"
	ListPoliciesAction = "auth:ListPolicies"
	AttachPolicyAction = "auth:AttachPolicy"
	DetachPolicyAction = "auth:DetachPolicy"

	UserProfileAction                        = "user:UserProfile"
	ReadUserAction                           = "user:ReadUser"
	DeleteUserAction                         = "user:DeleteUser"
	ReadCredentialsAction                    = "user:ReadCredentials"
	CreateCredentialsAction                  = "user:CreateCredentials"
	DeleteCredentialsDeleteCredentialsAction = "user:DeleteCredentials"
	ListCredentialsAction                    = "user:ListCredentials"
)
```