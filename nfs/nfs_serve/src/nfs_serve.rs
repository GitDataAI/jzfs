#![allow(dead_code,unused_imports)]

use crate::nfs::{
    fattr3, fileid3, filename3, ftype3, nfspath3, nfsstat3, nfstime3, sattr3, set_mode3, set_size3,
    specdata3,
};
use crate::vfs::{NFSFileSystem, ReadDirResult, VFSCapabilities};
use async_trait::async_trait;
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use std::time::{SystemTime, UNIX_EPOCH};

// Conditional imports for platform-specific code
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, PermissionsExt};
#[cfg(windows)]
use std::os::windows::fs::MetadataExt as WindowsMetadataExt;

#[derive(Debug)]
pub struct Storage {
    pub local_path: PathBuf,
    rootdir: fileid3,
    id_to_path: Arc<Mutex<HashMap<fileid3, PathBuf>>>,
    path_to_id: Arc<Mutex<HashMap<PathBuf, fileid3>>>,
    next_id: Arc<Mutex<fileid3>>,
}

impl Storage {
    pub fn new(local_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let rootdir = 1u64; // Root directory always gets ID 1
        let mut id_to_path = HashMap::new();
        let mut path_to_id = HashMap::new();

        // Initialize root directory mapping
        id_to_path.insert(rootdir, local_path.clone());
        path_to_id.insert(local_path.clone(), rootdir);

        Ok(Storage {
            local_path,
            rootdir,
            id_to_path: Arc::new(Mutex::new(id_to_path)),
            path_to_id: Arc::new(Mutex::new(path_to_id)),
            next_id: Arc::new(Mutex::new(2)), // Start from 2, as 1 is root
        })
    }

    fn get_path_for_id(&self, id: fileid3) -> Option<PathBuf> {
        self.id_to_path.lock().unwrap().get(&id).cloned()
    }

    fn get_id_for_path(&self, path: &Path) -> Option<fileid3> {
        self.path_to_id.lock().unwrap().get(path).cloned()
    }

    fn register_path(&self, path: PathBuf) -> fileid3 {
        let mut path_to_id = self.path_to_id.lock().unwrap();
        let mut id_to_path = self.id_to_path.lock().unwrap();
        let mut next_id = self.next_id.lock().unwrap();

        if let Some(existing_id) = path_to_id.get(&path) {
            return *existing_id;
        }

        let id = *next_id;
        *next_id += 1;

        path_to_id.insert(path.clone(), id);
        id_to_path.insert(id, path);

        id
    }

    #[cfg(unix)]
    fn metadata_to_fattr3(&self, metadata: &std::fs::Metadata, id: fileid3) -> fattr3 {
        let ftype = if metadata.is_dir() {
            ftype3::NF3DIR
        } else if metadata.is_file() {
            ftype3::NF3REG
        } else if metadata.file_type().is_symlink() {
            ftype3::NF3LNK
        } else {
            ftype3::NF3REG // Default fallback
        };

        fattr3 {
            ftype,
            mode: metadata.mode(),
            nlink: metadata.nlink() as u32,
            uid: metadata.uid(),
            gid: metadata.gid(),
            size: metadata.size(),
            used: metadata.blocks() * 512, // More accurate on Unix
            rdev: specdata3 {
                specdata1: (metadata.rdev() >> 32) as u32,
                specdata2: (metadata.rdev() & 0xFFFFFFFF) as u32,
            },
            fsid: 1, // Filesystem ID
            fileid: id,
            atime: nfstime3 {
                seconds: metadata.atime() as u32,
                nseconds: metadata.atime_nsec() as u32,
            },
            mtime: nfstime3 {
                seconds: metadata.mtime() as u32,
                nseconds: metadata.mtime_nsec() as u32,
            },
            ctime: nfstime3 {
                seconds: metadata.ctime() as u32,
                nseconds: metadata.ctime_nsec() as u32,
            },
        }
    }

