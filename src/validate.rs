use std::path::Path;

use failure;

use discovery::get_flac_files_in_dir;
use offset::calc_disc_info_for_files;
use fetch::get_ar_bin;
use fetch::unpack_ar_bin;
use crc::calc_ar_crcs;
use decode::decode_flac_file;
use util::LookaheadExt;

pub fn validate<P: AsRef<Path>>(flac_dir: P) -> Result<(), failure::Error> {
    let flac_files = get_flac_files_in_dir(flac_dir)?;

    let disc_info = calc_disc_info_for_files(&flac_files)?;

    let ar_bin_data = get_ar_bin(&disc_info)?;

    let bin_results = unpack_ar_bin(&ar_bin_data)?;

    // Iterate over each item in bin results.
    for bin_result in bin_results {
        let (bin_disc_info, track_results) = bin_result;
        println!("{:#?}", track_results);
        assert_eq!(bin_disc_info, disc_info);
    }

    for (lookahead_pos, flac_file) in flac_files.iter().lookahead() {
        let samples = decode_flac_file(flac_file)?;

        let crcs = calc_ar_crcs(&samples, lookahead_pos.is_start(), lookahead_pos.is_end())?;

        println!("{:?}", crcs);
    }

    // println!("{:?}", disc_info);

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
