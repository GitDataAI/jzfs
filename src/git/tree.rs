use crate::git::dtos::FileDto;
use git2::Commit;

pub struct GitTree<'a>{
    pub commit: Commit<'a>
}

impl <'a>GitTree<'a>{
    pub fn new(commit: Commit<'a>) -> Self{
        Self{
            commit
        }
    }
    pub fn tree(&self) -> anyhow::Result<Vec<FileDto>>{
        let tree = self.commit.tree()?;
        let msg:&str = self.commit.message().unwrap_or("");
        let mut map = Vec::new();
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry|{
            map.push(FileDto{
                path: root.to_string(),
                name: entry.name().unwrap().to_string(),
                hash: entry.id().to_string(),
                message: msg.to_string().replace("\n", " ").replace("\r", " ")
            });      
            return git2::TreeWalkResult::Ok;
        })?;
        Ok(map)
    }
}