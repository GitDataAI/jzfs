use gitdata::blob::GitBlob;

fn main() {
    let start = std::time::Instant::now();
    let blob = GitBlob::new("/home/zhenyi/文档/gitdata".into()).unwrap();
    let res = blob.tree("main".to_string()).unwrap();
    println!("{:?}", start.elapsed())
}