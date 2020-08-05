use super::utils;

pub struct Segment {
    pub flag: u32,
    pub offset: u64,
    pub origin_size: u64,
    pub storage_size: u64,
}

pub fn unpack(buf: &Vec<u8>, mut o: usize) -> (Segment, usize) {
    let flag = utils::read_u32(buf, &mut o);
    let offset = utils::read_u64(buf, &mut o);
    let origin_size = utils::read_u64(buf, &mut o);
    let storage_size = utils::read_u64(buf, &mut o);
    (
        Segment {
            flag,
            offset,
            origin_size,
            storage_size,
        },
        o,
    )
}
