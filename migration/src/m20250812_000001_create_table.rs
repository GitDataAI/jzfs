use crate::ColumnType::Text;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 启用UUID扩展
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
            .await?;

        // 创建email_verifications表
        manager
            .create_table(
                Table::create()
                    .table(EmailVerifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EmailVerifications::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(
                        ColumnDef::new(EmailVerifications::UserUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerifications::Email)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerifications::VerificationCode)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerifications::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerifications::VerifiedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(EmailVerifications::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(EmailVerifications::IsUsed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建git_blob表
        manager
            .create_table(
                Table::create()
                    .table(GitBlob::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitBlob::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(GitBlob::RepoUid).uuid().not_null())
                    .col(ColumnDef::new(GitBlob::BlobId).string().not_null())
                    .col(ColumnDef::new(GitBlob::Name).string().null())
                    .col(ColumnDef::new(GitBlob::Size).integer().not_null())
                    .col(ColumnDef::new(GitBlob::CommitId).string().not_null())
                    .col(
                        ColumnDef::new(GitBlob::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建git_commit表
        manager
            .create_table(
                Table::create()
                    .table(GitCommit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitCommit::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(GitCommit::RepoUid).uuid().not_null())
                    .col(ColumnDef::new(GitCommit::CommitId).string().not_null())
                    .col(ColumnDef::new(GitCommit::RefsUid).string().not_null())
                    .col(ColumnDef::new(GitCommit::Tree).string().not_null())
                    .col(ColumnDef::new(GitCommit::ParentsId).json().not_null())
                    .col(ColumnDef::new(GitCommit::Author).uuid().null())
                    .col(ColumnDef::new(GitCommit::Committer).uuid().null())
                    .col(ColumnDef::new(GitCommit::Content).text().not_null())
                    .col(ColumnDef::new(GitCommit::Time).big_integer().not_null())
                    .col(ColumnDef::new(GitCommit::Offset).integer().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建git_refs表
        manager
            .create_table(
                Table::create()
                    .table(GitRefs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitRefs::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(GitRefs::RepoUid).uuid().not_null())
                    .col(ColumnDef::new(GitRefs::RefName).text().not_null())
                    .col(ColumnDef::new(GitRefs::RefGitId).string().not_null())
                    .col(
                        ColumnDef::new(GitRefs::DefaultBranch)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(GitRefs::IsPrivate)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(GitRefs::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(GitRefs::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建git_repo表
        manager
            .create_table(
                Table::create()
                    .table(GitRepo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitRepo::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(GitRepo::Namespace).string().not_null())
                    .col(ColumnDef::new(GitRepo::RepoName).text().not_null())
                    .col(ColumnDef::new(GitRepo::Description).text().null())
                    .col(ColumnDef::new(GitRepo::DefaultHead).string().not_null())
                    .col(
                        ColumnDef::new(GitRepo::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(GitRepo::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(ColumnDef::new(GitRepo::Storage).string().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建git_tag表
        manager
            .create_table(
                Table::create()
                    .table(GitTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitTag::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(GitTag::RepoUid).uuid().not_null())
                    .col(ColumnDef::new(GitTag::TagId).string().not_null())
                    .col(ColumnDef::new(GitTag::TagName).string().not_null())
                    .col(ColumnDef::new(GitTag::Tagger).uuid().null())
                    .col(ColumnDef::new(GitTag::Message).string().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建git_tree表
        manager
            .create_table(
                Table::create()
                    .table(GitTree::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitTree::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(GitTree::RepoUid).uuid().not_null())
                    .col(ColumnDef::new(GitTree::TreeId).string().not_null())
                    .col(ColumnDef::new(GitTree::SubTrees).json().not_null())
                    .col(ColumnDef::new(GitTree::Size).integer().not_null())
                    .col(ColumnDef::new(GitTree::CommitId).string().not_null())
                    .col(
                        ColumnDef::new(GitTree::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建lfs_locks表
        manager
            .create_table(
                Table::create()
                    .table(LfsLocks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LfsLocks::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LfsLocks::Data).text().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建lfs_objects表
        manager
            .create_table(
                Table::create()
                    .table(LfsObjects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LfsObjects::Oid)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LfsObjects::Size).big_integer().not_null())
                    .col(
                        ColumnDef::new(LfsObjects::Exist)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(LfsObjects::Splited)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建lfs_split_relations表
        manager
            .create_table(
                Table::create()
                    .table(LfsSplitRelations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LfsSplitRelations::OriOid)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LfsSplitRelations::SubOid)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LfsSplitRelations::Offset)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LfsSplitRelations::Size)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_lfs_split_relations")
                            .col(LfsSplitRelations::OriOid)
                            .col(LfsSplitRelations::SubOid)
                            .col(LfsSplitRelations::Offset),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建oauth_providers表
        manager
            .create_table(
                Table::create()
                    .table(OauthProviders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OauthProviders::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(OauthProviders::Name).string().not_null())
                    .col(
                        ColumnDef::new(OauthProviders::DisplayName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OauthProviders::ClientId).string().not_null())
                    .col(
                        ColumnDef::new(OauthProviders::ClientSecret)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OauthProviders::AuthorizationUrl)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OauthProviders::TokenUrl).string().not_null())
                    .col(
                        ColumnDef::new(OauthProviders::UserInfoUrl)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OauthProviders::Scope).string().not_null())
                    .col(
                        ColumnDef::new(OauthProviders::IsEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(OauthProviders::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(OauthProviders::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建password_resets表
        manager
            .create_table(
                Table::create()
                    .table(PasswordResets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PasswordResets::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(PasswordResets::UserUid).uuid().not_null())
                    .col(
                        ColumnDef::new(PasswordResets::ResetToken)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResets::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(ColumnDef::new(PasswordResets::UsedAt).timestamp().null())
                    .col(
                        ColumnDef::new(PasswordResets::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PasswordResets::IsUsed)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建ssh_keys表
        manager
            .create_table(
                Table::create()
                    .table(SshKeys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SshKeys::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(SshKeys::UserId).uuid().not_null())
                    .col(ColumnDef::new(SshKeys::Name).string().not_null())
                    .col(ColumnDef::new(SshKeys::Fingerprint).string().not_null())
                    .col(ColumnDef::new(SshKeys::Description).string().null())
                    .col(ColumnDef::new(SshKeys::Content).string().not_null())
                    .col(
                        ColumnDef::new(SshKeys::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(SshKeys::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建access_key表
        manager
            .create_table(
                Table::create()
                    .table(AccessKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AccessKey::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(AccessKey::Title).string().not_null())
                    .col(ColumnDef::new(AccessKey::Description).string().null())
                    .col(ColumnDef::new(AccessKey::Token).string().not_null())
                    .col(
                        ColumnDef::new(AccessKey::UseHistory)
                            .array(Text)
                            .not_null()
                            .default("{}"),
                    )
                    .col(ColumnDef::new(AccessKey::ResourceOwner).string().not_null())
                    .col(
                        ColumnDef::new(AccessKey::ResourceOwnerUid)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(AccessKey::Expiration).string().not_null())
                    .col(ColumnDef::new(AccessKey::Fingerprint).string().not_null())
                    .col(
                        ColumnDef::new(AccessKey::RepoAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::EmailAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::EventAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::FollowAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::GpgAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::SshAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::WebhookAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::WikiAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::ProjectAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::IssueAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::CommentAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::ProfileAccess)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(AccessKey::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(AccessKey::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建user_black表
        manager
            .create_table(
                Table::create()
                    .table(UserBlack::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserBlack::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserBlack::UserUid).uuid().not_null())
                    .col(ColumnDef::new(UserBlack::BlackUid).uuid().not_null())
                    .col(
                        ColumnDef::new(UserBlack::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建user_follow表
        manager
            .create_table(
                Table::create()
                    .table(UserFollow::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserFollow::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserFollow::UserUid).uuid().not_null())
                    .col(ColumnDef::new(UserFollow::FollowUid).uuid().not_null())
                    .col(
                        ColumnDef::new(UserFollow::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建user_login_logs表
        manager
            .create_table(
                Table::create()
                    .table(UserLoginLogs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLoginLogs::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserLoginLogs::UserUid).uuid().not_null())
                    .col(
                        ColumnDef::new(UserLoginLogs::LoginMethod)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserLoginLogs::IpAddress).string().null())
                    .col(ColumnDef::new(UserLoginLogs::UserAgent).string().null())
                    .col(ColumnDef::new(UserLoginLogs::LocationInfo).json().null())
                    .col(
                        ColumnDef::new(UserLoginLogs::Success)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(UserLoginLogs::FailureReason).string().null())
                    .col(
                        ColumnDef::new(UserLoginLogs::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建user_nostr表
        manager
            .create_table(
                Table::create()
                    .table(UserNostr::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserNostr::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserNostr::UserUid).uuid().not_null())
                    .col(ColumnDef::new(UserNostr::Relay).string().not_null())
                    .col(ColumnDef::new(UserNostr::Pubkey).string().not_null())
                    .col(ColumnDef::new(UserNostr::Seckey).string().not_null())
                    .col(ColumnDef::new(UserNostr::PinCode).string().not_null())
                    .col(
                        ColumnDef::new(UserNostr::IsActive)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(UserNostr::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserNostr::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(ColumnDef::new(UserNostr::LastUsedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        // 创建user_repo_active表
        manager
            .create_table(
                Table::create()
                    .table(UserRepoActive::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRepoActive::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserRepoActive::UserUid).uuid().null())
                    .col(ColumnDef::new(UserRepoActive::Commit).uuid().not_null())
                    .col(ColumnDef::new(UserRepoActive::RepoUid).uuid().not_null())
                    .col(
                        ColumnDef::new(UserRepoActive::Time)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserRepoActive::Offset).integer().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建user_repo_tagger表
        manager
            .create_table(
                Table::create()
                    .table(UserRepoTagger::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRepoTagger::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserRepoTagger::RepoUid).uuid().not_null())
                    .col(ColumnDef::new(UserRepoTagger::Name).string().not_null())
                    .col(ColumnDef::new(UserRepoTagger::Email).string().not_null())
                    .col(ColumnDef::new(UserRepoTagger::UserUid).uuid().null())
                    .to_owned(),
            )
            .await?;

        // 创建user_sessions表
        manager
            .create_table(
                Table::create()
                    .table(UserSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserSessions::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserSessions::UserId).uuid().null())
                    .col(
                        ColumnDef::new(UserSessions::SessionToken)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserSessions::IpAddress).string().null())
                    .col(ColumnDef::new(UserSessions::Value).string().not_null())
                    .col(ColumnDef::new(UserSessions::UserAgent).string().null())
                    .col(ColumnDef::new(UserSessions::DeviceInfo).string().null())
                    .col(
                        ColumnDef::new(UserSessions::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserSessions::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSessions::LastUsedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(UserSessions::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建users表
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(Users::Username).string().not_null())
                    .col(ColumnDef::new(Users::Email).string().not_null())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Users::DisplayName).string().null())
                    .col(ColumnDef::new(Users::AvatarUrl).string().null())
                    .col(ColumnDef::new(Users::Bio).string().null())
                    .col(ColumnDef::new(Users::Location).string().null())
                    .col(ColumnDef::new(Users::WebsiteUrl).string().null())
                    .col(ColumnDef::new(Users::Company).string().null())
                    .col(
                        ColumnDef::new(Users::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Users::IsVerified)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::IsPremium)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra("DEFAULT NOW()".to_string()),
                    )
                    .col(ColumnDef::new(Users::LastLoginAt).timestamp().null())
                    .col(ColumnDef::new(Users::Timezone).string().null())
                    .col(ColumnDef::new(Users::Language).string().null())
                    .col(ColumnDef::new(Users::Theme).string().null())
                    .col(
                        ColumnDef::new(Users::LoginCount)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建user_repo表
        manager
            .create_table(
                Table::create()
                    .table(UserRepo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRepo::Uid)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .extra("DEFAULT uuid_generate_v4()".to_string()),
                    )
                    .col(ColumnDef::new(UserRepo::RepoUid).uuid().not_null())
                    .col(ColumnDef::new(UserRepo::UserUid).uuid().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_email_verifications_user_uid")
                    .table(EmailVerifications::Table)
                    .col(EmailVerifications::UserUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_git_blob_repo_uid")
                    .table(GitBlob::Table)
                    .col(GitBlob::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_git_commit_repo_uid")
                    .table(GitCommit::Table)
                    .col(GitCommit::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_git_refs_repo_uid")
                    .table(GitRefs::Table)
                    .col(GitRefs::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_git_tag_repo_uid")
                    .table(GitTag::Table)
                    .col(GitTag::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_git_tree_repo_uid")
                    .table(GitTree::Table)
                    .col(GitTree::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ssh_keys_user_id")
                    .table(SshKeys::Table)
                    .col(SshKeys::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_access_key_resource_owner_uid")
                    .table(AccessKey::Table)
                    .col(AccessKey::ResourceOwnerUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_black_user_uid")
                    .table(UserBlack::Table)
                    .col(UserBlack::UserUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_follow_user_uid")
                    .table(UserFollow::Table)
                    .col(UserFollow::UserUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_login_logs_user_uid")
                    .table(UserLoginLogs::Table)
                    .col(UserLoginLogs::UserUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_nostr_user_uid")
                    .table(UserNostr::Table)
                    .col(UserNostr::UserUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_repo_active_repo_uid")
                    .table(UserRepoActive::Table)
                    .col(UserRepoActive::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_repo_tagger_repo_uid")
                    .table(UserRepoTagger::Table)
                    .col(UserRepoTagger::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_sessions_user_id")
                    .table(UserSessions::Table)
                    .col(UserSessions::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_repo_repo_uid")
                    .table(UserRepo::Table)
                    .col(UserRepo::RepoUid)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_user_repo_user_uid")
                    .table(UserRepo::Table)
                    .col(UserRepo::UserUid)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除索引
        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_repo_user_uid")
                    .table(UserRepo::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_repo_repo_uid")
                    .table(UserRepo::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_sessions_user_id")
                    .table(UserSessions::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_repo_tagger_repo_uid")
                    .table(UserRepoTagger::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_repo_active_repo_uid")
                    .table(UserRepoActive::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_nostr_user_uid")
                    .table(UserNostr::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_login_logs_user_uid")
                    .table(UserLoginLogs::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_follow_user_uid")
                    .table(UserFollow::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_user_black_user_uid")
                    .table(UserBlack::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_access_key_resource_owner_uid")
                    .table(AccessKey::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_ssh_keys_user_id")
                    .table(SshKeys::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_git_tree_repo_uid")
                    .table(GitTree::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_git_tag_repo_uid")
                    .table(GitTag::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_git_refs_repo_uid")
                    .table(GitRefs::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_git_commit_repo_uid")
                    .table(GitCommit::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_git_blob_repo_uid")
                    .table(GitBlob::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_email_verifications_user_uid")
                    .table(EmailVerifications::Table)
                    .to_owned(),
            )
            .await?;

        // 删除表
        manager
            .drop_table(Table::drop().table(UserRepo::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserSessions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserRepoTagger::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserRepoActive::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserNostr::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserLoginLogs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserFollow::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserBlack::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AccessKey::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SshKeys::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PasswordResets::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OauthProviders::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LfsSplitRelations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LfsObjects::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(LfsLocks::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitTree::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitTag::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitRepo::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitRefs::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitCommit::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(GitBlob::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(EmailVerifications::Table).to_owned())
            .await?;

        // 禁用UUID扩展（通常不建议在生产环境中这样做）
        manager
            .get_connection()
            .execute_unprepared("DROP EXTENSION IF EXISTS \"uuid-ossp\"")
            .await?;

        Ok(())
    }
}

// 表名和列名定义
#[derive(Iden)]
enum EmailVerifications {
    Table,
    Uid,
    UserUid,
    Email,
    VerificationCode,
    ExpiresAt,
    VerifiedAt,
    CreatedAt,
    IsUsed,
}

#[derive(Iden)]
enum GitBlob {
    Table,
    Uid,
    RepoUid,
    BlobId,
    Name,
    Size,
    CommitId,
    CreatedAt,
}

#[derive(Iden)]
enum GitCommit {
    Table,
    Uid,
    RepoUid,
    RefsUid,
    CommitId,
    Tree,
    ParentsId,
    Author,
    Committer,
    Content,
    Time,
    Offset,
}

#[derive(Iden)]
enum GitRefs {
    Table,
    Uid,
    RepoUid,
    RefName,
    RefGitId,
    DefaultBranch,
    CreatedAt,
    UpdatedAt,
    IsPrivate,
}

#[derive(Iden)]
enum GitRepo {
    Table,
    Uid,
    Description,
    Namespace,
    RepoName,
    DefaultHead,
    CreatedAt,
    UpdatedAt,
    Storage,
}

#[derive(Iden)]
enum GitTag {
    Table,
    Uid,
    RepoUid,
    TagId,
    TagName,
    Tagger,
    Message,
}

#[derive(Iden)]
enum GitTree {
    Table,
    Uid,
    RepoUid,
    TreeId,
    SubTrees,
    Size,
    CommitId,
    CreatedAt,
}

#[derive(Iden)]
enum LfsLocks {
    Table,
    Id,
    Data,
}

#[derive(Iden)]
enum LfsObjects {
    Table,
    Oid,
    Size,
    Exist,
    Splited,
}

#[derive(Iden)]
enum LfsSplitRelations {
    Table,
    OriOid,
    SubOid,
    Offset,
    Size,
}

#[derive(Iden)]
enum OauthProviders {
    Table,
    Uid,
    Name,
    DisplayName,
    ClientId,
    ClientSecret,
    AuthorizationUrl,
    TokenUrl,
    UserInfoUrl,
    Scope,
    IsEnabled,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum PasswordResets {
    Table,
    Uid,
    UserUid,
    ResetToken,
    CreatedAt,
    UsedAt,
    ExpiresAt,
    IsUsed,
}

#[derive(Iden)]
enum SshKeys {
    Table,
    Uid,
    UserId,
    Name,
    Fingerprint,
    Description,
    Content,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum AccessKey {
    Table,
    Uid,
    Title,
    Description,
    Token,
    UseHistory,
    ResourceOwner,
    ResourceOwnerUid,
    Expiration,
    Fingerprint,
    RepoAccess,
    EmailAccess,
    EventAccess,
    FollowAccess,
    GpgAccess,
    SshAccess,
    WebhookAccess,
    WikiAccess,
    ProjectAccess,
    IssueAccess,
    CommentAccess,
    ProfileAccess,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum UserBlack {
    Table,
    Uid,
    UserUid,
    BlackUid,
    CreatedAt,
}

#[derive(Iden)]
enum UserFollow {
    Table,
    Uid,
    UserUid,
    FollowUid,
    CreatedAt,
}

#[derive(Iden)]
enum UserLoginLogs {
    Table,
    Uid,
    UserUid,
    LoginMethod,
    IpAddress,
    UserAgent,
    LocationInfo,
    Success,
    FailureReason,
    CreatedAt,
}

#[derive(Iden)]
enum UserNostr {
    Table,
    Uid,
    UserUid,
    Relay,
    Pubkey,
    Seckey,
    PinCode,
    IsActive,
    UpdatedAt,
    CreatedAt,
    LastUsedAt,
}

#[derive(Iden)]
enum UserRepoActive {
    Table,
    Uid,
    UserUid,
    Commit,
    RepoUid,
    Time,
    Offset,
}

#[derive(Iden)]
enum UserRepoTagger {
    Table,
    Uid,
    RepoUid,
    Name,
    Email,
    UserUid,
}

#[derive(Iden)]
enum UserSessions {
    Table,
    Uid,
    UserId,
    SessionToken,
    IpAddress,
    Value,
    UserAgent,
    DeviceInfo,
    CreatedAt,
    ExpiresAt,
    LastUsedAt,
    IsActive,
}

#[derive(Iden)]
enum Users {
    Table,
    Uid,
    Username,
    Email,
    PasswordHash,
    DisplayName,
    AvatarUrl,
    Bio,
    Location,
    WebsiteUrl,
    Company,
    IsActive,
    IsVerified,
    IsPremium,
    CreatedAt,
    UpdatedAt,
    LastLoginAt,
    Timezone,
    Language,
    Theme,
    LoginCount,
}

#[derive(Iden)]
enum UserRepo {
    Table,
    Uid,
    RepoUid,
    UserUid,
}
