use prollytree::tree::ProllyTree;

pub fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    let mut prolly_tree = ProllyTree::new(14376937, 64, 24,8);
    let file_path = "E:\\Vmware\\Fedora-Server-dvd-x86_64-42-1.1.iso";
    let chunks = prolly_tree.process_file(file_path)?;
    println!("文件被分成了 {} 个块:", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        println!("块 #{}: 偏移量={}, 大小={} MB, 哈希={:x}", i, chunk.offset(), chunk.size() / 1024 / 1024, chunk.hash());
    }
    println!("Root Hash: {}", prolly_tree.root_hash());
    println!("分块计算耗时: {:?}", start.elapsed());
    Ok(())
}