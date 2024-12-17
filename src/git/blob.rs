use git2::Commit;

pub struct Blob<'a>{
    pub commit: Commit<'a>
}

impl <'a>Blob<'a> {
    pub fn new(commit: Commit<'a>) -> Self{
        Self{
            commit
        }
    }
    
    pub fn blobs(&self) -> anyhow::Result<Vec<String>>{
        let tree = self.commit.tree()?;
        let mut result = Vec::new();
        for entry in tree.iter(){
            match entry.kind(){
                Some(git2::ObjectType::Blob) => result.push(entry.name().unwrap().to_string()),
                _ => {}
            }
        }
        Ok(result)
    }
}