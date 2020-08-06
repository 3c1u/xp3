use std::fs;
use std::io::Read;
use std::path::Path;

use libflate::zlib::Decoder;

use super::extent;
use super::header::{self, Header};
use super::info::{self, XP3Info};
use super::segment::Segment;

use super::cxdec::CxDec;

pub struct XP3 {
    #[allow(dead_code)]
    header: Header,
    info: XP3Info,
    buf: Vec<u8>,
}

impl XP3 {
    pub fn get(&self, sg: &Segment, decoder: &mut CxDec, key: u32) -> Vec<u8> {
        let mut raw = self.buf[sg.offset as usize..(sg.offset + sg.storage_size) as usize].to_vec();
        if sg.flag == 1 {
            let mut decode = Decoder::new(&raw[..]).unwrap();
            let mut copy = Vec::new();
            decode.read_to_end(&mut copy).unwrap();
            raw = copy;
        }
        assert_eq!(raw.len(), sg.origin_size as usize);
        decoder.decrypt(&mut raw, key, 0);
        raw
    }

    pub fn extract(&self, path: &str) {
        let mut decoder = CxDec::new();

        if !Path::exists(Path::new(path)) {
            fs::create_dir(path).unwrap();
        }

        for i in &self.info.file {
            let fs = Path::new(path).join(i.file_name.clone());
            let mut file = Vec::new();
            for j in 0..i.segment_size as usize {
                let data = self.get(&i.seg[j], &mut decoder, i.key);
                file.extend_from_slice(data.as_slice());
            }
            let _ = fs::write(&fs, file);
            println!("{} done", fs.as_path().to_str().unwrap());
        }
    }
}

pub fn unpack(buf: &Vec<u8>) -> Result<XP3, ()> {
    let (header, _) = header::unpack(&buf);
    let (info, _) = info::unpack(&buf, header.offset as usize);
    return Ok(XP3 {
        header,
        info,
        buf: buf.clone(),
    });
}
