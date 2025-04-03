// use std::process::Command;
// use crate::GitParam;
// 
// impl GitParam {
//     pub fn clone(&mut self, branch: Vec<String>, other: GitParam) -> anyhow::Result<()> {
//         let repo = self.root.join(self.uid.clone());
//         let target = other.root.join(other.uid.clone());
//         let clone = Command::new("git")
//             .args(&["clone", repo.to_str().ok_or(anyhow::anyhow!(""))?])
//             .output()
//             .expect("failed to execute process");
//         Ok(())
//     }
// }