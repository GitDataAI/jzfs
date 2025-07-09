use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use prollytree::tree::ProllyTree;

pub fn main() -> std::io::Result<()> {
    let start = std::time::Instant::now();
    let mut prolly_tree = ProllyTree::new(10737418, 64, 24,8);
    let file_path = "E:\\Vmware\\Fedora-Server-dvd-x86_64-42-1.1.iso";
    let chunks = prolly_tree.process_file(file_path)?;
    println!("文件被分成了 {} 个块:", chunks.len());
    for (i, chunk) in chunks.iter().enumerate() {
        println!("块 #{}: 偏移量={}, 大小={}, 哈希={:x}",
                 i, chunk.offset(), chunk.size(), chunk.hash());
    }
    if let Some(chunk) = prolly_tree.find_chunk_by_offset(1024 * 1024) {
        println!("找到包含偏移量1MB的块: 偏移量={}, 大小={}", chunk.offset(), chunk.size());
        let mut file = File::open(file_path)?;
        let mut buffer = vec![0u8; chunk.size()];
        file.seek(SeekFrom::Start(chunk.offset()))?;
        file.read_exact(&mut buffer)?;
        println!("块前20字节: {:?}", &buffer[..20.min(buffer.len())]);
    }
    println!("分块计算耗时: {:?}", start.elapsed());
    Ok(())
}