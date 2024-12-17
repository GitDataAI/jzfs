CREATE TABLE "users" (
                         "uid" UUID PRIMARY KEY,
                         "name" VARCHAR NOT NULL,
                         "username" VARCHAR NOT NULL,
                         "avatar" BYTEA,
                         "phone" VARCHAR,
                         "status" INTEGER NOT NULL,
                         "website" VARCHAR[] NOT NULL,
                         "company" VARCHAR NOT NULL,
                         "description" VARCHAR NOT NULL,
                         "localtime" VARCHAR NOT NULL,
                         "timezone" VARCHAR NOT NULL,
                         "theme" VARCHAR NOT NULL,
                         "pro" BOOLEAN NOT NULL,
                         "passwd" VARCHAR NOT NULL,
                         "created_at" TIMESTAMPTZ NOT NULL,
                         "updated_at" TIMESTAMPTZ NOT NULL,
                         "lastlogin" TIMESTAMPTZ NOT NULL,
                         "is_groups" BOOLEAN NOT NULL
);

CREATE TABLE "users_data" (
                              "uid" UUID PRIMARY KEY,
                              "user_id" UUID NOT NULL,
                              "repo" UUID[] NOT NULL,
                              "project" UUID[] NOT NULL,
                              "issue" UUID[] NOT NULL,
                              "pr" UUID[] NOT NULL,
                              "commit" UUID[] NOT NULL,
                              "tag" UUID[] NOT NULL,
                              "star" UUID[] NOT NULL,
                              "follow" UUID[] NOT NULL,
                              "following" UUID[] NOT NULL,
                              "watcher" UUID[] NOT NULL
);
CREATE TABLE "users_email" (
                               "uid" UUID PRIMARY KEY,
                               "user_id" UUID NOT NULL,
                               "name" VARCHAR NOT NULL,
                               "email" VARCHAR NOT NULL,
                               "is_public" BOOLEAN NOT NULL,
                               "verified" BOOLEAN NOT NULL,
                               "bind_at" VARCHAR NOT NULL
);

CREATE TABLE "users_key" (
                             "uid" UUID PRIMARY KEY,
                             "user_id" UUID NOT NULL,
                             "pubkey" VARCHAR NOT NULL,
                             "name" VARCHAR NOT NULL,
                             "created_at" TIMESTAMPTZ NOT NULL,
                             "last_use" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "repos" (
                         "uid" UUID PRIMARY KEY,
                         "name" VARCHAR NOT NULL,
                         "description" VARCHAR NOT NULL,
                         "owner" VARCHAR NOT NULL,
                         "commit" BIGINT NOT NULL,
                         "head_hash" VARCHAR NOT NULL,
                         "ssh_path" VARCHAR NOT NULL,
                         "http_path" VARCHAR NOT NULL,
                         "star" BIGINT NOT NULL,
                         "fork" BIGINT NOT NULL,
                         "is_fork" BOOLEAN NOT NULL,
                         "fork_from" UUID,
                         "watch" BIGINT NOT NULL,
                         "issue" BIGINT NOT NULL,
                         "open_issue" BIGINT NOT NULL,
                         "close_issue" BIGINT NOT NULL,
                         "pr" BIGINT NOT NULL,
                         "open_pr" BIGINT NOT NULL,
                         "close_pr" BIGINT NOT NULL,
                         "is_empty" BOOLEAN NOT NULL,
                         "visible" BOOLEAN NOT NULL,
                         "topic" VARCHAR[] NOT NULL,
                         "size" DOUBLE PRECISION NOT NULL,
                         "created_at" TIMESTAMPTZ NOT NULL,
                         "updated_at" TIMESTAMPTZ NOT NULL,
                         "created_by" UUID NOT NULL
);

CREATE TABLE "repo_branch" (
                               "uid" UUID PRIMARY KEY,
                               "repo_id" UUID NOT NULL,
                               "branch" VARCHAR NOT NULL,
                               "protect" BOOLEAN NOT NULL,
                               "visible" BOOLEAN NOT NULL,
                               "head" UUID,
                               "created_at" TIMESTAMPTZ NOT NULL,
                               "updated_at" TIMESTAMPTZ NOT NULL,
                               "created_by" UUID NOT NULL
);

CREATE TABLE "repo_commit" (
                               "uid" UUID PRIMARY KEY,
                               "repo_id" UUID NOT NULL,
                               "branch_id" UUID NOT NULL,
                               "bio" VARCHAR NOT NULL,
                               "commit_user" VARCHAR NOT NULL,
                               "commit_email" VARCHAR NOT NULL,
                               "commit_id" VARCHAR NOT NULL,
                               "created_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "repo_contribute" (
                                   "uid" VARCHAR PRIMARY KEY,
                                   "user_id" UUID NOT NULL,
                                   "repo_id" UUID NOT NULL,
                                   "contribute" VARCHAR NOT NULL,
                                   "first_at" TIMESTAMPTZ NOT NULL,
                                   "last_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "repo_license" (
                                "uid" UUID PRIMARY KEY,
                                "repo_id" UUID NOT NULL,
                                "name" VARCHAR NOT NULL,
                                "license" VARCHAR NOT NULL,
                                "created_at" TIMESTAMPTZ NOT NULL,
                                "updated_at" TIMESTAMPTZ NOT NULL,
                                "created_by" UUID NOT NULL
);

CREATE TABLE "repo_watch" (
                              "uid" UUID PRIMARY KEY,
                              "user_id" UUID NOT NULL,
                              "repo_id" UUID NOT NULL,
                              "mode" BIGINT NOT NULL,
                              "created_at" TIMESTAMPTZ NOT NULL,
                              "updated_at" TIMESTAMPTZ NOT NULL
);
CREATE TABLE "groups" (
                          "uid" UUID PRIMARY KEY,
                          "group_id" UUID NOT NULL,
                          "users_id" UUID NOT NULL,
                          "access" INTEGER NOT NULL,
                          "join_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "groups_repo" (
                               "uid" UUID PRIMARY KEY,
                               "group_id" UUID NOT NULL,
                               "repo_id" UUID NOT NULL
);

CREATE TABLE "groups_invite" (
                                 "uid" UUID PRIMARY KEY,
                                 "group_id" UUID NOT NULL,
                                 "user_id" UUID NOT NULL,
                                 "email" VARCHAR NOT NULL,
                                 "status" INTEGER NOT NULL,
                                 "created_at" TIMESTAMPTZ NOT NULL,
                                 "updated_at" TIMESTAMPTZ NOT NULL,
                                 "invited_by" UUID NOT NULL
);

CREATE TABLE "groups_labels" (
                                 "uid" UUID PRIMARY KEY,
                                 "label" VARCHAR NOT NULL,
                                 "color" VARCHAR NOT NULL,
                                 "group_id" UUID NOT NULL
);

