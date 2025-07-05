use crate::nfs::*;
use async_trait::async_trait;
use std::cmp::Ordering;
use std::sync::Once;
use std::time::SystemTime;
#[derive(Default, Debug)]
pub struct DirEntrySimple {
    pub fileid: fileid3,
    pub name: filename3,
}
#[derive(Default, Debug)]
pub struct ReadDirSimpleResult {
    pub entries: Vec<DirEntrySimple>,
    pub end: bool,
}

#[derive(Default, Debug)]
pub struct DirEntry {
    pub fileid: fileid3,
    pub name: filename3,
    pub attr: fattr3,
}
#[derive(Default, Debug)]
pub struct ReadDirResult {
    pub entries: Vec<DirEntry>,
    pub end: bool,
}

impl ReadDirSimpleResult {
    fn from_readdir_result(result: &ReadDirResult) -> ReadDirSimpleResult {
        let entries: Vec<DirEntrySimple> = result
            .entries
            .iter()
            .map(|e| DirEntrySimple {
                fileid: e.fileid,
                name: e.name.clone(),
            })
            .collect();
        ReadDirSimpleResult {
            entries,
            end: result.end,
        }
    }
}

static mut GENERATION_NUMBER: u64 = 0;
static GENERATION_NUMBER_INIT: Once = Once::new();

fn get_generation_number() -> u64 {
    unsafe {
        GENERATION_NUMBER_INIT.call_once(|| {
            GENERATION_NUMBER = SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
        });
        GENERATION_NUMBER
    }
}

/// 支持的功能特性
pub enum VFSCapabilities {
    ReadOnly,
    ReadWrite,
}

/// 实现NFS文件系统所需的基本API
///
/// 不透明文件句柄(Opaque FH)
/// ------------------
/// 文件仅通过64位文件ID唯一标识(基本上等同于inode编号)
/// 我们在内部自动生成不透明文件句柄，它由以下部分组成：
///  - 64位生成号，源自服务器启动时间
///    (即当NFS服务器重启时，不透明文件句柄会过期)
///  - 64位文件ID
//
/// readdir分页
/// ------------------
/// 我们不使用cookie验证器，仅使用start_after参数。
/// 实现应允许从任意位置开始读取。也就是说，
/// 下一次readdir查询可能从上次readdir响应的最后一个条目开始。
//
/// readdir有一个奇怪的限制，它限制响应中的字节数(而不是条目数)。
/// 调用者必须截断readdir响应或发出更多readdir调用来填充预期的字节数，而不超过限制。
//
/// 其他要求
/// ------------------
///  getattr需要快速执行，NFS经常使用它
//
/// 0文件ID是保留的，不应使用
///
#[async_trait]
pub trait NFSFileSystem: Sync {
    /// 返回支持的功能特性集
    fn capabilities(&self) -> VFSCapabilities;
    /// 返回根目录"/"的ID
    fn root_dir(&self) -> fileid3;
    /// 在目录中查找路径的ID
    ///
    /// 例如，给定包含文件a.txt的目录dir/
    /// 可以调用lookup(id_of("dir/"), "a.txt")
    /// 这应该返回文件"dir/a.txt"的ID
    ///
    /// 此方法应快速执行，因为它使用非常频繁。
    async fn lookup(&self, dirid: fileid3, filename: &filename3) -> Result<fileid3, nfsstat3>;

    /// 返回ID对应的属性
    /// 此方法应快速执行，因为它使用非常频繁。
    async fn getattr(&self, id: fileid3) -> Result<fattr3, nfsstat3>;

    /// 设置ID对应的属性
    /// 如果是只读文件系统，应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn setattr(&self, id: fileid3, setattr: sattr3) -> Result<fattr3, nfsstat3>;

    /// 读取文件内容，返回(字节数据, EOF)
    /// 注意，偏移量/计数可能超过文件末尾，
    /// 在这种情况下，将返回文件末尾之前的所有字节。
    /// 如果读取到达文件末尾，必须标记EOF。
    async fn read(&self, id: fileid3, offset: u64, count: u32)
    -> Result<(Vec<u8>, bool), nfsstat3>;

    /// 写入文件内容，返回(字节数据, EOF)
    /// 注意，偏移量/计数可能超过文件末尾，
    /// 在这种情况下，文件将被扩展。
    /// 如果由于只读文件系统而不支持写入，
    /// 应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn write(&self, id: fileid3, offset: u64, data: &[u8]) -> Result<fattr3, nfsstat3>;

