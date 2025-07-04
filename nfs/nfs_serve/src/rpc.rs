use crate::xdr::*;
use byteorder::{ReadBytesExt, WriteBytesExt};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::FromPrimitive;
use std::io::{Read, Write};
// Transcribed from RFC 1057

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
/// 仅定义为rpc_body的判别式，不应直接使用
pub enum _msg_type {
    CALL = 0,
    REPLY = 1,
}
XDREnumSerde!(_msg_type);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
/// 仅定义为reply_body的判别式，不应直接使用
pub enum _reply_stat {
    MSG_ACCEPTED = 0,
    MSG_DENIED = 1,
}
XDREnumSerde!(_reply_stat);

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
/// 仅定义为accept_body的判别式，不应直接使用
pub enum _accept_stat {
    /// RPC executed successfully
    SUCCESS = 0,
    /// remote hasn't exported program
    PROG_UNAVAIL = 1,
    /// 远程不支持该版本号
    PROG_MISMATCH = 2,
    /// program can't support procedure
    PROC_UNAVAIL = 3,
    /// 过程无法解码参数
    GARBAGE_ARGS = 4,
}
XDREnumSerde!(_accept_stat);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
/// 仅定义为reject_body的判别式，不应直接使用
pub enum _reject_stat {
    /// RPC version number != 2
    RPC_MISMATCH = 0,
    /// remote can't authenticate caller
    AUTH_ERROR = 1,
}
XDREnumSerde!(_reject_stat);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default, FromPrimitive, ToPrimitive)]
#[repr(u32)]
///   认证失败的原因
pub enum auth_stat {
    /// 无效的凭据(密封已破坏)
    #[default]
    AUTH_BADCRED = 1,
    /// 客户端必须开始新会话
    AUTH_REJECTEDCRED = 2,
    /// 无效的验证器(密封已破坏)
    AUTH_BADVERF = 3,
    /// 验证器已过期或重放
    AUTH_REJECTEDVERF = 4,
    /// 因安全原因被拒绝
    AUTH_TOOWEAK = 5,
}
XDREnumSerde!(auth_stat);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
#[repr(u32)]
#[non_exhaustive]
pub enum auth_flavor {
    AUTH_NULL = 0,
    AUTH_UNIX = 1,
    AUTH_SHORT = 2,
    AUTH_DES = 3, /* and more to be defined */
}
XDREnumSerde!(auth_flavor);

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default)]
pub struct auth_unix {
    stamp: u32,
    machinename: Vec<u8>,
    uid: u32,
    gid: u32,
    gids: Vec<u32>,
}
XDRStruct!(auth_unix, stamp, machinename, uid, gid, gids);

///RPC协议提供了调用者与服务之间的双向认证机制。调用消息包含两个
///认证字段：凭据(credentials)和验证器(verifier)。回复消息包含一个
///认证字段：响应验证器。RPC协议规范将这三个字段定义为以下不透明类型
///(使用外部数据表示(XDR)语言[9])：
///
///换句话说，任何"opaque_auth"结构都是一个"auth_flavor"枚举，后跟对RPC
///协议实现来说不透明(无法解释)的字节数据。
///
///认证字段中包含的数据的解释和语义由各个独立的认证协议规范定义。
///(第9节定义了各种认证协议。)
///
///如果认证参数被拒绝，回复消息将包含说明拒绝原因的信息。
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct opaque_auth {
    pub flavor: auth_flavor,
    pub body: Vec<u8>,
}
XDRStruct!(opaque_auth, flavor, body);
impl Default for opaque_auth {
    fn default() -> opaque_auth {
        opaque_auth {
            flavor: auth_flavor::AUTH_NULL,
            body: Vec::new(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default)]
///所有消息都以事务标识符xid开头，后跟一个双臂判别联合。联合的判别式是
///msg_type，用于切换消息的两种类型之一。REPLY消息的xid始终与发起的CALL
///消息的xid匹配。注意：xid字段仅用于客户端将回复消息与调用消息匹配，或
///服务器检测重传；服务端不能将此ID视为任何类型的序列号。
pub struct rpc_msg {
    pub xid: u32,
    pub body: rpc_body,
}
XDRStruct!(rpc_msg, xid, body);

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug)]
#[repr(u32)]
/// 判别式为msg_type
pub enum rpc_body {
    CALL(call_body),
    REPLY(reply_body),
}

