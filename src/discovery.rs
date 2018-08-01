// Finds, filters, and sorts file paths in a directory to get the working set of FLAC files to use.
use std::path::{Path, PathBuf};

use glob::glob;

pub fn get_flac_files_in_dir<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let dir = dir.as_ref();
    let pattern = dir.join("*.flac");

    let mut res: Vec<_> = glob(&pattern.to_string_lossy()).unwrap().filter_map(Result::ok).collect();
    res.sort();

    res
}

#[cfg(test)]
mod tests {
    extern crate test_util;

    #[test]
    fn get_flac_files_in_dir() {
        test_util::shared_code();
    }
}
