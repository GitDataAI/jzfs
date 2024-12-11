use git2::Repository;
use jzfs::store::host::GitLocal;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let repo = GitLocal{
        uid: "E:/harness".to_string(),
        repo: Repository::open("E:/harness")?
    };
    println!("{:?}", repo.branchs().unwrap());
    let result = repo.commits_history("main".to_string());
    println!("{:?}", result.unwrap());
    Ok(())
}