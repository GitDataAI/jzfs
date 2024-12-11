use std::io::ErrorKind;
use git2::Repository;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let repo = Repository::open("E:/harness").unwrap();
    let head = repo.head()?;
    let head_commit = repo.find_commit(head.target().unwrap());
    let tree = head_commit?.tree()?;
    tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
        println!("Root: {}, file: {}, id: {}",root, entry.name().unwrap_or("N/A"),entry.id());
        0
    }).unwrap();
    // for entry in tree.iter() {
    //     
    //     println!("name: {}, type: {:?}, sha: {}", entry.name().unwrap_or("N/A"), entry.kind(), entry.id());
    // }
    Ok(())
}