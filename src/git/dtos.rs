use git2::{Branch, Signature};

pub struct AuthorDto{
    pub name: String,
    pub email: String
}

impl <'a>From<Signature<'a>> for AuthorDto {
    fn from(value: Signature) -> Self {
        let email = value.email().unwrap_or("").to_string();
        let name = value.name().unwrap_or("").to_string();
        Self{ name, email }
    }
}

pub struct CommitDto{
    pub hash: String,
    pub author: AuthorDto,
    pub message: String,
    pub date: String,
}
impl CommitDto {
    pub fn commits(branch: Branch) -> anyhow::Result<Vec<CommitDto>>{
        let mut result = Vec::new();
        let head = branch.into_reference();
        let head = head.peel_to_commit();
        if head.is_err(){
            return Err(head.err().unwrap().into());
        }
        let mut commit = head?;
        loop {
            result.push(CommitDto{
                hash: commit.id().to_string(),
                author: commit.author().into(),
                message: commit.message().unwrap_or("").to_string(),
                date: commit.time().seconds().to_string(),
            });
            match commit.parent(0){
                Ok(parent) => commit = parent,
                Err(_) => break
            }
        }
        Ok(result)
    }
}

#[derive(Debug)]
pub struct FileDto{
    pub name: String,
    pub path: String,
    pub hash: String,
    pub message: String
}

