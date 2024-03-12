# rbac设计方案

rbac中引入resource_type, resource, statement, policy, group，user,的概念他们之间的关系如下图

![image](https://github.com/GitDataAI/jiaozifs/assets/41407352/632d8b90-25d4-423e-bcea-5114c339ddf8)

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

action 列表
```go
var Actions = []string{
"repo:ReadRepository",
"repo:CreateRepository",
"repo:UpdateRepository",
"repo:DeleteRepository",
"repo:ListRepositories",
"repo:ReadObject",
"repo:WriteObject",
"repo:DeleteObject",
"repo:ListObjects",
"repo:CreateCommit",
"repo:ReadCommit",
"repo:ListCommits",
"repo:CreateBranch",
"repo:DeleteBranch",
"repo:ReadBranch",
"repo:ReadBranch",
"repo:ListBranches",
"repo:GetWip",
"repo:ListWip",
"repo:WriteWip",
"repo:CreateWip",
"repo:DeleteWip",
"repo:ReadConfig",
"repo:WriteConfig",
"repo:CreateMergeRequest",
"repo:ReadMergeRequest",
"repo:UpdateMergeRequest",
"repo:ListMergeRequest",
"repo:MergeMergeRequest",
"repo:AddGroupMember",
"repo:RemoveGroupMember",
"repo:GetGroupMember",
"repo:GetGroupMember",
"auth:ReadGroup",
"auth:CreateGroup",
"auth:DeleteGroup",
"auth:ListGroups",
"auth:ReadPolicy",
"auth:CreatePolicy",
"auth:UpdatePolicy",
"auth:DeletePolicy",
"auth:ListPolicies",
"auth:AttachPolicy",
"auth:DetachPolicy",
"user:UserProfile",
"user:ReadUser",
"user:ListUsers",
"user:DeleteUser",
"user:ReadCredentials",
"user:CreateCredentials",
"user:DeleteCredentials",
"user:ListCredentials",
}
```

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
数据库设计基于postgres

策略表
```go
type Statement struct {
	Effect   string   `json:"effect"`
	Action   []string `json:"action"`
	Resource Resource `json:"resource"`
}

type Policy struct {
	bun.BaseModel `bun:"table:policies"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	// Name policy name
	Name string `bun:"name,unique,notnull" json:"name"`
	// Actions
	Statements []Statement `bun:"statements,type:jsonb,notnull" json:"statements"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}
```

group 表
```go
type Group struct {
	bun.BaseModel `bun:"table:groups"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	// Name policy name
	Name string `bun:"name,unique,notnull" json:"secret_key"`
	// Policies
	Policies []uuid.UUID `bun:"policies,type:jsonb,notnull" json:"policies"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}
```

用户组表
```go
type UserGroup struct {
	bun.BaseModel `bun:"table:usergroup"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	UserID        uuid.UUID `bun:"user_id,type:uuid,unique:user_group_pk,notnull" json:"user_id"`
	GroupID       uuid.UUID `bun:"group_id,type:uuid,unique:user_group_pk,notnull" json:"group_id"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}
```

仓库成员表
```go
type Member struct {
	bun.BaseModel `bun:"table:members"`
	ID            uuid.UUID `bun:"id,pk,type:uuid,default:uuid_generate_v4()" json:"id"`
	UserID        uuid.UUID `bun:"user_id,type:uuid,unique:user_repo_pk,notnull" json:"user_id"`
	RepoID        uuid.UUID `bun:"repo_id,type:uuid,unique:user_repo_pk,notnull" json:"repo_id"`
	GroupID       uuid.UUID `bun:"group_id,type:uuid,notnull" json:"group_id"`
	// CreatedAt
	CreatedAt time.Time `bun:"created_at,type:timestamp,notnull" json:"created_at"`
	// UpdatedAt
	UpdatedAt time.Time `bun:"updated_at,type:timestamp,notnull" json:"updated_at"`
}
```