    #[cfg(windows)]
    fn metadata_to_fattr3(&self, metadata: &std::fs::Metadata, id: fileid3) -> fattr3 {
        let ftype = if metadata.is_dir() {
            ftype3::NF3DIR
        } else if metadata.is_file() {
            ftype3::NF3REG
        } else if metadata.file_type().is_symlink() {
            ftype3::NF3LNK
        } else {
            ftype3::NF3REG // Default fallback
        };

        // Convert Windows file times to Unix timestamp
        let creation_time = metadata.creation_time();
        let last_access_time = metadata.last_access_time();
        let last_write_time = metadata.last_write_time();

        // Windows time is in 100-nanosecond intervals since January 1, 1601 UTC
        // Convert to seconds since epoch (January 1, 1970)
        // 116444736000000000 is the offset between 1601 and 1970
        let to_unix_time = |win_time: u64| -> (u32, u32) {
            if win_time == 0 {
                return (0, 0);
            }

            let unix_time = (win_time - 116444736000000000) / 10000000;
            let seconds = unix_time as u32;
            let nseconds = ((win_time - 116444736000000000) % 10000000) * 100;
            (seconds, nseconds as u32)
        };

        let (ctime_sec, ctime_nsec) = to_unix_time(creation_time);
        let (atime_sec, atime_nsec) = to_unix_time(last_access_time);
        let (mtime_sec, mtime_nsec) = to_unix_time(last_write_time);

        fattr3 {
            ftype,
            mode: 0o755, // Default permissions for Windows
            nlink: 1,    // Windows doesn't expose link count easily
            uid: 0,      // Default UID for Windows
            gid: 0,      // Default GID for Windows
            size: metadata.file_size(),
            used: metadata.file_size(), // Simplified for Windows
            rdev: specdata3 {
                specdata1: 0,
                specdata2: 0,
            },
            fsid: 1, // Filesystem ID
            fileid: id,
            atime: nfstime3 {
                seconds: atime_sec,
                nseconds: atime_nsec,
            },
            mtime: nfstime3 {
                seconds: mtime_sec,
                nseconds: mtime_nsec,
            },
            ctime: nfstime3 {
                seconds: ctime_sec,
                nseconds: ctime_nsec,
            },
        }
    }

    #[cfg(unix)]
    fn apply_sattr3(&self, path: &Path, setattr: &sattr3) -> Result<(), nfsstat3> {
        match setattr.mode {
            set_mode3::Void => {}
            set_mode3::mode(mode) => {
                fs::set_permissions(path, std::fs::Permissions::from_mode(mode))
                    .map_err(|_| nfsstat3::NFS3ERR_IO)?;
            }
        }

        match setattr.size {
            set_size3::Void => {}
            set_size3::size(size) => {
                let file = OpenOptions::new()
                    .write(true)
                    .open(path)
                    .map_err(|_| nfsstat3::NFS3ERR_IO)?;
                file.set_len(size).map_err(|_| nfsstat3::NFS3ERR_IO)?;
            }
        }
        // Note: uid/gid changes would require additional privileges

        Ok(())
    }

    #[cfg(windows)]
    fn apply_sattr3(&self, path: &Path, setattr: &sattr3) -> Result<(), nfsstat3> {
        // Windows doesn't support Unix-style permissions
        // We only handle file size changes

        match setattr.size {
            set_size3::Void => {}
            set_size3::size(size) => {
                let file = OpenOptions::new()
                    .write(true)
                    .open(path)
                    .map_err(|_| nfsstat3::NFS3ERR_IO)?;
                file.set_len(size).map_err(|_| nfsstat3::NFS3ERR_IO)?;
            }
        }

        Ok(())
    }
}

#[async_trait]
impl NFSFileSystem for Storage {
    fn capabilities(&self) -> VFSCapabilities {
        VFSCapabilities::ReadWrite
    }

    fn root_dir(&self) -> fileid3 {
        self.rootdir
    }

    async fn lookup(&self, dirid: fileid3, filename: &filename3) -> Result<fileid3, nfsstat3> {
        let dir_path = self.get_path_for_id(dirid).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let target_path = dir_path.join(filename.to_string().replace("\"",""));

        // Check if file exists
        if !target_path.exists() {
            return Err(nfsstat3::NFS3ERR_NOENT);
        }

        // Get or create ID for this path
        let id = if let Some(existing_id) = self.get_id_for_path(&target_path) {
            existing_id
        } else {
            self.register_path(target_path)
        };

        Ok(id)
    }

    async fn getattr(&self, id: fileid3) -> Result<fattr3, nfsstat3> {
        let path = self.get_path_for_id(id).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let metadata = fs::metadata(&path).map_err(|_| nfsstat3::NFS3ERR_NOENT)?;

        Ok(self.metadata_to_fattr3(&metadata, id))
    }

    async fn setattr(&self, id: fileid3, setattr: sattr3) -> Result<fattr3, nfsstat3> {
        let path = self.get_path_for_id(id).ok_or(nfsstat3::NFS3ERR_STALE)?;

        self.apply_sattr3(&path, &setattr)?;

        // Return updated attributes
        let metadata = fs::metadata(&path).map_err(|_| nfsstat3::NFS3ERR_IO)?;

        Ok(self.metadata_to_fattr3(&metadata, id))
    }

