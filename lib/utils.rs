use std::string::String;

pub fn assert(buf: &[u8], cmp: &[u8], offset: usize) -> bool {
    let len = cmp.len();
    for i in 0..len {
        if buf[offset + i] != cmp[i] {
            return false;
        }
    }
    true
}

pub fn read_u8(buf: &Vec<u8>, offset: &mut usize) -> u8 {
    let ret: u8 = buf[*offset];
    *offset += 1;
    ret
}

pub fn read_u16(buf: &Vec<u8>, offset: &mut usize) -> u16 {
    let mut ret: u16 = 0;
    for i in (0..2).rev() {
        ret = (ret << 8) + buf[i + *offset] as u16;
    }
    *offset += 2;
    ret
}

pub fn read_u32(buf: &Vec<u8>, offset: &mut usize) -> u32 {
    let mut ret: u32 = 0;
    for i in (0..4).rev() {
        ret = (ret << 8) + buf[i + *offset] as u32;
    }
    *offset += 4;
    ret
}

pub fn read_u64(buf: &Vec<u8>, offset: &mut usize) -> u64 {
    let mut ret: u64 = 0;
    for i in (0..8).rev() {
        ret = (ret << 8) + buf[i + *offset] as u64;
    }
    *offset += 8;
    ret
}

pub fn read_utf16(buf: &Vec<u8>, offset: &mut usize, len: u16) -> String {
    let slice = buf[*offset..*offset + 2 * len as usize].to_vec();
    let mut comb = Vec::new();
    for i in 0..len as usize {
        comb.push(((slice[i * 2 + 1] as u16) << 8) | (slice[i * 2] as u16));
    }
    let ret = String::from_utf16(comb.as_slice()).expect("invalid UTF-16 string");
    *offset += 2 * len as usize;
    ret
}
