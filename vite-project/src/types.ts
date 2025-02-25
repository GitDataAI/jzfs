import {DateTime} from "luxon";

export interface UserModel {
    uid: string;
    name: string;
    username: string;
    email: string;
    description?: string;
    website?: string;
    avatar?: string;
    setting: string[];
    active: boolean;
    timezone?: string;
    language?: string;
    theme?: string,
    location?: string;
    created_at: DateTime;
    updated_at: DateTime;
    topic: string[];
}


export interface Repository {
    uid: string;
    name: string;
    description?: string;
    owner_id: string;
    visibility: boolean;
    fork?: string;
    default_branch: string;
    node_uid: string;
    nums_fork: number;
    nums_star: number;
    nums_watch: number;
    nums_issue: number;
    nums_pullrequest: number;
    nums_commit: number;
    nums_release: number;
    nums_tag: number;
    nums_branch: number;
    ssh: string;
    http: string;
    created_at: DateTime;
    updated_at: DateTime;
    created_by: string;
    avatar?: string;
    topic: string[];
}
export interface Watch {
    uid: string;
    user_id: string;
    repository_id: string;
    level: number;
    created_at: DateTime;
}
export interface Star {
    uid: string;
    user_id: string;
    repository_id: string;
    created_at: DateTime;
}

export interface Follow {
    uid: string;
    user_id: string;
    target_id: string;
    created_at: DateTime;
}


export interface UserDashBored {
    user: UserModel;
    repos: Repository[];
    stars: Star[];
    following: Follow[];
    followers: Follow[];
    readme?: Uint8Array;
    watch: Watch[]
}

export interface Branches {
    name: string,
    head: string,
    time: string,
}
export interface Commits {
    id: string,
    msg: string,
    time: string,
    author: string,
    email: string
}
export interface Tags {
    name: string,
    time: string,
    commit: Commits,
}

export interface Blob {
    branches: Record<string, Commits[]>;
}

export interface Tree {
    dir: string,
    id: string,
    name: string,
    child: Tree[],
    is_dir: boolean,
    commit: Commits[]
}

export interface RepoAccess {
    owner_uid: string,
    name: string,
    avatar?: string,
    repos: string[],
    repo_uids: string[]
}


export interface HotTime {
    years: number;
    month: number;
    day: number;
}

export interface HotTimeParma {
    start: HotTime;
    end: HotTime;
    limit: number;
}

export interface HotRepo {
    complex: number;
    click: number;
    fork: number;
    star: number;
    owner: string;
    model: Repository;
}


export interface DBCommit {
    uid: string,
    id: string, // sha
    repo_uid: string,
    branch_uid: string,
    branch_name: string,
    author: string,
    email: string,
    message: string,
    time: string,
    status: string,
    runner: string[],
}

export interface TokenCreate {
  name: string;
  description?: string;
  expire: number;
  access: number;
}

export interface TokenCreateReopens {
  uid: string;
  token: string;
  expire: number;
}

export interface TokenDelete {
  uid: string;
  name: string;
}

export interface TokenModel {
  uid: string;
  user_id: string;
  name: string;
  description?: string;
  access: string;
  use_history: string[];
  created_at: DateTime;
  updated_at: DateTime;
  expires_at: DateTime;
}

export interface SSHKeyCreateParma {
    name: string;
    description?: string;
    public_key: string;
}

export interface SSHKeyModel {
    uid: string;
    user_id: string;
    name: string;
    description?: string;
    created_at: DateTime;
    updated_at: DateTime;
}