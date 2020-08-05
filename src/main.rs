use clap::App;
use clap::Arg;
use std::fs;

use xp3::solve;

fn main() {
    let matches = App::new("XP3Parser")
        .version("1.0.0")
        .author("9646516 <zyq855@gmail.com>")
        .arg(
            Arg::with_name("source")
                .short("s")
                .value_name("SRC")
                .help("Path of XP3 File")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .value_name("DEST")
                .help("Path of Output Directory")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // enable logger
    simple_logger::init().unwrap();

    let path = matches
        .value_of("source")
        .expect("no archive files are given");
    let desk = matches
        .value_of("destination")
        .expect("no destination directory is given");
    let data = fs::read(path).expect("invalid path");
    let res = solve::unpack(&data).expect("parse failed");
    // res.extract(desk);
}
