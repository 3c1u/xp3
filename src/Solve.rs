use super::Header;
use super::Segment::Segment;
use super::XP3Info;
use libflate::zlib::Decoder;
use std::fs;
use std::path::Path;

pub struct XP3 {
    Header: Header::Header,
    XP3Info: XP3Info::XP3Info,
    buf: Vec<u8>,
}
impl XP3 {
    pub fn get(&self, sg: &Segment) -> Vec<u8> {
        let mut ret = Vec::new();
        ret.push(1);
        ret
    }
    pub fn extract(&self, path: &str) {
        fs::create_dir(path);
        for i in &self.XP3Info.XP3File {
            let fs = Path::new(path).join(i.fileName.clone());
            for j in 0..i.segmSize as usize {
                let data = self.get(&i.seg[j]);
                fs::write(fs.clone(), data).unwrap();
            }
            println!("{} done", fs.as_path().to_str().unwrap());
        }
    }
}
pub fn unpack(buf: &Vec<u8>) -> Result<XP3, ()> {
    let (Header, EndOfHeader) = Header::unpack(&buf);
    let (XP3Info, EndOfXP3Info) = XP3Info::unpack(&buf, Header.offset as usize);
    return Ok(XP3 {
        Header,
        XP3Info,
        buf: buf.clone(),
    });
}
