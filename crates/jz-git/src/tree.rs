use crate::GitParam;
use git2::{BranchType, TreeWalkResult};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitTreeParam {
    pub path: String,
    pub branches: Option<String>,
    pub sha: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct TreeEntityItem {
    pub name: String,
    pub path: String,
    pub root: String,
    pub r#type: String,
    pub oid: String,
}

impl GitParam {
    pub fn tree(&mut self, param: GitTreeParam) -> anyhow::Result<Vec<TreeEntityItem>> {
        let mut param_path = param
            .path
            .split("/")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("/");
        if !param_path.is_empty() && !param_path.ends_with("/") {
            param_path.push_str("/");
        }

        let repo = self.repo()?;
        let branches = match param.branches {
            Some(b) => match repo.find_branch(&b, BranchType::Local) {
                Ok(b) => b.into_reference(),
                Err(_) => return Err(anyhow::anyhow!("branch not found")),
            },
            None => match repo.head() {
                Ok(b) => b,
                Err(_) => return Err(anyhow::anyhow!("branch not found")),
            },
        };

        let commit = if let Some(sha) = param.sha {
            repo.find_commit(sha.parse::<git2::Oid>()?)?
        } else {
            branches.peel_to_commit()?
        };

        let tree = commit.tree()?;
        let mut result = vec![];
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            let name = entry.name().unwrap_or("?").to_string();
            let path = format!("{}{}", root, entry.name().unwrap_or("?"));
            if root == param_path {
                match entry.kind() {
                    Some(git2::ObjectType::Tree) => {
                        result.push(TreeEntityItem {
                            name: name.clone(),
                            path: path.clone(),
                            root: root.to_string(),
                            r#type: "tree".to_string(),
                            oid: entry.id().to_string(),
                        });
                    }
                    Some(git2::ObjectType::Blob) => result.push(TreeEntityItem {
                        name: name.clone(),
                        path: path.clone(),
                        root: root.to_string(),
                        r#type: "blob".to_string(),
                        oid: entry.id().to_string(),
                    }),
                    _ => {}
                }
            }
            TreeWalkResult::Ok
        })
        .ok();

        Ok(result)
    }
}

#[test]
fn test_tree() {
    let mut parma = GitParam {
        root: "/home/zhenyi/文档/".parse().unwrap(),
        uid: "gitlab".parse().unwrap(),
        repo: None,
    };
    let tree = parma.tree(GitTreeParam {
        path: "".to_string(),
        branches: Option::from("master".to_string()),
        sha: None,
    });
    let tree = tree.unwrap();
    dbg!(tree.len());
    // let start = std::time::Instant::now();
    // let p = parma.tree_dir(tree.clone(), Some("main".to_string()), Option::from("3ee5ee2029529e3b0c600f4ec555b3daca178fbe".to_string())).unwrap();
    // dbg!(start.elapsed());
    // dbg!(p.len());
    // dbg!(&tree);
    let start = std::time::Instant::now();
    let p = parma
        .tree_msg(Some("master".to_string()))
        .unwrap();
    dbg!(start.elapsed());
    dbg!(p.len());
    // let t = tree.iter().map(|x|x.path.to_string()).collect::<Vec<_>>();
    // let b = p.iter().map(|x|x.name.to_string()).collect::<Vec<_>>();
    // diff t and b
    // for (i, x) in t.iter().enumerate() {
    //     if !b.contains(x) {
    //         dbg!(i);
    //         dbg!(x);
    //     }
    // }
}