    /// 使用指定属性创建文件
    /// 如果由于只读文件系统而不支持创建，
    /// 应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn create(
        &self,
        dirid: fileid3,
        filename: &filename3,
        attr: sattr3,
    ) -> Result<(fileid3, fattr3), nfsstat3>;

    /// 仅在文件不存在时创建文件
    /// 如果是只读文件系统，应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn create_exclusive(
        &self,
        dirid: fileid3,
        filename: &filename3,
    ) -> Result<fileid3, nfsstat3>;

    /// 使用指定属性创建目录
    /// 如果由于只读文件系统而不支持创建，
    /// 应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn mkdir(
        &self,
        dirid: fileid3,
        dirname: &filename3,
    ) -> Result<(fileid3, fattr3), nfsstat3>;

    /// 删除文件
    /// 如果由于只读文件系统而不支持删除，
    /// 应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn remove(&self, dirid: fileid3, filename: &filename3) -> Result<(), nfsstat3>;

    /// 重命名文件
    /// 如果由于只读文件系统而不支持重命名，
    /// 应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn rename(
        &self,
        from_dirid: fileid3,
        from_filename: &filename3,
        to_dirid: fileid3,
        to_filename: &filename3,
    ) -> Result<(), nfsstat3>;

    /// 返回带分页的目录内容
    /// 目录列表应具有确定性
    /// 最多可返回max_entries个条目，start_after用于
    /// 确定从哪个位置开始返回条目
    ///
    /// 例如，如果目录条目ID为[1,6,2,11,8,9]
    /// 且start_after=6，则readdir应返回2,11,8,...
    //
    async fn readdir(
        &self,
        dirid: fileid3,
        start_after: fileid3,
        max_entries: usize,
    ) -> Result<ReadDirResult, nfsstat3>;

    /// readdir的简化版本
    /// 只需返回文件名和ID
    async fn readdir_simple(
        &self,
        dirid: fileid3,
        count: usize,
    ) -> Result<ReadDirSimpleResult, nfsstat3> {
        Ok(ReadDirSimpleResult::from_readdir_result(
            &self.readdir(dirid, 0, count).await?,
        ))
    }

    /// 使用指定属性创建符号链接
    /// 如果由于只读文件系统而不支持创建，
    /// 应返回Err(nfsstat3::NFS3ERR_ROFS)
    async fn symlink(
        &self,
        dirid: fileid3,
        linkname: &filename3,
        symlink: &nfspath3,
        attr: &sattr3,
    ) -> Result<(fileid3, fattr3), nfsstat3>;

    /// 读取符号链接
    async fn readlink(&self, id: fileid3) -> Result<nfspath3, nfsstat3>;

    /// 获取静态文件系统信息
    async fn fsinfo(&self, root_fileid: fileid3) -> Result<fsinfo3, nfsstat3> {
        let dir_attr: post_op_attr = match self.getattr(root_fileid).await {
            Ok(v) => post_op_attr::attributes(v),
            Err(_) => post_op_attr::Void,
        };

        let res = fsinfo3 {
            obj_attributes: dir_attr,
            rtmax: 1024 * 1024,
            rtpref: 1024 * 124,
            rtmult: 1024 * 1024,
            wtmax: 1024 * 1024,
            wtpref: 1024 * 1024,
            wtmult: 1024 * 1024,
            dtpref: 1024 * 1024,
            maxfilesize: 128 * 1024 * 1024 * 1024,
            time_delta: nfstime3 {
                seconds: 0,
                nseconds: 1000000,
            },
            properties: FSF_SYMLINK | FSF_HOMOGENEOUS | FSF_CANSETTIME,
        };
        Ok(res)
    }

    /// 将fileid转换为不透明的NFS文件句柄。可选实现。
    fn id_to_fh(&self, id: fileid3) -> nfs_fh3 {
        let gennum = get_generation_number();
        let mut ret: Vec<u8> = Vec::new();
        ret.extend_from_slice(&gennum.to_le_bytes());
        ret.extend_from_slice(&id.to_le_bytes());
        nfs_fh3 { data: ret }
    }
    /// 将不透明的NFS文件句柄转换为fileid。可选实现。
    fn fh_to_id(&self, id: &nfs_fh3) -> Result<fileid3, nfsstat3> {
        if id.data.len() != 16 {
            return Err(nfsstat3::NFS3ERR_BADHANDLE);
        }
        let gens = u64::from_le_bytes(id.data[0..8].try_into().unwrap());
        let id = u64::from_le_bytes(id.data[8..16].try_into().unwrap());
        let gennum = get_generation_number();
        match gens.cmp(&gennum) {
            Ordering::Less => Err(nfsstat3::NFS3ERR_STALE),
            Ordering::Greater => Err(nfsstat3::NFS3ERR_BADHANDLE),
            Ordering::Equal => Ok(id),
        }
    }
    /// 将完整路径转换为fileid。可选实现。
    /// 默认实现使用lookup()遍历目录结构
    async fn path_to_id(&self, path: &[u8]) -> Result<fileid3, nfsstat3> {
        let splits = path.split(|&r| r == b'/');
        let mut fid = self.root_dir();
        for component in splits {
            if component.is_empty() {
                continue;
            }
            fid = self.lookup(fid, &component.into()).await?;
        }
        Ok(fid)
    }

    fn serverid(&self) -> cookieverf3 {
        let gennum = get_generation_number();
        gennum.to_le_bytes()
    }
}