    async fn read(
        &self,
        id: fileid3,
        offset: u64,
        count: u32,
    ) -> Result<(Vec<u8>, bool), nfsstat3> {
        let path = self.get_path_for_id(id).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let mut file = File::open(&path).map_err(|_| nfsstat3::NFS3ERR_NOENT)?;

        file.seek(SeekFrom::Start(offset))
            .map_err(|_| nfsstat3::NFS3ERR_IO)?;

        let mut buffer = vec![0u8; count as usize];
        let bytes_read = file.read(&mut buffer).map_err(|_| nfsstat3::NFS3ERR_IO)?;

        buffer.truncate(bytes_read);
        let eof = bytes_read < count as usize;

        Ok((buffer, eof))
    }

    async fn write(&self, id: fileid3, offset: u64, data: &[u8]) -> Result<fattr3, nfsstat3> {
        let path = self.get_path_for_id(id).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .map_err(|_| nfsstat3::NFS3ERR_IO)?;

        file.seek(SeekFrom::Start(offset))
            .map_err(|_| nfsstat3::NFS3ERR_IO)?;

        file.write_all(data).map_err(|_| nfsstat3::NFS3ERR_IO)?;

        file.flush().map_err(|_| nfsstat3::NFS3ERR_IO)?;

        // Return updated attributes
        let metadata = fs::metadata(&path).map_err(|_| nfsstat3::NFS3ERR_IO)?;

        Ok(self.metadata_to_fattr3(&metadata, id))
    }

