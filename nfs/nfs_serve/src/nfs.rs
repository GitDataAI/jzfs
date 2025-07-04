use crate::nfssting::nfsstring;
use crate::xdr::*;
use byteorder::{ReadBytesExt, WriteBytesExt};
use filetime;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::FromPrimitive;
use std::io::{Read, Write};

/*
| 类型名 | 作用 |
| --- | --- |
| `nfsstring` | 用于表示 NFS 字符串，内部存储为 `Vec<u8>`，并实现了多种常用的转换和操作方法 |
| `opaque` | 别名，实际为 `u8` 类型 |
| `filename3` | 别名，实际为 `nfsstring` 类型，用于表示文件名 |
| `nfspath3` | 别名，实际为 `nfsstring` 类型，用于表示 NFS 路径 |
| `fileid3` | 别名，实际为 `u64` 类型，用于表示文件 ID |
| `cookie3` | 别名，实际为 `u64` 类型，用于表示 cookie |
| `cookieverf3` | 别名，是长度为 `NFS3_COOKIEVERFSIZE` 的 `opaque` 数组，用于表示 cookie 验证器 |
| `createverf3` | 别名，是长度为 `NFS3_CREATEVERFSIZE` 的 `opaque` 数组，用于表示创建验证器 |
| `writeverf3` | 别名，是长度为 `NFS3_WRITEVERFSIZE` 的 `opaque` 数组，用于表示写操作验证器 |
| `uid3` | 别名，实际为 `u32` 类型，用于表示用户 ID |
| `gid3` | 别名，实际为 `u32` 类型，用于表示组 ID |
| `size3` | 别名，实际为 `u64` 类型，用于表示大小 |
| `offset3` | 别名，实际为 `u64` 类型，用于表示偏移量 |
| `mode3` | 别名，实际为 `u32` 类型，用于表示文件模式 |
| `count3` | 别名，实际为 `u32` 类型，用于表示计数 |
| `nfsstat3` | 表示 NFSv3 操作的状态码，每个枚举值代表一种操作结果 |
| `ftype3` | 表示文件类型，如普通文件、目录、设备文件等 |
| `specdata3` | 用于表示设备号信息，包含两个 `u32` 类型的字段 |
| `nfs_fh3` | 用于表示文件句柄信息，内部存储为 `Vec<u8>` |
| `nfstime3` | 用于表示 NFS 时间，包含秒和纳秒两个字段 |
| `fattr3` | 用于表示文件属性，包含文件类型、模式、链接数等多种属性 |
| `fsinfo3` | 用于表示文件系统信息，包含对象属性、读写参数、文件系统属性等 |
| `wcc_attr` | 用于表示写操作前后的文件属性变化，包含大小、修改时间和创建时间 |
| `pre_op_attr` | 用于表示操作前的文件属性，有 `Void` 和 `attributes` 两种状态 |
| `post_op_attr` | 用于表示操作后的文件属性，有 `Void` 和 `attributes` 两种状态 |
| `wcc_data` | 用于表示写操作前后的属性数据，包含操作前和操作后的属性 |
| `post_op_fh3` | 用于表示操作后的文件句柄，有 `Void` 和 `handle` 两种状态 |
| `_time_how` | 仅作为设置访问时间和修改时间的判别器，不应直接使用 |
| `set_mode3` | 用于表示设置文件模式的操作，有 `Void` 和 `mode` 两种状态 |
| `set_uid3` | 用于表示设置用户 ID 的操作，有 `Void` 和 `uid` 两种状态 |
| `set_gid3` | 用于表示设置组 ID 的操作，有 `Void` 和 `gid` 两种状态 |
| `set_size3` | 用于表示设置文件大小的操作，有 `Void` 和 `size` 两种状态 |
| `set_atime` | 用于表示设置文件访问时间的操作，有 `DONT_CHANGE`、`SET_TO_SERVER_TIME` 和 `SET_TO_CLIENT_TIME` 三种状态 |
| `set_mtime` | 用于表示设置文件修改时间的操作，有 `DONT_CHANGE`、`SET_TO_SERVER_TIME` 和 `SET_TO_CLIENT_TIME` 三种状态 |
| `sattr3` | 用于表示设置文件属性的参数，包含模式、用户 ID、组 ID、大小、访问时间和修改时间等设置 |
| `diropargs3` | 用于表示目录操作的参数，包含目录句柄和文件名 |
| `symlinkdata3` | 用于表示符号链接的数据，包含符号链接的属性和链接数据 |
| `get_root_mount_handle` | 定义根挂载句柄，返回 `vec![0]` |
*/

