#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs;

pub mod Header;
pub mod Segment;
pub mod Solve;
pub mod XP3File;
pub mod XP3Info;
pub mod utils;
pub mod extent;
fn main() {
    let path = "test/2.xp3";
    let data = fs::read(path).expect("path not valid");
    let res = Solve::unpack(&data).expect("Exact Failed");
    res.extract("./123");
}
