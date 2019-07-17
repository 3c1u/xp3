pub fn Assert(buf: &Vec<u8>, cmp: &Vec<u8>, offset: usize) -> bool {
    let len = cmp.len();
    for i in 0..len {
        if buf[offset + i] != cmp[i] {
            return false;
        }
    }
    true
}

pub fn ReadU8(buf: &Vec<u8>, offset: &mut usize) -> u8 {
    let mut ret: u8 = buf[*offset];
    *offset += 1;
    ret
}

pub fn ReadU16(buf: &Vec<u8>, offset: &mut usize) -> u16 {
    let mut ret: u16 = 0;
    for i in (0..2).rev() {
        ret = (ret << 8) + buf[i + *offset] as u16;
    }
    *offset += 2;
    ret
}

pub fn ReadU32(buf: &Vec<u8>, offset: &mut usize) -> u32 {
    let mut ret: u32 = 0;
    for i in (0..4).rev() {
        ret = (ret << 8) + buf[i + *offset] as u32;
    }
    *offset += 4;
    ret
}

pub fn ReadU64(buf: &Vec<u8>, offset: &mut usize) -> u64 {
    let mut ret: u64 = 0;
    for i in (0..8).rev() {
        ret = (ret << 8) + buf[i + *offset] as u64;
    }
    *offset += 8;
    ret
}
