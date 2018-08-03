//! Finds, filters, and sorts file paths in a directory to get the working set of FLAC files to use.
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
    extern crate tempfile;

    use std::fs::File;
    // use std::thread::sleep_ms;

    use self::tempfile::tempdir;

    use super::get_flac_files_in_dir;

    #[test]
    fn test_get_flac_files_in_dir() {
        let dir = tempdir().unwrap();

        for i in 1..9 {
            // This format spec creates strings of the form '001', '002', ...
            let fp = dir.path().join(format!("{:0>3}.flac", i));
            File::create(fp).unwrap();

            // Create extra files to test exclusion.
            let fp = dir.path().join(format!("{:0>3}.other", i));
            File::create(fp).unwrap();
        }

        let expected = vec![
            dir.path().join("001.flac"),
            dir.path().join("002.flac"),
            dir.path().join("003.flac"),
            dir.path().join("004.flac"),
            dir.path().join("005.flac"),
            dir.path().join("006.flac"),
            dir.path().join("007.flac"),
            dir.path().join("008.flac"),
        ];

        let produced = get_flac_files_in_dir(dir.path());

        assert_eq!(expected, produced);
    }
}
