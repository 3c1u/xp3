use super::utils;
use libflate::zlib::Decoder;
use std::io::Read;

pub struct XP3Info {
    zlib: u8,
    // 文件信息表是否用zlib压缩过
    psize: u64,
    // 文件信息表在包文件中的大小
    rsize: u64,
    // if zlib
// 文件信息表解压后的大小
    fileInfo: Vec<u8>, //psize
// 文件信息表数据
}

pub fn unpack(buf: &Vec<u8>, mut offset: usize) -> (XP3Info, usize) {
    let zlib=utils::ReadU8(&buf,&mut offset);
    let psize=utils::ReadU64(&buf,&mut offset);
    let rsize=if zlib!=0 {
        utils::ReadU64(&buf,&mut offset)
    }else{
        psize
    };
    let mut file=buf[offset..(offset+psize as usize)].to_vec();
    if zlib!=0{
        let mut decode=Decoder::new(& file[..]).unwrap();
        let mut copy=Vec::new();
        decode.read_to_end(&mut copy).unwrap();
        file=copy;
    }
    (XP3Info {
        zlib,
        psize,
        rsize,
        fileInfo:file
    }, offset+psize as usize)
}
