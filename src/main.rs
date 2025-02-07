use clap::App;
use clap::Arg;
use std::fs;

use xp3::Xp3;
use xp3::cxdec::{CxDec, CxDecScheme};

fn main() {
    let matches = App::new("XP3Parser")
        .version("1.0.0")
        .author("9646516 <zyq855@gmail.com>")
        .arg(
            Arg::with_name("source")
                .short("s")
                .value_name("SRC")
                .help("path to an XP3 file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .value_name("DEST")
                .help("path of the putput directory")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("scheme")
                .short("S")
                .value_name("SCHEME")
                .help("path to the scheme file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // enable logger
    simple_logger::init().unwrap();

    let path = matches
        .value_of("source")
        .expect("no archive files are given");
    let dest = matches
        .value_of("destination")
        .expect("no destination directory is given");
    let scheme = matches
        .value_of("scheme")
        .expect("no scheme is given");
    let data = fs::read(path).expect("invalid path");
    let res = Xp3::open(&data).expect("parse failed");

    let scheme =
    CxDecScheme::open(scheme).expect("failed to load the scheme");
    let decoder = CxDec::new(&scheme);
    res.extract(dest, decoder);
}
