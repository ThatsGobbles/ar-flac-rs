use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn load_bytes<P: AsRef<Path>>(bin_path: P) -> Vec<u8> {
    let mut f = File::open(bin_path).unwrap();
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).unwrap();

    buffer
}
