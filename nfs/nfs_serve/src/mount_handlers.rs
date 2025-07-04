use crate::context::RPCContext;
use crate::mount::*;
use crate::rpc::*;
use crate::xdr::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::{FromPrimitive, ToPrimitive};
use std::io::{Read, Write};
use tracing::debug;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
enum MountProgram {
    MOUNTPROC3_NULL = 0,
    MOUNTPROC3_MNT = 1,
    MOUNTPROC3_DUMP = 2,
    MOUNTPROC3_UMNT = 3,
    MOUNTPROC3_UMNTALL = 4,
    MOUNTPROC3_EXPORT = 5,
    INVALID,
}

pub async fn handle_mount(
    xid: u32,
    call: call_body,
    input: &mut impl Read,
    output: &mut impl Write,
    context: &RPCContext,
) -> Result<(), anyhow::Error> {
    let prog = MountProgram::from_u32(call.proc).unwrap_or(MountProgram::INVALID);

    match prog {
        MountProgram::MOUNTPROC3_NULL => mountproc3_null(xid, input, output)?,
        MountProgram::MOUNTPROC3_MNT => mountproc3_mnt(xid, input, output, context).await?,
        MountProgram::MOUNTPROC3_UMNT => mountproc3_umnt(xid, input, output, context).await?,
        MountProgram::MOUNTPROC3_UMNTALL => {
            mountproc3_umnt_all(xid, input, output, context).await?
        }
        MountProgram::MOUNTPROC3_EXPORT => mountproc3_export(xid, input, output, context)?,
        _ => {
            proc_unavail_reply_message(xid).serialize(output)?;
        }
    }
    Ok(())
}

pub fn mountproc3_null(
    xid: u32,
    _: &mut impl Read,
    output: &mut impl Write,
) -> Result<(), anyhow::Error> {
    debug!("mountproc3_null({:?}) ", xid);
    // build an RPC reply
    let msg = make_success_reply(xid);
    debug!("\t{:?} --> {:?}", xid, msg);
    msg.serialize(output)?;
    Ok(())
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
struct mountres3_ok {
    fhandle: fhandle3, // really same thing as nfs::nfs_fh3
    auth_flavors: Vec<u32>,
}
XDRStruct!(mountres3_ok, fhandle, auth_flavors);

pub async fn mountproc3_mnt(
    xid: u32,
    input: &mut impl Read,
    output: &mut impl Write,
    context: &RPCContext,
) -> Result<(), anyhow::Error> {
    let mut path = dirpath::new();
    path.deserialize(input)?;
    let utf8path = std::str::from_utf8(&path).unwrap_or_default();
    debug!("mountproc3_mnt({:?},{:?}) ", xid, utf8path);
    let path = if let Some(path) = utf8path.strip_prefix(context.export_name.as_str()) {
        let path = path
            .trim_start_matches('/')
            .trim_end_matches('/')
            .trim()
            .as_bytes();
        let mut new_path = Vec::with_capacity(path.len() + 1);
        new_path.push(b'/');
        new_path.extend_from_slice(path);
        new_path
    } else {
        // invalid export
        debug!("{:?} --> no matching export", xid);
        make_success_reply(xid).serialize(output)?;
        mountstat3::MNT3ERR_NOENT.serialize(output)?;
        return Ok(());
    };
    if let Ok(fileid) = context.vfs.path_to_id(&path).await {
        let response = mountres3_ok {
            fhandle: context.vfs.id_to_fh(fileid).data,
            auth_flavors: vec![
                auth_flavor::AUTH_NULL.to_u32().unwrap(),
                auth_flavor::AUTH_UNIX.to_u32().unwrap(),
            ],
        };
        debug!("{:?} --> {:?}", xid, response);
        if let Some(ref chan) = context.mount_signal {
            let _ = chan.send(true).await;
        }
        make_success_reply(xid).serialize(output)?;
        mountstat3::MNT3_OK.serialize(output)?;
        response.serialize(output)?;
    } else {
        debug!("{:?} --> MNT3ERR_NOENT", xid);
        make_success_reply(xid).serialize(output)?;
        mountstat3::MNT3ERR_NOENT.serialize(output)?;
    }
    Ok(())
}

pub fn mountproc3_export(
    xid: u32,
    _: &mut impl Read,
    output: &mut impl Write,
    context: &RPCContext,
) -> Result<(), anyhow::Error> {
    debug!("mountproc3_export({:?}) ", xid);
    make_success_reply(xid).serialize(output)?;
    true.serialize(output)?;
    // dirpath
    context.export_name.as_bytes().to_vec().serialize(output)?;
    // groups
    false.serialize(output)?;
    // next exports
    false.serialize(output)?;
    Ok(())
}

pub async fn mountproc3_umnt(
    xid: u32,
    input: &mut impl Read,
    output: &mut impl Write,
    context: &RPCContext,
) -> Result<(), anyhow::Error> {
    let mut path = dirpath::new();
    path.deserialize(input)?;
    let utf8path = std::str::from_utf8(&path).unwrap_or_default();
    debug!("mountproc3_umnt({:?},{:?}) ", xid, utf8path);
    if let Some(ref chan) = context.mount_signal {
        let _ = chan.send(false).await;
    }
    make_success_reply(xid).serialize(output)?;
    mountstat3::MNT3_OK.serialize(output)?;
    Ok(())
}

pub async fn mountproc3_umnt_all(
    xid: u32,
    _input: &mut impl Read,
    output: &mut impl Write,
    context: &RPCContext,
) -> Result<(), anyhow::Error> {
    debug!("mountproc3_umnt_all({:?}) ", xid);
    if let Some(ref chan) = context.mount_signal {
        let _ = chan.send(false).await;
    }
    make_success_reply(xid).serialize(output)?;
    mountstat3::MNT3_OK.serialize(output)?;
    Ok(())
}