/// 调用 NFS 版本 3 服务所需的 RPC 程序号，以十进制表示。
pub const PROGRAM: u32 = 100003;
/// NFS 的版本号，此处为版本 3。
pub const VERSION: u32 = 3;

// Section 2.4 Sizes
//
/// 不透明文件句柄的最大字节大小。
pub const NFS3_FHSIZE: u32 = 64;

/// READDIR 和 READDIRPLUS 操作传递的不透明 cookie 验证器的字节大小。
pub const NFS3_COOKIEVERFSIZE: u32 = 8;

/// 用于独占创建操作的不透明验证器的字节大小。
pub const NFS3_CREATEVERFSIZE: u32 = 8;

/// 用于异步写操作的不透明验证器的字节大小。
pub const NFS3_WRITEVERFSIZE: u32 = 8;

// ... existing code ...

// Section 3.3.19. Procedure 19: FSINFO - Get static file system Information
// 以下常量用于 fsinfo 中构建位掩码 'properties'，该位掩码表示文件系统属性。

/// 如果该位为 1 (TRUE)，表示文件系统支持硬链接。
pub const FSF_LINK: u32 = 0x0001;

/// 如果该位为 1 (TRUE)，表示文件系统支持符号链接。
pub const FSF_SYMLINK: u32 = 0x0002;

/// 如果该位为 1 (TRUE)，表示文件系统中每个文件和目录的 PATHCONF 返回信息相同。
/// 如果为 0 (FALSE)，客户端应按需为每个文件和目录检索 PATHCONF 信息。
pub const FSF_HOMOGENEOUS: u32 = 0x0008;

/// 如果该位为 1 (TRUE)，服务器将通过 SETATTR 设置文件时间（达到 time_delta 指示的精度）。
/// 如果为 0 (FALSE)，服务器无法按请求设置时间。
pub const FSF_CANSETTIME: u32 = 0x0010;

pub type opaque = u8;
pub type filename3 = nfsstring;
pub type nfspath3 = nfsstring;
pub type fileid3 = u64;
pub type cookie3 = u64;
pub type cookieverf3 = [opaque; NFS3_COOKIEVERFSIZE as usize];
pub type createverf3 = [opaque; NFS3_CREATEVERFSIZE as usize];
pub type writeverf3 = [opaque; NFS3_WRITEVERFSIZE as usize];
pub type uid3 = u32;
pub type gid3 = u32;
pub type size3 = u64;
pub type offset3 = u64;
pub type mode3 = u32;
pub type count3 = u32;


