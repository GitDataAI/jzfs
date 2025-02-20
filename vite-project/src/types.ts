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
    follow: Follow[];
    followed: Follow[];
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
    repos: string[]
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
    model: Repository;
}
