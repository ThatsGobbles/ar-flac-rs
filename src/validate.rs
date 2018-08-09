use std::path::Path;

use failure;

use discovery::get_flac_files_in_dir;
use offset::calc_disc_info_for_files;

pub fn validate<P: AsRef<Path>>(flac_dir: P) -> Result<(), failure::Error> {
    let flac_files = get_flac_files_in_dir(flac_dir);

    let disc_info = calc_disc_info_for_files(&flac_files);

    println!("{:?}", disc_info);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::validate;

    #[test]
    fn test_create_ar_bin_url() {
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        validate(&flac_dir);
    }
}
