use super::utils;

const HEADER: [u8; 11] = [
    0x58, 0x50, 0x33, 0x0D, 0x0A, 0x20, 0x0A, 0x1A, 0x8B, 0x67, 0x01,
];
const CUSHION_INDEX: [u8; 8] = [0x17, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
const HEADER_MINOR_VERSION: [u8; 4] = [0x1, 0x0, 0x0, 0x0];
const CUSHION_HEADER: [u8; 1] = [0x80];
const INDEX_SIZE: [u8; 8] = [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];

pub struct Header {
    pub offset: u64,
    pub yoshiri_version: bool,
}

pub fn unpack(buf: &Vec<u8>) -> (Header, usize) {
    if !utils::assert(&buf, &HEADER.to_vec(), 0) {
        panic!("This is Not a Valid XP3 File (Header Failed)");
    }
    let ver = utils::assert(&buf, &CUSHION_INDEX.to_vec(), 11);
    if ver {
        if !utils::assert(&buf, &HEADER_MINOR_VERSION.to_vec(), 19) {
            panic!("This is Not a Valid XP3 File (Header Minor Version Failed)");
        }

        if !utils::assert(&buf, &CUSHION_HEADER.to_vec(), 23) {
            panic!("This is Not a Valid XP3 File (Cushion Header Failed)");
        }

        if !utils::assert(&buf, &INDEX_SIZE.to_vec(), 24) {
            panic!("This is Not a Valid XP3 File (Index Size Failed)");
        }
    }
    let mut p = if ver { 32 } else { 11 };
    let o = utils::read_u64(&buf, &mut p);
    (
        Header {
            offset: o,
            yoshiri_version: ver,
        },
        p - 8,
    )
}
