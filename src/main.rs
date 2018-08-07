#![feature(attr_literals)]

extern crate clap;
extern crate glob;
extern crate metaflac;
#[macro_use] extern crate failure;
extern crate reqwest;
extern crate byteorder;

extern crate test_util;

mod discovery;
mod offset;
mod util;
mod fetch;
mod crc;

use clap::{Arg, App};

fn main() {
    let matches = App::new("ar-flac-rs")
                    .version("1.0")
                    .about("Validates FLAC files against the online AccurateRip database")
                    .arg(
                        Arg::with_name("FLAC_DIR")
                        .help("path to directory of FLAC files to validate")
                    )
                    .get_matches();

    println!("Using FLAC_DIR: {}", matches.value_of("FLAC_DIR").unwrap());
}
