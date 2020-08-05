use super::file::{self, XP3File};
use super::utils;
use libflate::zlib::Decoder;
use std::io::Read;

pub struct XP3Info {
    pub zlib: u8,
    // 文件信息表是否用zlib压缩过
    pub psize: u64,
    // 文件信息表在包文件中的大小
    pub rsize: u64,
    // if zlib
    // 文件信息表解压后的大小
    pub file: Vec<XP3File>,
}

pub fn unpack(buf: &Vec<u8>, mut offset: usize) -> (XP3Info, usize) {
    let zlib = utils::read_u8(&buf, &mut offset);
    let psize = utils::read_u64(&buf, &mut offset);
    let rsize = if zlib != 0 {
        utils::read_u64(&buf, &mut offset)
    } else {
        psize
    };
    let mut raw = buf[offset..(offset + psize as usize)].to_vec();
    if zlib != 0 {
        let mut decode = Decoder::new(&raw[..]).unwrap();
        let mut copy = Vec::new();
        decode.read_to_end(&mut copy).unwrap();
        raw = copy;
    }
    let file = file::unpack(&mut raw);
    (
        XP3Info {
            zlib,
            psize,
            rsize,
            file,
        },
        offset + psize as usize,
    )
}
