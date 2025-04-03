use anyhow::anyhow;
use git2::{MergeAnalysis, MergePreference};
use git2::build::CheckoutBuilder;
use serde::{Deserialize, Serialize};
use crate::GitParam;

#[derive(Deserialize)]
pub struct GitPrTest {
    pub branch: String,
    pub target: String,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct GitPRTextResponse {
    pub status: GitPRStatus,
    pub merge_preference: GitPRMergePreference,
}

#[derive(Deserialize,Serialize,Debug)]
#[derive(PartialEq)]
pub enum GitPRStatus {
    CannotMerge,
    CanMerge,
    UpToDate,
    Unborn,
    Unknown,
}

#[derive(Deserialize,Serialize,Debug)]
pub enum GitPRMergePreference {
    None,
    NoFastForward,
    FastforwardOnly,
}

impl From<MergeAnalysis> for GitPRStatus {
    fn from(value: MergeAnalysis) -> Self {
        if value.is_none() {
            GitPRStatus::CannotMerge
        } else if value.is_normal() {
            GitPRStatus::CanMerge
        } else if value.is_up_to_date() {
            GitPRStatus::UpToDate
        } else if value.is_unborn() {
            GitPRStatus::Unborn
        } else {
            GitPRStatus::Unknown
        }
    }
}

impl From<MergePreference> for GitPRMergePreference {
    fn from(value: MergePreference) -> Self {
        if value.is_none() {
            GitPRMergePreference::None
        } else if value.is_no_fast_forward() {
            GitPRMergePreference::NoFastForward
        } else if value.is_fastforward_only() {
            GitPRMergePreference::FastforwardOnly
        } else {
            GitPRMergePreference::None
        }
    }
}

impl GitParam {
    pub fn pr_test(&mut self, param: GitPrTest) -> anyhow::Result<GitPRTextResponse> {
        let repo = self.repo()?;
        let branch = repo.find_branch(&param.branch, git2::BranchType::Local)?;
        let target = repo.find_branch(&param.target, git2::BranchType::Local)?;
        let branch_head = branch.into_reference();
        let target_head = target.into_reference();
        let target_head_commit = target_head.peel_to_commit()?;
        let target_annotated_commit = repo.find_annotated_commit(target_head_commit.id())?;
        match repo.merge_analysis_for_ref(&branch_head, &[&target_annotated_commit]) {
            Ok((analysis, preference)) => {
                let status = GitPRStatus::from(analysis);
                let preference = GitPRMergePreference::from(preference);
                Ok(GitPRTextResponse {
                    status,
                    merge_preference: preference,
                })
            },
            Err(e) => {
                Err(anyhow!("merge analysis error: {}", e))
            }
        }
    }
    pub fn pr_merge(&mut self, param: GitPrTest) -> anyhow::Result<()> {
        let repo = self.repo()?;

        let branch = repo.find_branch(&param.branch, git2::BranchType::Local)?;
        let target = repo.find_branch(&param.target, git2::BranchType::Local)?;

        let branch_head = branch.get().peel_to_commit()?;
        let target_head = target.get().peel_to_commit()?;
        let target_annotated = repo.find_annotated_commit(target_head.id())?;

        let branch_ref_name = branch.get().name().ok_or_else(|| anyhow!("Invalid branch name"))?;
        repo.set_head(branch_ref_name)?;
        repo.checkout_head(Some(CheckoutBuilder::new().force()))?;

        match repo.merge_analysis(&[&target_annotated]) {
            Ok((analysis, _)) => {
                
                if analysis.is_normal() {
                    let mut merge_opts = git2::MergeOptions::new();
                    repo.merge(&[&target_annotated], Some(&mut merge_opts), None)?;

                    if repo.index()?.has_conflicts() {
                        return Err(anyhow!("Merge conflicts detected"));
                    }

                    let mut index = repo.index()?;
                    let tree_id = index.write_tree()?;
                    let tree = repo.find_tree(tree_id)?;
                    let signature = repo.signature()?;

                    repo.commit(
                        Some("HEAD"),
                        &signature,
                        &signature,
                        &format!("Merge {} into {}", param.target, param.branch),
                        &tree,
                        &[&branch_head, &target_head],
                    )?;

                    repo.cleanup_state()?;
                    Ok(())
                } else {
                    Err(anyhow!("Cannot merge: analysis result {:?}", analysis))
                }
            },
            Err(e) => Err(anyhow!("Merge analysis failed: {}", e)),
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::PathBuf;
    use super::*;
    use crate::GitParam;

    #[test]
    fn test_pr_test() {
        let mut git = GitParam::new(PathBuf::from("/home/zhenyi/文档"), "r-nacos".to_string()).unwrap();
        let param = GitPrTest {
            branch: "master".to_string(),
            target: "pr_ts".to_string(),
        };
        let result = git.pr_test(param);
        dbg!(result);
    }

    #[test]
    fn test_pr_merge() {
        let mut git = GitParam::new(PathBuf::from("/home/zhenyi/文档"), "r-nacos".to_string()).unwrap();
        let param = GitPrTest {
            branch: "master".to_string(),
            target: "pr_ts".to_string(),
        };
        let result = git.pr_merge(param);
        dbg!(result);
    }
}