#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum nfsstat3 {
    /// 表示调用成功完成。
    NFS3_OK = 0,
    /// 非所有者。由于调用者不是特权用户（root）或不是操作目标的所有者，操作未被允许。
    NFS3ERR_PERM = 1,
    /// 文件或目录不存在。指定的文件或目录名不存在。
    NFS3ERR_NOENT = 2,
    /// I/O 错误。处理请求操作时发生硬错误（例如磁盘错误）。
    NFS3ERR_IO = 5,
    /// I/O 错误。设备或地址不存在。
    NFS3ERR_NXIO = 6,
    /// 权限被拒绝。调用者没有执行请求操作的正确权限。与 NFS3ERR_PERM 不同，该错误不限于所有者或特权用户权限失败的情况。
    NFS3ERR_ACCES = 13,
    /// 文件已存在。指定的文件已存在。
    NFS3ERR_EXIST = 17,
    /// 尝试进行跨设备硬链接。
    NFS3ERR_XDEV = 18,
    /// 设备不存在。
    NFS3ERR_NODEV = 19,
    /// 不是目录。调用者在目录操作中指定了非目录对象。
    NFS3ERR_NOTDIR = 20,
    /// 是目录。调用者在非目录操作中指定了目录对象。
    NFS3ERR_ISDIR = 21,
    /// 参数无效或操作不支持该参数。例如，尝试对非符号链接对象执行 READLINK 操作，或尝试在不支持的服务器上设置文件时间属性。
    NFS3ERR_INVAL = 22,
    /// 文件过大。该操作会导致文件大小超过服务器限制。
    NFS3ERR_FBIG = 27,
    /// 设备空间不足。该操作会导致服务器文件系统超出其容量限制。
    NFS3ERR_NOSPC = 28,
    /// 文件系统为只读。尝试在只读文件系统上执行修改操作。
    NFS3ERR_ROFS = 30,
    /// 硬链接数量过多。
    NFS3ERR_MLINK = 31,
    /// 操作中的文件名过长。
    NFS3ERR_NAMETOOLONG = 63,
    /// 尝试删除非空目录。
    NFS3ERR_NOTEMPTY = 66,
    /// 资源（配额）硬限制已超出。用户在服务器上的资源限制已被超出。
    NFS3ERR_DQUOT = 69,
    /// 文件句柄无效。参数中提供的文件句柄无效。该文件句柄引用的文件已不存在或访问权限已被撤销。
    NFS3ERR_STALE = 70,
    /// 路径中远程层级过多。参数中提供的文件句柄引用了服务器上非本地文件系统中的文件。
    NFS3ERR_REMOTE = 71,
    /// 非法的 NFS 文件句柄。文件句柄未通过内部一致性检查。
    NFS3ERR_BADHANDLE = 10001,
    /// 在 SETATTR 操作期间检测到更新同步不匹配。
    NFS3ERR_NOT_SYNC = 10002,
    /// READDIR 或 READDIRPLUS 的 cookie 已过期。
    NFS3ERR_BAD_COOKIE = 10003,
    /// 操作不受支持。
    NFS3ERR_NOTSUPP = 10004,
    /// 缓冲区或请求过小。
    NFS3ERR_TOOSMALL = 10005,
    /// 服务器上发生的错误无法映射到任何合法的 NFS 版本 3 协议错误值。客户端应将其转换为适当的错误。UNIX 客户端可选择将其转换为 EIO。
    NFS3ERR_SERVERFAULT = 10006,
    /// 尝试创建服务器不支持的类型的对象。
    NFS3ERR_BADTYPE = 10007,
    /// 服务器已启动请求，但无法及时完成。客户端应等待，然后使用新的 RPC 事务 ID 重试请求。例如，支持分层存储的服务器在接收到处理已迁移文件的请求时，应启动回迁过程并返回此错误。
    NFS3ERR_JUKEBOX = 10008,
}

XDREnumSerde!(nfsstat3);

#[derive(Copy, Clone, Debug, Default, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum ftype3 {
    /// 普通文件
    #[default]
    NF3REG = 1,
    /// 目录
    NF3DIR = 2,
    /// 块特殊设备
    NF3BLK = 3,
    /// 字符特殊设备
    NF3CHR = 4,
    /// 符号链接
    NF3LNK = 5,
    /// 套接字
    NF3SOCK = 6,
    /// 命名管道
    NF3FIFO = 7,
}
XDREnumSerde!(ftype3);

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct specdata3 {
    pub specdata1: u32,
    pub specdata2: u32,
}
XDRStruct!(specdata3, specdata1, specdata2);

