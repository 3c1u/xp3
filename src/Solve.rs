use super::Header;
use super::Segment;
use super::XP3Info;
use super::extent;
use libflate::zlib::Decoder;
use std::fs;
use std::path::Path;
use std::io::Read;
pub struct XP3 {
    Header: Header::Header,
    XP3Info: XP3Info::XP3Info,
    buf: Vec<u8>,
}
impl XP3 {
    pub fn get(&self, sg: &Segment::Segment) -> Vec<u8> {
        let mut Raw=self.buf[sg.offset as usize..(sg.offset+sg.storageSize)as usize].to_vec();
        if sg.flag==1 {
            let mut decode = Decoder::new(&Raw[..]).unwrap();
            let mut copy = Vec::new();
            decode.read_to_end(&mut copy).unwrap();
            Raw = copy;
        }
        assert_eq!(Raw.len(),sg.originSize as usize);
        extent::decode(&mut Raw);
        Raw
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
