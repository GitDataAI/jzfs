use crate::xdr::*;
use byteorder::{ReadBytesExt, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::FromPrimitive;
use std::io::{Read, Write};
// 转录自RFC 1057附录A

pub const PROGRAM: u32 = 100005;
pub const VERSION: u32 = 3;

pub const MNTPATHLEN: u32 = 1024; /* 路径名的最大字节数 */
pub const MNTNAMLEN: u32 = 255; /* 名称的最大字节数 */
pub const FHSIZE3: u32 = 64; /* V3文件句柄的最大字节数 */

pub type fhandle3 = Vec<u8>;
pub type dirpath = Vec<u8>;
pub type name = Vec<u8>;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum mountstat3 {
    MNT3_OK = 0,                 /* 无错误 */
    MNT3ERR_PERM = 1,            /* 不是所有者 */
    MNT3ERR_NOENT = 2,           /* 没有这样的文件或目录 */
    MNT3ERR_IO = 5,              /* I/O错误 */
    MNT3ERR_ACCES = 13,          /* 权限被拒绝 */
    MNT3ERR_NOTDIR = 20,         /* 不是目录 */
    MNT3ERR_INVAL = 22,          /* 无效参数 */
    MNT3ERR_NAMETOOLONG = 63,    /* 文件名太长 */
    MNT3ERR_NOTSUPP = 10004,     /* 不支持的操作 */
    MNT3ERR_SERVERFAULT = 10006, /* 服务器故障 */
}
XDREnumSerde!(mountstat3);