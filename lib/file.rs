use super::segment::{self, Segment};
use super::utils;
use std::string::String;

const FILE: [u8; 4] = [0x46, 0x69, 0x6C, 0x65];
const INFO: [u8; 4] = [0x69, 0x6E, 0x66, 0x6F];
const SEGM: [u8; 4] = [0x73, 0x65, 0x67, 0x6D];
const ADLR: [u8; 4] = [0x61, 0x64, 0x6c, 0x72];
pub struct XP3File {
    pub file_size: u64,
    // 文件信息数据大小
    pub info_size: u64,
    // 文件基本数据大小
    pub protect: u32,
    // 估计是表示此文件是否加过密
    pub rsize: u64,
    // 文件原始大小
    pub psize: u64,
    // 文件包中大小
    pub name_len: u16,
    // 文件名长度（指的是UTF-16字符个数）
    pub file_name: String,
    // 文件名(UTF-16LE编码，无0结尾) nameLen wchar_t
    pub segment_size: u64,
    // 文件段数据大小
    pub seg: Vec<Segment>,
    pub adler_size: u64,
    // 文件附加数据大小，一般是4
    pub key: u32,
    // 附加数据，用于加密
}
pub fn unpack(buf: &mut Vec<u8>) -> Vec<XP3File> {
    let mut offset = 0;
    let mut ret = Vec::new();
    while offset + 4 <= buf.len() {
        if !utils::assert(&buf, &FILE.to_vec(), offset.clone()) {
            panic!("XP3File FILE Tag Failed");
        }
        offset += 4;
        
        let file_size = utils::read_u64(&buf, &mut offset);
        if !utils::assert(&buf, &INFO.to_vec(), offset.clone()) {
            panic!("XP3File Info Tag Failed");
        }
        offset += 4;

        let info_size = utils::read_u64(&buf, &mut offset);
        let protect = utils::read_u32(&buf, &mut offset);
        let rsize = utils::read_u64(&buf, &mut offset);
        let psize = utils::read_u64(&buf, &mut offset);
        let name_len = utils::read_u16(&buf, &mut offset);
        let file_name = utils::read_utf16(&buf, &mut offset, name_len.clone());

        if !utils::assert(&buf, &SEGM.to_vec(), offset.clone()) {
            panic!("XP3File SEGM Tag Failed");
        }
        offset += 4;

        let mut segment_size = utils::read_u64(&buf, &mut offset);
        assert!(segment_size % 28 == 0);
        segment_size /= 28;
        let mut seg = Vec::new();

        for _i in 0..segment_size {
            let (res, o) = segment::unpack(&buf, offset.clone());
            offset = o;
            seg.push(res);
        }

        if !utils::assert(&buf, &ADLR.to_vec(), offset.clone()) {
            panic!("XP3File ADLR Tag Failed");
        }
        offset += 4;

        let adler_size = utils::read_u64(&buf, &mut offset);
        let key = utils::read_u32(&buf, &mut offset);

        if protect != 0 {
            assert_eq!(protect, 1 << 31);
            println!("{}:This File Does Not Wish to Be Extract", file_name)
        }

        ret.push(XP3File {
            file_size,
            info_size,
            protect,
            rsize,
            psize,
            name_len,
            file_name,
            segment_size,
            seg,
            adler_size,
            key,
        });
    }
    ret
}
