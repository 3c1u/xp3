use super::utils;
use super::XP3File;
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
    pub XP3File: Vec<XP3File::XP3File>,
}

pub fn unpack(buf: &Vec<u8>, mut offset: usize) -> (XP3Info, usize) {
    let zlib = utils::ReadU8(&buf, &mut offset);
    let psize = utils::ReadU64(&buf, &mut offset);
    let rsize = if zlib != 0 {
        utils::ReadU64(&buf, &mut offset)
    } else {
        psize
    };
    let mut Raw = buf[offset..(offset + psize as usize)].to_vec();
    if zlib != 0 {
        let mut decode = Decoder::new(&Raw[..]).unwrap();
        let mut copy = Vec::new();
        decode.read_to_end(&mut copy).unwrap();
        Raw = copy;
    }
    let XP3File = XP3File::unpack(&mut Raw);
    (
        XP3Info {
            zlib,
            psize,
            rsize,
            XP3File,
        },
        offset + psize as usize,
    )
}
