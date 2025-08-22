use crate::GitContext;
use crate::object::commit::Signature;
use error::AppError;
use git2::ObjectType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct TagItem {
    pub tag_id: String,
    pub tag_name: String,
    pub tag_msg: String,
    pub tag_tagger: Option<Signature>,
}

impl GitContext {
    pub fn tag_list(&self) -> Result<Vec<TagItem>, AppError> {
        let repo = self.repo()?;
        let tags = repo.tag_names(None)?;
        let mut result = vec![];
        for tag in tags.iter().flatten() {
            let obj = match repo.revparse_single(&format!("refs/tags/{}", tag)) {
                Ok(obj) => obj,
                Err(_) => continue,
            };
            match obj.kind() {
                Some(ObjectType::Tag) => {
                    let tag = match obj.as_tag() {
                        None => continue,
                        Some(tag) => tag,
                    };
                    let tag_name = tag.name().unwrap_or("nil").to_string();
                    let tag_msg = tag.message().unwrap_or("nil").to_string();
                    let tag_id = tag.id().to_string();
                    let mut tag_item = TagItem {
                        tag_id,
                        tag_name,
                        tag_msg,
                        tag_tagger: None,
                    };
                    match tag.tagger() {
                        None => {}
                        Some(tagger) => {
                            let tagger = Signature {
                                name: tagger.name().unwrap_or("nil").to_string(),
                                email: tagger.email().unwrap_or("nil").to_string(),
                            };
                            tag_item.tag_tagger = Some(tagger);
                        }
                    };
                    result.push(tag_item);
                }
                Some(ObjectType::Commit) => {
                    let commit = match obj.as_commit() {
                        None => continue,
                        Some(commit) => commit,
                    };
                    let tag_name = commit.message().unwrap_or("nil").to_string();
                    let tag_msg = commit.message().unwrap_or("nil").to_string();
                    let tag_id = commit.id().to_string();
                    let mut tag_item = TagItem {
                        tag_id,
                        tag_name,
                        tag_msg,
                        tag_tagger: None,
                    };
                    let author = commit.author();
                    let author = Signature {
                        name: author.name().unwrap_or("nil").to_string(),
                        email: author.email().unwrap_or("nil").to_string(),
                    };
                    tag_item.tag_tagger = Some(author);
                    result.push(tag_item);
                }
                _ => continue,
            }
        }
        Ok(result)
    }
}
