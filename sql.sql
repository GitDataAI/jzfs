create database gitdata;
CREATE TABLE users (
           uid UUID PRIMARY KEY,
           name VARCHAR(255) NOT NULL,
           username VARCHAR(255) NOT NULL,
           email VARCHAR(255) NOT NULL,
           public_email BOOLEAN NOT NULL,
           avatar VARCHAR(255),
           phone VARCHAR(255),
           status INTEGER NOT NULL,
           sex VARCHAR(255),
           website TEXT[],
           company VARCHAR(255) NOT NULL,
           description TEXT NOT NULL,
           "localtime" VARCHAR(255) NOT NULL,
           timezone VARCHAR(255) NOT NULL,
           theme VARCHAR(255) NOT NULL,
           team UUID[],
           repo UUID[],
           project UUID[],
           issue UUID[],
           pr UUID[],
           commit UUID[],
           tag UUID[],
           star UUID[],
           follow UUID[],
           pro BOOLEAN NOT NULL,
           passwd VARCHAR(255) NOT NULL,
           created_at TIMESTAMPTZ NOT NULL,
           updated_at TIMESTAMPTZ NOT NULL,
           lastlogin TIMESTAMPTZ NOT NULL
);

CREATE TABLE teams (
               uid UUID PRIMARY KEY,
               group_id UUID NOT NULL,
               name VARCHAR(255) NOT NULL,
               description TEXT NOT NULL,
               created_at TIMESTAMPTZ NOT NULL,
               updated_at TIMESTAMPTZ NOT NULL,
               created_by UUID NOT NULL
);

CREATE TABLE teams_invite (
              uid UUID PRIMARY KEY,
              group_id UUID NOT NULL,
              team_id UUID NOT NULL,
              user_id UUID NOT NULL,
              email VARCHAR(255) NOT NULL,
              status INTEGER NOT NULL, -- 0 wait / 1 ok / -1 no
              created_at TIMESTAMPTZ NOT NULL,
              updated_at TIMESTAMPTZ NOT NULL,
              invited_by UUID NOT NULL
);

CREATE TABLE teams_users (
             uid UUID PRIMARY KEY,
             team_id UUID NOT NULL,
             user_id UUID NOT NULL,
             join_at TIMESTAMPTZ NOT NULL,
             access INTEGER NOT NULL -- 0 read / 1 write / 2 admin / 3 owner
);


CREATE TABLE groups (
            uid UUID PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            avatar VARCHAR(255),
            website TEXT[], -- 存储字符串数组
            location VARCHAR(255) NOT NULL,
            unit VARCHAR(255),
            contact VARCHAR(255) NOT NULL,
            owner UUID NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE group_repo (
            uid UUID PRIMARY KEY NOT NULL UNIQUE ,
            repo_id UUID NOT NULL,
            group_id UUID NOT NULL
);

CREATE TABLE group_repo_access (
            uid UUID NOT NULL PRIMARY KEY UNIQUE ,
           repo_id UUID NOT NULL,
           group_id UUID NOT NULL,
           team_id UUID NOT NULL,
           access INTEGER NOT NULL -- 0 read / 1 write / 2 admin / 3 owner
);

CREATE TABLE repos (
           uid UUID PRIMARY KEY,
           name VARCHAR(255) NOT NULL,
           description TEXT,
           commit BIGINT NOT NULL,
           head_hash VARCHAR(255) NOT NULL,
           star BIGINT NOT NULL,
           fork BIGINT NOT NULL,
           is_fork BOOLEAN NOT NULL,
           fork_from UUID,
           watch BIGINT NOT NULL,
           issue BIGINT NOT NULL,
           open_issue BIGINT NOT NULL,
           close_issue BIGINT NOT NULL,
           pr BIGINT NOT NULL,
           open_pr BIGINT NOT NULL,
           close_pr BIGINT NOT NULL,
           is_empty BOOLEAN NOT NULL,
           visible BOOLEAN NOT NULL,
           topic TEXT[],
           size DOUBLE PRECISION NOT NULL,
           created_at TIMESTAMPTZ NOT NULL,
           updated_at TIMESTAMPTZ NOT NULL,
           created_by UUID NOT NULL
);


CREATE TABLE repo_branch (
                             uid UUID PRIMARY KEY,
                             repo_id UUID NOT NULL,
                             branch VARCHAR(255) NOT NULL,
                             protect BOOLEAN NOT NULL,
                             visible BOOLEAN NOT NULL,
                             head UUID,
                             created_at TIMESTAMPTZ NOT NULL,
                             updated_at TIMESTAMPTZ NOT NULL,
                             created_by UUID NOT NULL,
                             FOREIGN KEY (repo_id) REFERENCES repos(uid)
);

CREATE TABLE repo_commit (
                             uid UUID PRIMARY KEY,
                             repo_id UUID NOT NULL,
                             branch_id UUID NOT NULL,
                             bio TEXT,
                             commit_user VARCHAR(255) NOT NULL,
                             commit_email VARCHAR(255) NOT NULL,
                             commit_user_id UUID NOT NULL,
                             commit_id BIGINT NOT NULL,
                             created_at TIMESTAMPTZ NOT NULL,
                             FOREIGN KEY (repo_id) REFERENCES repos(uid),
                             FOREIGN KEY (branch_id) REFERENCES repo_branch(uid)
);

CREATE TABLE repo_contribute (
                                 uid VARCHAR(255) PRIMARY KEY,
                                 user_id UUID NOT NULL,
                                 repo_id UUID NOT NULL,
                                 contribute TEXT NOT NULL,
                                 first_at TIMESTAMPTZ NOT NULL,
                                 last_at TIMESTAMPTZ NOT NULL,
                                 FOREIGN KEY (repo_id) REFERENCES repos(uid)
);
CREATE TABLE repo_license (
                              uid UUID PRIMARY KEY,
                              repo_id UUID NOT NULL,
                              name VARCHAR(255) NOT NULL,
                              license TEXT NOT NULL,
                              created_at TIMESTAMPTZ NOT NULL,
                              updated_at TIMESTAMPTZ NOT NULL,
                              created_by UUID NOT NULL,
                              FOREIGN KEY (repo_id) REFERENCES repos(uid)
);

CREATE TABLE repo_watch (
                            uid UUID PRIMARY KEY,
                            user_id UUID NOT NULL,
                            repo_id UUID NOT NULL,
                            mode BIGINT NOT NULL,
                            created_at TIMESTAMPTZ NOT NULL,
                            updated_at TIMESTAMPTZ NOT NULL,
                            FOREIGN KEY (repo_id) REFERENCES repos(uid)
);