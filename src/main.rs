use clap::App;
use clap::Arg;
use std::fs;

use xp3::solve;

fn main() {
    let matches = App::new("XP3Parser")
        .version("1.0.0")
        .author("9646516 <zyq855@gmail.com>")
        .arg(
            Arg::with_name("Source")
                .short("s")
                .value_name("Source")
                .help("Path of XP3 File")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("Desk")
                .short("d")
                .value_name("Desk")
                .help("Path of Output Directory")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let path = matches.value_of("Source").expect("Need Source");
    let desk = matches.value_of("Desk").expect("Need Desk");
    let data = fs::read(path).expect("path not valid");
    let res = solve::unpack(&data).expect("Exact Failed");
    res.extract(desk);
}