    async fn create(
        &self,
        dirid: fileid3,
        filename: &filename3,
        attr: sattr3,
    ) -> Result<(fileid3, fattr3), nfsstat3> {
        let dir_path = self.get_path_for_id(dirid).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let file_path = dir_path.join(filename.to_string().replace("\"",""));

        // Create the file
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::AlreadyExists => nfsstat3::NFS3ERR_EXIST,
                _ => nfsstat3::NFS3ERR_IO,
            })?;

        // Apply initial attributes if specified
        if let set_size3::size(size) = attr.size {
            file.set_len(size).map_err(|_| nfsstat3::NFS3ERR_IO)?;
        }

        drop(file); // Close the file before setting permissions

        // Apply other attributes
        self.apply_sattr3(&file_path, &attr)?;

        // Register the new file and get its ID
        let id = self.register_path(file_path.clone());

        // Get final attributes
        let metadata = fs::metadata(&file_path).map_err(|_| nfsstat3::NFS3ERR_IO)?;

        let fattr = self.metadata_to_fattr3(&metadata, id);

        Ok((id, fattr))
    }

    async fn create_exclusive(
        &self,
        dirid: fileid3,
        filename: &filename3,
    ) -> Result<fileid3, nfsstat3> {
        let dir_path = self.get_path_for_id(dirid).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let file_path = dir_path.join(filename.to_string().replace("\"",""));
        // Create the file exclusively
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path)
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::AlreadyExists => nfsstat3::NFS3ERR_EXIST,
                _ => nfsstat3::NFS3ERR_IO,
            })?;

        // Register the new file and get its ID
        let id = self.register_path(file_path);

        Ok(id)
    }

    async fn mkdir(
        &self,
        dirid: fileid3,
        dirname: &filename3,
    ) -> Result<(fileid3, fattr3), nfsstat3> {
        let parent_path = self.get_path_for_id(dirid).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let dir_path = parent_path.join(dirname.to_string());

        // Create the directory
        fs::create_dir(&dir_path).map_err(|e| match e.kind() {
            std::io::ErrorKind::AlreadyExists => nfsstat3::NFS3ERR_EXIST,
            _ => nfsstat3::NFS3ERR_IO,
        })?;

        // Register the new directory and get its ID
        let id = self.register_path(dir_path.clone());

        // Get attributes
        let metadata = fs::metadata(&dir_path).map_err(|_| nfsstat3::NFS3ERR_IO)?;

        let fattr = self.metadata_to_fattr3(&metadata, id);

        Ok((id, fattr))
    }

    async fn remove(&self, dirid: fileid3, filename: &filename3) -> Result<(), nfsstat3> {
        let dir_path = self.get_path_for_id(dirid).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let target_path = dir_path.join(filename.to_string().replace("\"",""));

        if !target_path.exists() {
            return Err(nfsstat3::NFS3ERR_NOENT);
        }

        // Remove file or directory
        if target_path.is_dir() {
            fs::remove_dir_all(&target_path).map_err(|_| nfsstat3::NFS3ERR_IO)?;
        } else {
            fs::remove_file(&target_path).map_err(|_| nfsstat3::NFS3ERR_IO)?;
        }

        // Remove from our mappings
        if let Some(id) = self.get_id_for_path(&target_path) {
            self.id_to_path.lock().unwrap().remove(&id);
            self.path_to_id.lock().unwrap().remove(&target_path);
        }

        Ok(())
    }

    async fn rename(
        &self,
        from_dirid: fileid3,
        from_filename: &filename3,
        to_dirid: fileid3,
        to_filename: &filename3,
    ) -> Result<(), nfsstat3> {
        let from_dir = self
            .get_path_for_id(from_dirid)
            .ok_or(nfsstat3::NFS3ERR_STALE)?;
        let to_dir = self
            .get_path_for_id(to_dirid)
            .ok_or(nfsstat3::NFS3ERR_STALE)?;

        let from_path = from_dir.join(from_filename.to_string().replace("\"",""));
        let to_path = to_dir.join(to_filename.to_string().replace("\"",""));

        if !from_path.exists() {
            return Err(nfsstat3::NFS3ERR_NOENT);
        }

        // Perform the rename
        fs::rename(&from_path, &to_path).map_err(|_| nfsstat3::NFS3ERR_IO)?;

        // Update our mappings
        if let Some(id) = self.get_id_for_path(&from_path) {
            let mut id_to_path = self.id_to_path.lock().unwrap();
            let mut path_to_id = self.path_to_id.lock().unwrap();

            path_to_id.remove(&from_path);
            id_to_path.insert(id, to_path.clone());
            path_to_id.insert(to_path, id);
        }

        Ok(())
    }

    async fn readdir(
        &self,
        dirid: fileid3,
        start_after: fileid3,
        max_entries: usize,
    ) -> Result<ReadDirResult, nfsstat3> {
        let dir_path = self.get_path_for_id(dirid).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let entries = fs::read_dir(&dir_path).map_err(|_| nfsstat3::NFS3ERR_NOTDIR)?;

        let mut results = Vec::new();
        let mut found_start = start_after == 0; // If start_after is 0, start from beginning

        for entry in entries {
            if results.len() >= max_entries {
                break;
            }

            let entry = entry.map_err(|_| nfsstat3::NFS3ERR_IO)?;
            let entry_path = entry.path();

            // Get or create ID for this entry
            let entry_id = if let Some(existing_id) = self.get_id_for_path(&entry_path) {
                existing_id
            } else {
                self.register_path(entry_path.clone())
            };

            // Skip entries until we find our starting point
            if !found_start {
                if entry_id == start_after {
                    found_start = true;
                }
                continue;
            }

            let filename = entry.file_name().to_string_lossy().to_string().replace("\"","");
            let metadata = entry.metadata().map_err(|_| nfsstat3::NFS3ERR_IO)?;
            let fattr = self.metadata_to_fattr3(&metadata, entry_id);

            results.push(crate::vfs::DirEntry {
                fileid: entry_id,
                name: filename3::from(filename.as_bytes()),
                attr: fattr,
            });
        }

        Ok(ReadDirResult {
            entries: results,
            end: true, // Simplified - in practice you'd track this properly
        })
    }

    async fn symlink(
        &self,
        dirid: fileid3,
        linkname: &filename3,
        symlink: &nfspath3,
        attr: &sattr3,
    ) -> Result<(fileid3, fattr3), nfsstat3> {
        let dir_path = self.get_path_for_id(dirid).ok_or(nfsstat3::NFS3ERR_STALE)?;
        let link_path = dir_path.join(linkname.to_string());

        // Platform-specific symlink creation
        #[cfg(unix)]
        std::os::unix::fs::symlink(symlink.to_string(), &link_path)
            .map_err(|_| nfsstat3::NFS3ERR_IO)?;

        #[cfg(windows)]
        {
            let target = symlink.to_string();
            let is_dir = Path::new(&target).is_dir();

            // Windows requires administrator privileges for symlinks
            // and differentiates between file and directory symlinks
            if is_dir {
                std::os::windows::fs::symlink_dir(target, &link_path)
                    .map_err(|_| nfsstat3::NFS3ERR_IO)?;
            } else {
                std::os::windows::fs::symlink_file(target, &link_path)
                    .map_err(|_| nfsstat3::NFS3ERR_IO)?;
            }
        }

        // Apply attributes if needed
        self.apply_sattr3(&link_path, attr)?;

        // Register the new symlink and get its ID
        let id = self.register_path(link_path.clone());

        // Get attributes - use symlink_metadata to not follow the link
        let metadata = fs::symlink_metadata(&link_path)
            .map_err(|_| nfsstat3::NFS3ERR_IO)?;

        let fattr = self.metadata_to_fattr3(&metadata, id);

        Ok((id, fattr))
    }

    async fn readlink(&self, id: fileid3) -> Result<nfspath3, nfsstat3> {
        let path = self.get_path_for_id(id).ok_or(nfsstat3::NFS3ERR_STALE)?;

        let target = fs::read_link(&path).map_err(|_| nfsstat3::NFS3ERR_INVAL)?;

        Ok(nfspath3::from(
            target.to_string_lossy().to_string().as_bytes(),
        ))
    }
}
