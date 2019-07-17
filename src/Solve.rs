use super::Header;
use super::XP3Info;

pub struct XP3 {
    Header: Header::Header,
    XP3Info: XP3Info::XP3Info,
}

pub fn unpack(buf: &Vec<u8>) -> Result<XP3, ()> {
    let (Header, EndOfHeader) = Header::unpack(&buf);
    let (XP3Info, EndOfXP3Info) = XP3Info::unpack(&buf, Header.offset as usize);

    return Ok(XP3 { Header, XP3Info });
}
