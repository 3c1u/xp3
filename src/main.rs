#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::fs;
use clap::Arg;
use clap::App;

pub mod Header;
pub mod Segment;
pub mod Solve;
pub mod XP3File;
pub mod XP3Info;
pub mod extent;
pub mod utils;
fn main() {
    let matches = App::new("XP3Parser")
        .version("1.0.0")
        .author("9646516 <zyq855@gmail.com>")
        .arg(Arg::with_name("Sourse")
            .short("s")
            .value_name("Sourse")
            .help("Path of XP3 File")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("Desk")
            .short("d")
            .value_name("Desk")
            .help("Path of Output Directory")
            .takes_value(true)
            .required(true)
        )
        .get_matches();
    let path = matches.value_of("Sourse").expect("Need Sourse");
    let desk = matches.value_of("Desk").expect("Need Desk");
    let data = fs::read(path).expect("path not valid");
    let res = Solve::unpack(&data).expect("Exact Failed");
    res.extract(desk);
}
