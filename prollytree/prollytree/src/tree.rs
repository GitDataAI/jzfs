use crate::chunk::Chunk;
use crate::node::Node;
use crate::region::{process_region, FileRegion};
use crossbeam::{channel, scope};
use std::fs::File;
use std::io;
use std::path::Path;

pub struct ProllyTree {
    pub root: Node,
    pub chunk_size_avg: usize,
    pub chunk_size_min: usize,
    pub chunk_size_max: usize,
    pub window_size: usize,
    pub mask_bits: u32,
    pub thread_count: usize,
}

impl ProllyTree {
    pub fn new(chunk_size_avg: usize, window_size: usize, mask_bits: u32,thread_count: usize) -> Self {
        ProllyTree {
            root: Node::new(true),
            chunk_size_avg,
            chunk_size_min: chunk_size_avg / 4,
            chunk_size_max: chunk_size_avg * 4,
            window_size,
            mask_bits,
            thread_count,
        }
    }
    pub fn process_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<Vec<Chunk>> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let file_size = file.metadata()?.len();
        let regions = self.divide_file_regions(file_size);
        let mut all_chunks = scope(|s| {
            let (sender, receiver) = channel::unbounded();
            for region in regions {
                let sender = sender.clone();
                let chunk_size_min = self.chunk_size_min;
                let chunk_size_max = self.chunk_size_max;
                let window_size = self.window_size;
                let mask_bits = self.mask_bits;
                let path_clone = path.clone();

                s.spawn(move |_| {
                    let result = process_region(
                        path_clone,
                        region.clone(),
                        chunk_size_min,
                        chunk_size_max,
                        window_size,
                        mask_bits,
                        file_size
                    );

                    match result {
                        Ok(chunks) => {
                            if let Err(e) = sender.send((region.start.clone(), chunks)) {
                                eprintln!("发送区域处理结果失败: {}", e);
                            }
                        },
                        Err(e) => {
                            eprintln!("处理区域 [{}, {}] 时出错: {}", region.start.clone(), region.end.clone(), e);
                        }
                    }
                });
            }
            drop(sender);
            let mut region_chunks = Vec::new();
            while let Ok((region_start, chunks)) = receiver.recv() {
                region_chunks.push((region_start, chunks));
            }
            region_chunks.sort_by_key(|(start, _)| *start);
            self.merge_region_chunks(region_chunks, file_size)
        }).unwrap();
        all_chunks.sort_by_key(|chunk| chunk.offset);
        all_chunks.dedup_by_key(|chunk| (chunk.offset, chunk.size));
        self.build_tree(all_chunks.clone());
        Ok(all_chunks)
    }
    fn divide_file_regions(&self, file_size: u64) -> Vec<FileRegion> {
        let mut regions = Vec::new();
        let thread_count = self.thread_count.min(8);
        let region_size = file_size / thread_count as u64;
        let region_size = region_size.max(4 * self.chunk_size_max as u64);
        let region_count = (file_size / region_size).max(1) as usize;
        let overlap = self.chunk_size_max * 2;
        for i in 0..region_count {
            let start = if i == 0 { 0 } else { i as u64 * region_size };
            let end = if i == region_count - 1 {
                file_size
            } else {
                ((i + 1) as u64 * region_size).min(file_size)
            };

            regions.push(FileRegion {
                start,
                end,
                overlap,
            });
        }

        regions
    }

    fn merge_region_chunks(&self, region_chunks: Vec<(u64, Vec<Chunk>)>, _file_size: u64) -> Vec<Chunk> {
        let mut all_chunks = Vec::new();
        let mut last_end = 0;
        for (i, (_region_start, chunks)) in region_chunks.iter().enumerate() {
            if chunks.is_empty() {
                continue;
            }
            if i == 0 {
                all_chunks.extend(chunks.clone());
                if let Some(last_chunk) = chunks.last() {
                    last_end = last_chunk.offset + last_chunk.size as u64;
                }
            } else {
                for chunk in chunks {
                    if chunk.offset >= last_end {
                        all_chunks.push(chunk.clone());
                    }
                }
            }
            if let Some(last_chunk) = chunks.last() {
                last_end = last_chunk.offset + last_chunk.size as u64;
            }
        }

        all_chunks
    }

    fn build_tree(&mut self, chunks: Vec<Chunk>) {
        self.root = Node::new(true);
        self.root.chunks = chunks;
    }
    pub fn get_chunks(&self) -> Vec<Chunk> {
        self.root.chunks.clone()
    }
    pub fn find_chunk_by_offset(&self, offset: u64) -> Option<Chunk> {
        for chunk in &self.root.chunks {
            if chunk.offset <= offset && offset < chunk.offset + chunk.size as u64 {
                return Some(chunk.clone());
            }
        }
        None
    }

}