impl Default for rpc_body {
    fn default() -> rpc_body {
        rpc_body::CALL(call_body::default())
    }
}
impl XDR for rpc_body {
    fn serialize<R: Write>(&self, dest: &mut R) -> std::io::Result<()> {
        match self {
            rpc_body::CALL(v) => {
                0_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
            rpc_body::REPLY(v) => {
                1_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
        }
        Ok(())
    }
    fn deserialize<R: Read>(&mut self, src: &mut R) -> std::io::Result<()> {
        let mut c: u32 = 0;
        c.deserialize(src)?;
        if c == 0 {
            let mut r = call_body::default();
            r.deserialize(src)?;
            *self = rpc_body::CALL(r);
        } else if c == 1 {
            let mut r = reply_body::default();
            r.deserialize(src)?;
            *self = rpc_body::REPLY(r);
        }
        Ok(())
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default)]

///RPC调用消息有三个无符号整数字段——远程程序号、远程程序版本号和远程过程
///号——用于唯一标识要调用的过程。程序号由某个中央机构(如Sun)管理。一旦
///实现者获得程序号，他们就可以实现其远程程序；第一个实现的版本号很可能是1。
///由于大多数新协议都会演进，调用消息的版本字段标识调用者使用的协议版本。
///版本号使得通过同一服务器进程使用新旧协议成为可能。
///
///过程号标识要调用的过程。这些号码在特定程序的协议规范中有记录。例如，
///文件服务的协议规范可能规定其过程号5为"read"，过程号12为"write"。
///
///就像远程程序协议可能在多个版本中发生变化一样，实际的RPC消息协议也可能
///发生变化。因此，调用消息中还包含RPC版本号，对于此处描述的RPC版本，该版本号
///始终等于2。
///
///请求消息的回复消息包含足够的信息来区分以下错误情况：
///
///(1) RPC的远程实现不支持协议版本2。返回支持的最低和最高RPC版本号。
///
///(2) 远程程序在远程系统上不可用。
///
///(3) 远程程序不支持请求的版本号。返回支持的最低和最高远程程序版本号。
///
///(4) 请求的过程号不存在。(这通常是客户端协议或编程错误。)
///
///(5) 从服务器的角度来看，远程过程的参数似乎是无效数据。(同样，这通常是由
///客户端和服务之间的协议不一致引起的。)
///
///在RPC协议规范的版本2中，rpcvers必须等于2。字段prog、vers和proc指定
///远程程序、其版本号以及要调用的远程程序中的过程。这些字段之后是两个认证
///参数：cred(认证凭据)和verf(认证验证器)。这两个认证参数之后是远程过程
///的参数，这些参数由特定的程序协议指定。
pub struct call_body {
    /// 必须等于2
    pub rpcvers: u32,
    pub prog: u32,
    pub vers: u32,
    pub proc: u32,
    pub cred: opaque_auth,
    pub verf: opaque_auth,
    /* procedure specific parameters start here */
}
XDRStruct!(call_body, rpcvers, prog, vers, proc, cred, verf);
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
#[repr(u32)]

pub enum reply_body {
    MSG_ACCEPTED(accepted_reply),
    MSG_DENIED(rejected_reply),
}
impl Default for reply_body {
    fn default() -> reply_body {
        reply_body::MSG_ACCEPTED(accepted_reply::default())
    }
}

impl XDR for reply_body {
    fn serialize<R: Write>(&self, dest: &mut R) -> std::io::Result<()> {
        match self {
            reply_body::MSG_ACCEPTED(v) => {
                0_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
            reply_body::MSG_DENIED(v) => {
                1_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
        }
        Ok(())
    }
    fn deserialize<R: Read>(&mut self, src: &mut R) -> std::io::Result<()> {
        let mut c: u32 = 0;
        c.deserialize(src)?;
        if c == 0 {
            let mut r = accepted_reply::default();
            r.deserialize(src)?;
            *self = reply_body::MSG_ACCEPTED(r);
        } else if c == 1 {
            let mut r = rejected_reply::default();
            r.deserialize(src)?;
            *self = reply_body::MSG_DENIED(r);
        }
        Ok(())
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct mismatch_info {
    pub low: u32,
    pub high: u32,
}
XDRStruct!(mismatch_info, low, high);

///服务器接受RPC调用的回复：
///即使调用被接受，也可能存在错误。第一个字段是服务器生成的认证验证器，
///用于向客户端验证自身。其后是一个联合，其判别式是枚举accept_stat。联合的
///SUCCESS分支是协议特定的。PROG_UNAVAIL、PROC_UNAVAIL和GARBAGE_ARGS分支
///为空。PROG_MISMATCH分支指定服务器支持的远程程序的最低和最高版本号。
/// 判别式为reply_stat
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default)]
pub struct accepted_reply {
    pub verf: opaque_auth,
    pub reply_data: accept_body,
}
XDRStruct!(accepted_reply, verf, reply_data);

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Default)]
#[repr(u32)]
/// 判别式为accept_stat
pub enum accept_body {
    #[default]
    SUCCESS,
    PROG_UNAVAIL,
    /// remote can't support version #
    PROG_MISMATCH(mismatch_info),
    PROC_UNAVAIL,
    /// procedure can't decode params
    GARBAGE_ARGS,
}
impl XDR for accept_body {
    fn serialize<R: Write>(&self, dest: &mut R) -> std::io::Result<()> {
        match self {
            accept_body::SUCCESS => {
                0_u32.serialize(dest)?;
            }
            accept_body::PROG_UNAVAIL => {
                1_u32.serialize(dest)?;
            }
            accept_body::PROG_MISMATCH(v) => {
                2_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
            accept_body::PROC_UNAVAIL => {
                3_u32.serialize(dest)?;
            }
            accept_body::GARBAGE_ARGS => {
                4_u32.serialize(dest)?;
            }
        }
        Ok(())
    }
    fn deserialize<R: Read>(&mut self, src: &mut R) -> std::io::Result<()> {
        let mut c: u32 = 0;
        c.deserialize(src)?;
        if c == 0 {
            *self = accept_body::SUCCESS;
        } else if c == 1 {
            *self = accept_body::PROG_UNAVAIL;
        } else if c == 2 {
            let mut r = mismatch_info::default();
            r.deserialize(src)?;
            *self = accept_body::PROG_MISMATCH(r);
        } else if c == 3 {
            *self = accept_body::PROC_UNAVAIL;
        } else {
            *self = accept_body::GARBAGE_ARGS;
        }
        Ok(())
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
#[repr(u32)]
///服务器拒绝RPC调用的回复：
///
///调用可能因两个原因被拒绝：服务器未运行兼容版本的RPC协议(RPC_MISMATCH)，
///或者服务器拒绝认证调用者(AUTH_ERROR)。在RPC版本不匹配的情况下，服务器返回
///支持的最低和最高RPC版本号。在认证被拒绝的情况下，返回失败状态。
/// 判别式为reject_stat
pub enum rejected_reply {
    RPC_MISMATCH(mismatch_info),
    AUTH_ERROR(auth_stat),
}

impl Default for rejected_reply {
    fn default() -> rejected_reply {
        rejected_reply::RPC_MISMATCH(mismatch_info::default())
    }
}

impl XDR for rejected_reply {
    fn serialize<R: Write>(&self, dest: &mut R) -> std::io::Result<()> {
        match self {
            rejected_reply::RPC_MISMATCH(v) => {
                0_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
            rejected_reply::AUTH_ERROR(v) => {
                1_u32.serialize(dest)?;
                v.serialize(dest)?;
            }
        }
        Ok(())
    }
    fn deserialize<R: Read>(&mut self, src: &mut R) -> std::io::Result<()> {
        let mut c: u32 = 0;
        c.deserialize(src)?;
        if c == 0 {
            let mut r = mismatch_info::default();
            r.deserialize(src)?;
            *self = rejected_reply::RPC_MISMATCH(r);
        } else if c == 1 {
            let mut r = auth_stat::default();
            r.deserialize(src)?;
            *self = rejected_reply::AUTH_ERROR(r);
        }
        Ok(())
    }
}

pub fn proc_unavail_reply_message(xid: u32) -> rpc_msg {
    let reply = reply_body::MSG_ACCEPTED(accepted_reply {
        verf: opaque_auth::default(),
        reply_data: accept_body::PROC_UNAVAIL,
    });
    rpc_msg {
        xid,
        body: rpc_body::REPLY(reply),
    }
}
pub fn prog_unavail_reply_message(xid: u32) -> rpc_msg {
    let reply = reply_body::MSG_ACCEPTED(accepted_reply {
        verf: opaque_auth::default(),
        reply_data: accept_body::PROG_UNAVAIL,
    });
    rpc_msg {
        xid,
        body: rpc_body::REPLY(reply),
    }
}
pub fn prog_mismatch_reply_message(xid: u32, accepted_ver: u32) -> rpc_msg {
    let reply = reply_body::MSG_ACCEPTED(accepted_reply {
        verf: opaque_auth::default(),
        reply_data: accept_body::PROG_MISMATCH(mismatch_info {
            low: accepted_ver,
            high: accepted_ver,
        }),
    });
    rpc_msg {
        xid,
        body: rpc_body::REPLY(reply),
    }
}
pub fn garbage_args_reply_message(xid: u32) -> rpc_msg {
    let reply = reply_body::MSG_ACCEPTED(accepted_reply {
        verf: opaque_auth::default(),
        reply_data: accept_body::GARBAGE_ARGS,
    });
    rpc_msg {
        xid,
        body: rpc_body::REPLY(reply),
    }
}

pub fn rpc_vers_mismatch(xid: u32) -> rpc_msg {
    let reply = reply_body::MSG_DENIED(rejected_reply::RPC_MISMATCH(mismatch_info::default()));
    rpc_msg {
        xid,
        body: rpc_body::REPLY(reply),
    }
}

pub fn make_success_reply(xid: u32) -> rpc_msg {
    let reply = reply_body::MSG_ACCEPTED(accepted_reply {
        verf: opaque_auth::default(),
        reply_data: accept_body::SUCCESS,
    });
    rpc_msg {
        xid,
        body: rpc_body::REPLY(reply),
    }
}
