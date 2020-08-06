use super::segment::{self, Segment};
use super::utils;
use std::string::String;

const FILE: [u8; 4] = [0x46, 0x69, 0x6C, 0x65];
const INFO: [u8; 4] = [0x69, 0x6E, 0x66, 0x6F];
const SEGM: [u8; 4] = [0x73, 0x65, 0x67, 0x6D];
const ADLR: [u8; 4] = [0x61, 0x64, 0x6c, 0x72];

pub struct XP3File {
    /// Size of the `File` section.
    pub file_size: u64,
    /// Size of the `info` section.
    pub info_size: u64,
    /// Whether the file is encrypted.
    pub protect: u32,
    /// Entry size after extraction.
    pub rsize: u64,
    /// Entry size before extraction
    pub psize: u64,
    /// The length of the name.
    pub name_len: u16,
    /// File name.
    pub file_name: String,
    /// Size of the `segm` section.
    pub segment_size: u64,
    // Segments.
    pub seg: Vec<Segment>,
    /// Size of the `adlr` section. (4 bytes)
    pub adler_size: u64,
    /// Adler-32 checksum.
    pub key: u32,
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
                }
                &SEGM => {
                    segment_size = utils::read_u64(&buf, &mut offset);

                    assert!(
                        segment_size % 28 == 0,
                        "segment size must be a multiple of 28"
                    );
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
                        log::warn!(
                            "{}: the file is protected; the extration might fail",
                            file_name
                        );
                    }
                }
                _ => {
                    if let Ok(header) = std::str::from_utf8(header) {
                        log::warn!("unrecognized header '{}'; skipping", header);
                        let size = utils::read_u64(&buf, &mut offset);
                        offset += size as usize;
                    } else {
                        panic!(
                            "malformed header '{:x?}'. the header identifier must be four characters in ASCII",
                            header
                        );
                    }
                }
            }

            // skip zeros (the header should be four characters)
            while offset < buf.len() && buf[offset] == 0 {
                offset += 1;
            }
        }

        if file_name.len() >= 0x100 {
            // bogus entry
            log::warn!(
                "the filename is too long; probably a bogus entry: \"{}\"",
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
