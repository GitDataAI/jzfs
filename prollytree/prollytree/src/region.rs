use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;
use crate::chunk::Chunk;
use crate::rollinghash::RollingHash;

#[derive(Clone)]
pub struct FileRegion {
    pub start: u64,
    pub end: u64,
    pub overlap: usize
}

pub fn process_region<P: AsRef<Path>>(
    path: P,
    region: FileRegion,
    chunk_size_min: usize,
    chunk_size_max: usize,
    window_size: usize,
    mask_bits: u32,
    file_size: u64
) -> io::Result<Vec<Chunk>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let end_with_overlap = (region.end + region.overlap as u64).min(file_size);
    reader.seek(SeekFrom::Start(region.start))?;
    let region_size = (end_with_overlap - region.start) as usize;
    let mut buffer = vec![0u8; region_size];
    reader.read_exact(&mut buffer)?;
    let mut offset = region.start;
    let mut chunks = Vec::new();
    let mut i = 0;
    let mut hasher = RollingHash::new(window_size);
    let boundary_mask = (1 << mask_bits) - 1;
    while i < buffer.len() {
        let mut chunk_size = 0;
        let start_i = i;
        let mut found_boundary = false;
        hasher.reset();
        while i < buffer.len() {
            let byte = buffer[i];
            hasher.add(byte);
            i += 1;
            chunk_size += 1;
            if chunk_size >= chunk_size_max {
                found_boundary = true;
                break;
            }
            if chunk_size >= chunk_size_min && hasher.window.len() == hasher.window_size {
                if (hasher.hash() & boundary_mask) == 0 {
                    found_boundary = true;
                    break;
                }
            }
        }
        if !found_boundary && i == buffer.len() {
            chunk_size = i - start_i;
        }
        let chunk_data = &buffer[start_i..start_i + chunk_size];
        let chunk_hash = calculate_hash(chunk_data);
        let chunk = Chunk::new(offset, chunk_size, chunk_hash);
        chunks.push(chunk);
        offset += chunk_size as u64;
    }

    Ok(chunks)
}

pub fn calculate_hash(data: &[u8]) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut hasher = DefaultHasher::new();
    for &byte in data {
        hasher.write_u8(byte);
    }
    hasher.finish()
}

pub fn calculate_chunking_params(file_size_gb: f64, target_chunks: usize) -> (usize, u32) {
    let file_size_bytes = file_size_gb * 1024.0 * 1024.0 * 1024.0;
    let avg_chunk_size = (file_size_bytes / target_chunks as f64) as usize;
    let mask_bits = (avg_chunk_size as f64).log2().ceil() as u32;
    (avg_chunk_size, mask_bits)
}