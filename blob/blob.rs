use crate::blob::GitBlob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use git2::BranchType;

#[derive(Clone,Debug,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct Branches {
    pub name: String,
    pub head: String,
    pub time: String,
}
#[derive(Clone,Debug,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct Commit {
    pub id: String,
    pub msg: String,
    pub time: String,
    pub author: String,
    pub email: String,
}
#[derive(Clone,Debug,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct Tag {
    pub name: String,
    pub time: String,
    pub commit: Commit,
}

impl GitBlob {
    pub fn blob(&self) -> io::Result<HashMap<Branches,Vec<Commit>>> {
        let mut branches = HashMap::new();
        let br = self.repository.branches(None)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get branches"))?;
        for idx in br
            .flatten()
            .map(|x|x.0)
            .collect::<Vec<_>>() {
            let mut commit = Vec::new();
            
            
            let name = idx.name().unwrap_or_default().unwrap_or("").to_string();
            let reference = idx.into_reference();
            
            let mut head = match reference.peel_to_commit() {
                Ok(x) => x,
                Err(_) => continue,
            };
            let head_id = head.id().to_string();
            let time = chrono::DateTime::from_timestamp(head.time().seconds(),0).unwrap();
            
            loop {
                let parent = match head.parent(0){
                    Ok(x) => x,
                    Err(_) => break,
                };
                let parent_id = parent.id().to_string();
                let time = chrono::DateTime::from_timestamp(parent.time().seconds(),0).unwrap().timestamp().to_string();
                let msg = parent.message().unwrap_or("").to_string();
                let author = parent.author().name().unwrap_or("").to_string();
                let email = parent.author().email().unwrap_or("").to_string();
                commit.push(Commit {
                    id: parent_id.clone(),
                    msg: msg.replace("\n",""),
                    time: time.to_string(),
                    author,
                    email,
                });
                if parent_id == head_id {
                    break;
                }
                head = parent;
            }
            branches.insert(Branches {
                name,
                head: head_id,
                time: time.to_string(),
            }, commit);
        }
        
        Ok(branches)
    }
    pub fn branch(&self) -> io::Result<Vec<Branches>> {
        Ok(self.repository.branches(Some(BranchType::Local))
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get branches"))?
            .flatten()
            .map(|x| x.0)
            .map(|x| {
                let name = x.name().unwrap_or_default().unwrap_or("").to_string();
                let reference = x.into_reference();
                let head = reference.peel_to_commit().unwrap();
                let head_id = head.id().to_string();
                let time = chrono::DateTime::from_timestamp(head.time().seconds(), 0).unwrap().timestamp().to_string();
                Branches {
                    name,
                    head: head_id,
                    time: time.to_string(),
                }
            })
            .collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blob::GitBlob;
    #[test]
    fn test_branch() -> io::Result<()> {
        let blob = GitBlob::new("/home/zhenyi/文档/gitdata/.git".into()).unwrap();
        let branches = blob.blob()?;
        dbg!(branches);
        Ok(())
    }
    #[test]
    fn test_blob() -> io::Result<()> {
        let blob = GitBlob::new("/home/zhenyi/文档/gitdata/.git".into()).unwrap();
        let branches = blob.branch()?;
        dbg!(branches);
        Ok(())
    }
}