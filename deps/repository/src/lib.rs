pub mod repository;

pub mod social {
    pub mod contributors;
    pub mod star;
    pub mod watch;
}

pub mod gitd {
    pub mod branch;
    pub mod commit;
    pub mod repo_nums;
}

pub mod repo_type {
    pub mod git_code;
    pub mod git_data;
    pub mod git_model;
}
pub mod storage {
    pub mod storage_nfs;
    pub mod storage_s3;
}
