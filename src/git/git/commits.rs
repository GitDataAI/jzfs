use crate::error::JZResult;
use crate::git::git::options::{BlobTree, BlobTreeMsg, Commits};
use crate::git::git::GitLocal;
use git2::BranchType;

impl GitLocal {
    pub fn commit_list(&self) -> JZResult<Vec<Commits>> {
        let branchs = self.branch_list()?;
        let mut result = vec![];
        for branch in branchs {
            if branch.local {
                if let Ok(refs) = self
                    .repository
                    .find_branch(&branch.name, git2::BranchType::Local)
                {
                    if let Ok(mut commits) = refs.into_reference().peel_to_commit() {
                        loop {
                            let hash_oid = commits.id().to_string();
                            if let Ok(tree) = self.get_commit_diff(&hash_oid) {
                                result.push(tree)
                            }
                            commits = match commits.parent(0) {
                                Ok(commit) => commit,
                                Err(_) => break,
                            };
                        }
                    }
                }
            }
        }
        Ok(result)
    }
    pub fn commit_tree(&self, commit_hash: &str) -> JZResult<Commits> {
        let mut tree_list = self.commit_list()?;
        tree_list.sort_by(|a, b| b.time.cmp(&a.time));
        if let Some(tree) = tree_list
            .split(|x| x.hash_oid == commit_hash)
            .map(|x| x.to_vec())
            .last()
        {
            println!("Lens {:?}", tree.len())
        }
        Ok(tree_list.last().unwrap().clone())
    }
    pub fn build_tree_msg(
        &self,
        branch: String,
        commit_hash: Option<String>,
    ) -> JZResult<BlobTreeMsg> {
        let branch = self.repository.find_branch(&*branch, BranchType::Local)?;
        let commits = self.commit_list()?;
        let tree = self.branch_tree(&branch.into_reference(), commit_hash)?;
        Ok(Self::_build_tree_msg(commits, tree))
    }
    fn _build_tree_msg(commits: Vec<Commits>, tree: BlobTree) -> BlobTreeMsg {
        let mut root = BlobTreeMsg {
            name: tree.name,
            path: tree.path.clone(),
            msg: "".to_string(),
            time: 0,
            is_dir: tree.is_dir,
            size: 0,
            children: vec![],
        };
        let mut children = vec![];
        if tree.is_dir {
            for child in tree.children {
                let mut child_tree = Self::_build_tree_msg(commits.clone(), child);
                for commit in commits.clone() {
                    for tr in commit.tree.clone().children {
                        if child_tree.path == tr.path {
                            child_tree.msg = commit.msg.clone();
                            child_tree.time = commit.time;
                            child_tree.size = tr.size;
                            break;
                        }
                    }
                }
                children.push(child_tree);
            }
        }
        for commit in commits {
            for tr in commit.tree.clone().children {
                if commit.tree.path == tree.path {
                    root.msg = commit.msg.clone();
                    root.time = commit.time;
                    root.size = tr.size;
                    break;
                }
            }
        }
        root.children = children;
        root
    }
}
