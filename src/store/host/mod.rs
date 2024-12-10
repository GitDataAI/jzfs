use std::fs::File;
use std::io::{Read, Seek, Write};
use crate::store::inode::{RepoFileTrait, DATA_PATH, IDX};

pub struct HostRepoFile{
    pub idx: IDX,
    pub fs: File,
}

impl RepoFileTrait for HostRepoFile {
    fn from_idx(value: IDX) -> anyhow::Result<Self> {
        if std::fs::read_dir(DATA_PATH).is_err(){
            std::fs::create_dir(DATA_PATH)?;
        }
        let path = format!("{}/{}",DATA_PATH,value.owner_id);
        if std::fs::read_dir(&path).is_err(){
            std::fs::create_dir(&path)?;
        }
        let path = format!("{}/{}/{}",path,value.owner_id,value.repo_uid);
        let fs = File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;
        Ok(Self{
            idx: value,
            fs,
        })
    }
    fn read(&mut self, offset: usize, size: usize) -> anyhow::Result<Vec<u8>> {
        self.fs.seek(std::io::SeekFrom::Start(offset as u64))?;
        let mut buf = vec![0; size];
        self.fs.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn write(&mut self, offset: usize, data: Vec<u8>) -> anyhow::Result<()> {
        self.fs.seek(std::io::SeekFrom::Start(offset as u64))?;
        self.fs.write_all(&data)?;
        Ok(())
    }
    fn clear(&mut self, offset: usize, size: usize) -> anyhow::Result<()> {
        self.fs.seek(std::io::SeekFrom::Start(offset as u64))?;
        self.fs.set_len(offset as u64 + size as u64)?;
        Ok(())
    }
}