#[derive(Clone, Debug)]
pub struct nfs_fh3 {
    pub data: Vec<u8>,
}
XDRStruct!(nfs_fh3, data);
#[allow(clippy::derivable_impls)]
impl Default for nfs_fh3 {
    fn default() -> nfs_fh3 {
        nfs_fh3 { data: Vec::new() }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct nfstime3 {
    pub seconds: u32,
    pub nseconds: u32,
}
XDRStruct!(nfstime3, seconds, nseconds);

impl From<nfstime3> for filetime::FileTime {
    fn from(time: nfstime3) -> Self {
        filetime::FileTime::from_unix_time(time.seconds as i64, time.nseconds)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct fattr3 {
    /// 文件类型，使用 `ftype3` 枚举表示
    pub ftype: ftype3,
    /// 文件模式，用于表示文件的权限和访问控制
    pub mode: mode3,
    /// 文件的硬链接数量
    pub nlink: u32,
    /// 文件所有者的用户 ID
    pub uid: uid3,
    /// 文件所属组的组 ID
    pub gid: gid3,
    /// 文件的大小，以字节为单位
    pub size: size3,
    /// 文件实际占用的空间大小，以字节为单位
    pub used: size3,
    /// 设备号信息，用于表示字符设备或块设备
    pub rdev: specdata3,
    /// 文件系统 ID，标识文件所在的文件系统
    pub fsid: u64,
    /// 文件 ID，在文件系统内唯一标识一个文件
    pub fileid: fileid3,
    /// 文件的最后访问时间
    pub atime: nfstime3,
    /// 文件的最后修改时间
    pub mtime: nfstime3,
    /// 文件状态的最后更改时间
    pub ctime: nfstime3,
}
XDRStruct!(
    fattr3, ftype, mode, nlink, uid, gid, size, used, rdev, fsid, fileid, atime, mtime, ctime
);

#[derive(Debug, Default)]
pub struct fsinfo3 {
    /// 操作后对象的属性，使用 `post_op_attr` 类型表示
    pub obj_attributes: post_op_attr,
    /// 读操作的最大传输块大小（以字节为单位）
    pub rtmax: u32,
    /// 读操作的首选传输块大小（以字节为单位）
    pub rtpref: u32,
    /// 读操作传输块大小的乘数因子
    pub rtmult: u32,
    /// 写操作的最大传输块大小（以字节为单位）
    pub wtmax: u32,
    /// 写操作的首选传输块大小（以字节为单位）
    pub wtpref: u32,
    /// 写操作传输块大小的乘数因子
    pub wtmult: u32,
    /// 目录操作的首选传输块大小（以字节为单位）
    pub dtpref: u32,
    /// 文件的最大允许大小（以字节为单位）
    pub maxfilesize: size3,
    /// 服务器时间戳的精度，使用 `nfstime3` 类型表示
    pub time_delta: nfstime3,
    /// 文件系统属性的位掩码，用于表示文件系统支持的特性
    pub properties: u32,
}
XDRStruct!(
    fsinfo3,
    obj_attributes,
    rtmax,
    rtpref,
    rtmult,
    wtmax,
    wtpref,
    wtmult,
    dtpref,
    maxfilesize,
    time_delta,
    properties
);

#[derive(Copy, Clone, Debug, Default)]
pub struct wcc_attr {
    pub size: size3,
    pub mtime: nfstime3,
    pub ctime: nfstime3,
}
XDRStruct!(wcc_attr, size, mtime, ctime);

#[derive(Copy, Clone, Debug, Default)]
#[repr(u32)]
pub enum pre_op_attr {
    #[default]
    Void,
    attributes(wcc_attr),
}
XDRBoolUnion!(pre_op_attr, attributes, wcc_attr);

#[derive(Copy, Clone, Debug, Default)]
#[repr(u32)]
pub enum post_op_attr {
    #[default]
    Void,
    attributes(fattr3),
}
XDRBoolUnion!(post_op_attr, attributes, fattr3);

#[derive(Copy, Clone, Debug, Default)]
pub struct wcc_data {
    pub before: pre_op_attr,
    pub after: post_op_attr,
}
XDRStruct!(wcc_data, before, after);

#[derive(Clone, Debug, Default)]
#[repr(u32)]
pub enum post_op_fh3 {
    #[default]
    Void,
    handle(nfs_fh3),
}
XDRBoolUnion!(post_op_fh3, handle, nfs_fh3);

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum _time_how {
    DONT_CHANGE = 0,
    SET_TO_SERVER_TIME = 1,
    SET_TO_CLIENT_TIME = 2,
}
XDREnumSerde!(_time_how);


#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum set_mode3 {
    Void,
    mode(mode3),
}
XDRBoolUnion!(set_mode3, mode, mode3);

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum set_uid3 {
    Void,
    uid(uid3),
}
XDRBoolUnion!(set_uid3, uid, uid3);

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum set_gid3 {
    Void,
    gid(gid3),
}
XDRBoolUnion!(set_gid3, gid, gid3);

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum set_size3 {
    Void,
    size(size3),
}
XDRBoolUnion!(set_size3, size, size3);


#[derive(Copy, Clone, Debug)]
#[repr(u32)]
/// discriminant is time_how
pub enum set_atime {
    DONT_CHANGE,
    SET_TO_SERVER_TIME,
    SET_TO_CLIENT_TIME(nfstime3),
}
impl XDR for set_atime {
    fn serialize<R: Write>(&self, dest: &mut R) -> std::io::Result<()> {
        match self {
            set_atime::DONT_CHANGE => {
                0_u32.serialize(dest)?;
            }
            set_atime::SET_TO_SERVER_TIME => {
                1_u32.serialize(dest)?;
            }
            set_atime::SET_TO_CLIENT_TIME(v) => {
                2_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
        }
        Ok(())
    }
    fn deserialize<R: Read>(&mut self, src: &mut R) -> std::io::Result<()> {
        let mut c: u32 = 0;
        c.deserialize(src)?;
        if c == 0 {
            *self = set_atime::DONT_CHANGE;
        } else if c == 1 {
            *self = set_atime::SET_TO_SERVER_TIME;
        } else if c == 2 {
            let mut r = nfstime3::default();
            r.deserialize(src)?;
            *self = set_atime::SET_TO_CLIENT_TIME(r);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid value for set_atime",
            ));
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
/// discriminant is time_how
pub enum set_mtime {
    DONT_CHANGE,
    SET_TO_SERVER_TIME,
    SET_TO_CLIENT_TIME(nfstime3),
}

impl XDR for set_mtime {
    fn serialize<R: Write>(&self, dest: &mut R) -> std::io::Result<()> {
        match self {
            set_mtime::DONT_CHANGE => {
                0_u32.serialize(dest)?;
            }
            set_mtime::SET_TO_SERVER_TIME => {
                1_u32.serialize(dest)?;
            }
            set_mtime::SET_TO_CLIENT_TIME(v) => {
                2_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
        }
        Ok(())
    }
    fn deserialize<R: Read>(&mut self, src: &mut R) -> std::io::Result<()> {
        let mut c: u32 = 0;
        c.deserialize(src)?;
        if c == 0 {
            *self = set_mtime::DONT_CHANGE;
        } else if c == 1 {
            *self = set_mtime::SET_TO_SERVER_TIME;
        } else if c == 2 {
            let mut r = nfstime3::default();
            r.deserialize(src)?;
            *self = set_mtime::SET_TO_CLIENT_TIME(r);
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid value for set_mtime",
            ));
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct sattr3 {
    pub mode: set_mode3,
    pub uid: set_uid3,
    pub gid: set_gid3,
    pub size: set_size3,
    pub atime: set_atime,
    pub mtime: set_mtime,
}
XDRStruct!(sattr3, mode, uid, gid, size, atime, mtime);
impl Default for sattr3 {
    fn default() -> sattr3 {
        sattr3 {
            mode: set_mode3::Void,
            uid: set_uid3::Void,
            gid: set_gid3::Void,
            size: set_size3::Void,
            atime: set_atime::DONT_CHANGE,
            mtime: set_mtime::DONT_CHANGE,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct diropargs3 {
    pub dir: nfs_fh3,
    pub name: filename3,
}
XDRStruct!(diropargs3, dir, name);

#[derive(Debug, Default)]
pub struct symlinkdata3 {
    pub symlink_attributes: sattr3,
    pub symlink_data: nfspath3,
}
XDRStruct!(symlinkdata3, symlink_attributes, symlink_data);

/// We define the root handle here
pub fn get_root_mount_handle() -> Vec<u8> {
    vec![0]
}
