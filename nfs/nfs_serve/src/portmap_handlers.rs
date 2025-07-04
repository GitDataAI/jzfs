use crate::context::RPCContext;
use crate::portmap;
use crate::rpc::*;
use crate::xdr::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::cast::FromPrimitive;
use std::io::{Read, Write};
use tracing::{debug, error};
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive)]
enum PortmapProgram {
    PMAPPROC_NULL = 0,
    PMAPPROC_SET = 1,
    PMAPPROC_UNSET = 2,
    PMAPPROC_GETPORT = 3,
    PMAPPROC_DUMP = 4,
    PMAPPROC_CALLIT = 5,
    INVALID,
}

pub fn handle_portmap(
    xid: u32,
    call: call_body,
    input: &mut impl Read,
    output: &mut impl Write,
    context: &RPCContext,
) -> Result<(), anyhow::Error> {
    if call.vers != portmap::VERSION {
        error!(
            "Invalid Portmap Version number {} != {}",
            call.vers,
            portmap::VERSION
        );
        prog_mismatch_reply_message(xid, portmap::VERSION).serialize(output)?;
        return Ok(());
    }
    let prog = PortmapProgram::from_u32(call.proc).unwrap_or(PortmapProgram::INVALID);

    match prog {
        PortmapProgram::PMAPPROC_NULL => pmapproc_null(xid, input, output)?,
        PortmapProgram::PMAPPROC_GETPORT => pmapproc_getport(xid, input, output, context)?,
        _ => {
            proc_unavail_reply_message(xid).serialize(output)?;
        }
    }
    Ok(())
}

pub fn pmapproc_null(
    xid: u32,
    _: &mut impl Read,
    output: &mut impl Write,
) -> Result<(), anyhow::Error> {
    debug!("pmapproc_null({:?}) ", xid);
    // build an RPC reply
    let msg = make_success_reply(xid);
    debug!("\t{:?} --> {:?}", xid, msg);
    msg.serialize(output)?;
    Ok(())
}

pub fn pmapproc_getport(
    xid: u32,
    read: &mut impl Read,
    output: &mut impl Write,
    context: &RPCContext,
) -> Result<(), anyhow::Error> {
    let mut mapping = portmap::mapping::default();
    mapping.deserialize(read)?;
    debug!("pmapproc_getport({:?}, {:?}) ", xid, mapping);
    make_success_reply(xid).serialize(output)?;
    let port = context.local_port as u32;
    debug!("\t{:?} --> {:?}", xid, port);
    port.serialize(output)?;
    Ok(())
}
