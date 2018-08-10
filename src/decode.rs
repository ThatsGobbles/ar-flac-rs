//! Handles decoding encoding audio formats into raw samples.

use std::process::Command;
use std::process::Stdio;
use std::path::Path;
use std::io::Read;

use failure;

pub fn decode_flac_file<P: AsRef<Path>>(flac_path: P) -> Result<Vec<u8>, failure::Error> {
    let process = Command::new("flac")
                          .args(&[
                              "-d",
                              "-c",
                              "-f",
                              "--force-raw-format",
                              "--totally-silent",
                              "--endian=little",
                              "--sign=signed",
                          ])
                          .arg(flac_path.as_ref())
                          .stdout(Stdio::piped())
                          .spawn()?;

    let mut output = vec![];

    // TODO: Might be able to return the stdout object directly.
    process.stdout.unwrap().read_to_end(&mut output)?;

    Ok(output)
}
