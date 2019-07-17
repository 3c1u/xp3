use super::utils;
pub struct Segment {
    pub flag: u32,
    pub offset: u64,
    pub originSize: u64,
    pub storageSize: u64,
}
pub fn unpack(buf: &Vec<u8>, mut o: usize) -> (Segment, usize) {
    let flag = utils::ReadU32(buf, &mut o);
    let offset = utils::ReadU64(buf, &mut o);
    let originSize = utils::ReadU64(buf, &mut o);
    let storageSize = utils::ReadU64(buf, &mut o);
    (
        Segment {
            flag,
            offset,
            originSize,
            storageSize,
        },
        o,
    )
}
