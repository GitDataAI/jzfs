use time::OffsetDateTime;

pub struct CommitDto{
    pub hash: String,
    pub message: String,
    pub author: String,
    pub email: String,
    pub date: OffsetDateTime,
    pub branch: String,
}

pub struct ObjectFile{
    pub root: String,
    pub name: String,
    pub hash: String,
}