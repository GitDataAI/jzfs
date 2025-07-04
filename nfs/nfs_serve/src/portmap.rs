use crate::xdr::*;
use std::io::{Read, Write};
// Transcribed from RFC 1057 Appendix A

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct mapping {
    pub prog: u32,
    pub vers: u32,
    pub prot: u32,
    pub port: u32,
}
XDRStruct!(mapping, prog, vers, prot, port);
pub const IPPROTO_TCP: u32 = 6; 
pub const IPPROTO_UDP: u32 = 17; 
pub const PROGRAM: u32 = 100000;
pub const VERSION: u32 = 2;
