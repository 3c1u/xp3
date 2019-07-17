use super::utils;
use super::Segment;
use crate::Solve::XP3;
use std::string::String;

const FILE: [u8; 4] = [0x46, 0x69, 0x6C, 0x65];
const INFO: [u8; 4] = [0x69, 0x6E, 0x66, 0x6F];
const SEGM: [u8; 4] = [0x73, 0x65, 0x67, 0x6D];
const ADLR: [u8; 4] = [0x61, 0x64, 0x6c, 0x72];
pub struct XP3File {
    pub fileSize: u64,
    // 文件信息数据大小
    pub infoSize: u64,
    // 文件基本数据大小
    pub protect: u32,
    // 估计是表示此文件是否加过密
    pub rsize: u64,
    // 文件原始大小
    pub psize: u64,
    // 文件包中大小
    pub nameLen: u16,
    // 文件名长度（指的是UTF-16字符个数）
    pub fileName: String,
    // 文件名(UTF-16LE编码，无0结尾) nameLen wchar_t
    pub segmSize: u64,
    // 文件段数据大小
    pub seg: Vec<Segment::Segment>,
    pub adlrSize: u64,
    // 文件附加数据大小，一般是4
    pub key: u32,
    // 附加数据，用于加密
}
pub fn unpack(buf: &mut Vec<u8>) -> Vec<XP3File> {
    let mut offset = 0;
    let mut ret = Vec::new();
    while offset + 4 <= buf.len() {
        if !utils::Assert(&buf, &FILE.to_vec(), offset.clone()) {
            panic!("XP3File FILE Tag Failed");
        }
        offset += 4;
        let fileSize = utils::ReadU64(&buf, &mut offset);
        if !utils::Assert(&buf, &INFO.to_vec(), offset.clone()) {
            panic!("XP3File Info Tag Failed");
        }
        offset += 4;
        let infoSize = utils::ReadU64(&buf, &mut offset);
        let protect = utils::ReadU32(&buf, &mut offset);
        let rsize = utils::ReadU64(&buf, &mut offset);
        let psize = utils::ReadU64(&buf, &mut offset);
        let nameLen = utils::ReadU16(&buf, &mut offset);
        let fileName = utils::ReadUTF16(&buf, &mut offset, nameLen.clone());
        if !utils::Assert(&buf, &SEGM.to_vec(), offset.clone()) {
            panic!("XP3File SEGM Tag Failed");
        }
        offset += 4;
        let mut segmSize = utils::ReadU64(&buf, &mut offset);
        assert!(segmSize % 28 == 0);
        segmSize /= 28;
        let mut seg = Vec::new();
        for i in 0..segmSize {
            let (res, o) = Segment::unpack(&buf, offset.clone());
            offset = o;
            seg.push(res);
        }
        if !utils::Assert(&buf, &ADLR.to_vec(), offset.clone()) {
            panic!("XP3File ADLR Tag Failed");
        }
        offset += 4;
        let adlrSize = utils::ReadU64(&buf, &mut offset);
        let key = utils::ReadU32(&buf, &mut offset);
        ret.push(XP3File {
            fileSize,
            infoSize,
            protect,
            rsize,
            psize,
            nameLen,
            fileName,
            segmSize,
            seg,
            adlrSize,
            key,
        });
    }
    ret
}
