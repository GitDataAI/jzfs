use git2::{BranchType, IndexConflict};
use crate::store::dto::ConflictsFiles;
use crate::store::host::GitLocal;

impl GitLocal {
    pub fn branchs(&self) -> anyhow::Result<Vec<String>> {
        let branch = self.repo.branches(None);
        if branch.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let branch = branch?;
        let branchs = branch.flatten()
            .map(|x|x.0)
            .map(|x| x.into_reference())
            .filter(|x|x.is_branch())
            .map(|x| x.name().unwrap().to_string())
            .map(|x| x.clone().split("refs/heads/")
                .map(|x|x.to_string())
                .collect::<Vec<String>>()
            )
            .map(|x| x[1].to_string())
            .collect::<Vec<_>>();
        Ok(branchs)
    }
    pub fn new_branches(&self, from: String, branch: String) -> anyhow::Result<()>{
        let branches = self.branchs()?;
        if branches.contains(&branch){
            return Err(anyhow::anyhow!("Branch already exists"))
        }
        if !branches.contains(&from){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let head = self.repo.find_branch(from.as_str(), BranchType::Local);
        if head.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let nb = self.repo.branch(branch.as_str(), &head?.into_reference().peel_to_commit()?, false);
        if nb.is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        Ok(())
    }
    pub fn del_branchs(&self, branch: String) -> anyhow::Result<()>{
        if self.repo.find_branch(branch.as_str(), BranchType::Local).is_err(){
            return Err(anyhow::anyhow!("Branch not found"))
        }
        let mut refs = self.repo.find_branch(branch.as_str(), BranchType::Local)?;
        refs.delete()?;
        Ok(())
    }
    
    pub fn check_conflicts(&self, branch: String, target: String) -> anyhow::Result<Vec<ConflictsFiles>>{
        
        self.repo.checkout_index(None, None).ok();
        
        let target_branch = self.repo.find_branch(&*target, BranchType::Local)?.into_reference();
        let target_commit = target_branch.peel_to_commit()?;

        let source_branch = self.repo.find_branch(&*branch, BranchType::Local)?.into_reference();
        let source_commit = source_branch.peel_to_commit()?;
        
        let merge = self.repo.merge_commits(
            &source_commit,
            &target_commit,
            None
        );
        if merge.is_err(){
            return Err(anyhow::anyhow!("Branch Check Error: {:?}", merge.err()))
        }
        let index = merge?;
        if index.has_conflicts() {
            let conflicts = index.conflicts()?
                .into_iter()
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .map(|x| ConflictsFiles{
                    ours: std::str::from_utf8(&x.our.unwrap().path).unwrap().to_string(),
                    theirs: std::str::from_utf8(&x.their.unwrap().path).unwrap().to_string()
                })
                .collect::<Vec<_>>();
            Ok(conflicts)
        } else {
            Err(anyhow::anyhow!("No Conflicts"))
        }
    }
    pub fn merge(&self, branch: String, target: String) -> anyhow::Result<()>{
        if self.check_conflicts(branch.clone(), target.clone()).is_ok(){
            return Err(anyhow::anyhow!("Conflicts"))
        }
        let target_branch = self.repo.find_reference(&*target)?;
        let target_commit = target_branch.peel_to_commit()?;
        let source_branch = self.repo.find_reference(&*branch)?;
        let source_commit = source_branch.peel_to_commit()?;
        
        self.repo.merge_commits(
            &source_commit,
            &target_commit,
            None
        )?;

        Ok(())
    }
    pub fn rename(&self, branch: String, new_name: String) -> anyhow::Result<()>{
        let branchs = self.branchs()?;
        if branchs.contains(&new_name){
            return Err(anyhow::anyhow!("Branch already exists"))
        }
        let mut branch = self.repo.find_branch(&*branch, BranchType::Local)?;
        branch.rename(&*new_name, true)?;
        Ok(())
    }
}