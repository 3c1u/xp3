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

    'outer: while offset + 4 <= buf.len() {
        let mut file_size = 0;
        let mut info_size = 0;
        let mut protect = 0;
        let mut rsize = 0;
        let mut psize = 0;
        let mut name_len = 0;
        let mut file_name = String::new();
        let mut adler_size = 0;
        let mut key = 0;
        let mut segment_size = 0;
        let mut seg = Vec::new();

        let mut has_file = false;

        loop {
            use std::convert::TryInto;

            if buf.len() <= offset {
                break 'outer;
            }

            let header: &[u8; 4] = if let Ok(header) = buf[offset..][..4].try_into() {
                header
            } else {
                panic!("failed to obtain the header");
            };

            offset += 4;

            match header {
                &FILE => {
                    if has_file {
                        offset -= 4;
                        break;
                    }

                    file_size = utils::read_u64(&buf, &mut offset);
                    has_file = true;
                }
                &ADLR => {
                    adler_size = utils::read_u64(&buf, &mut offset);
                    key = utils::read_u32(&buf, &mut offset);

                    if !utils::assert(&buf, &SEGM, offset.clone()) {
                        panic!("XP3File SEGM Tag Failed");
                    }
                }
                &SEGM => {
                    segment_size = utils::read_u64(&buf, &mut offset);

                    assert!(segment_size % 28 == 0);
                    segment_size /= 28;

                    for _i in 0..segment_size {
                        let (res, o) = segment::unpack(&buf, offset.clone());
                        offset = o;
                        seg.push(res);
                    }
                }
                &INFO => {
                    info_size = utils::read_u64(&buf, &mut offset);
                    protect = utils::read_u32(&buf, &mut offset);
                    rsize = utils::read_u64(&buf, &mut offset);
                    psize = utils::read_u64(&buf, &mut offset);
                    name_len = utils::read_u16(&buf, &mut offset);
                    file_name = utils::read_utf16(&buf, &mut offset, name_len.clone());

                    if protect != 0 {
                        assert_eq!(protect, 1 << 31);
                        /* println!(
                            "{}: the file is protected; the extration might fail",
                            file_name
                        ); */
                    }
                }
                _ => {
                    if let Ok(_) = std::str::from_utf8(header) {
                        // println!("unexpected header '{}'; skipping", header);
                        let size = utils::read_u64(&buf, &mut offset);
                        offset += size as usize;
                    } else {
                        panic!(
                            "malformed header '{:x?}'. the header identifier must be ASCII",
                            header
                        );
                    }
                }
            }

            // skip zeros
            while offset < buf.len() && buf[offset] == 0 {
                offset += 1;
            }
        }

        if file_name.len() >= 0x100 {
            // bogus entry
            println!(
                "the filename is too long; probably a bogus entry: {}",
                file_name
            );
            continue;
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
