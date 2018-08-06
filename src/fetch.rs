use reqwest;
use failure;

use offset::DiscInfo;

const ACCURATERIP_DB_URL: &str = "http://www.accuraterip.com/accuraterip";

fn create_ar_bin_url(disc_info: &DiscInfo) -> String {
    format!(
        "{}/{:x}/{:x}/{:x}/dBAR-{:0>3}-{:0>8x}-{:0>8x}-{:0>8x}.bin",
        ACCURATERIP_DB_URL,
        disc_info.id_1 & 0xF,
        disc_info.id_1 >> 4 & 0xF,
        disc_info.id_1 >> 8 & 0xF,
        disc_info.num_tracks,
        disc_info.id_1,
        disc_info.id_2,
        disc_info.cddb_id,
    )
}

pub fn get_ar_bin(disc_info: &DiscInfo) -> Result<Vec<u8>, failure::Error> {
    let url = create_ar_bin_url(disc_info);

    let mut response = reqwest::get(&url)?;
    let mut buffer: Vec<u8> = vec![];
    response.copy_to(&mut buffer);

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::Path;
    use std::path::PathBuf;
    use std::io::Read;

    use offset::DiscInfo;

    use super::create_ar_bin_url;
    use super::get_ar_bin;

    fn load_bytes<P: AsRef<Path>>(bin_path: P) -> Vec<u8> {
        let mut f = File::open(bin_path).unwrap();
        let mut buffer = Vec::new();

        f.read_to_end(&mut buffer).unwrap();

        buffer
    }

    #[test]
    fn test_create_ar_bin_url() {
        let inputs_and_expected = vec![
            (
                DiscInfo { id_1: 1227439, id_2: 9760253, cddb_id: 2332774410, num_tracks: 10 },
                "http://www.accuraterip.com/accuraterip/f/a/a/dBAR-010-0012baaf-0094edfd-8b0b500a.bin",
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = create_ar_bin_url(&input);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    #[ignore("pulls data from AccurateRip server")]
    fn test_get_ar_bin() {
        let bin_dir = PathBuf::from("test_util").join("input").join("bin");

        let inputs_and_expected = vec![
            (
                DiscInfo { id_1: 1227439, id_2: 9760253, cddb_id: 2332774410, num_tracks: 10 },
                load_bytes(bin_dir.join("dBAR-010-0012baaf-0094edfd-8b0b500a.bin")),
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = get_ar_bin(&input).unwrap();
            assert_eq!(expected, produced);
        }
    }
}
