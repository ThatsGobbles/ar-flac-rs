use std::io;

use reqwest;
use failure;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use offset::DiscInfo;

const ACCURATERIP_DB_URL: &str = "http://www.accuraterip.com/accuraterip";

pub struct TrackResult {
    confidence: u8,
    crc: u32,
    _unused: u32,
}

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

    match response.status() {
        reqwest::StatusCode::Ok => {
            let mut buffer: Vec<u8> = vec![];
            response.copy_to(&mut buffer)?;

            Ok(buffer)
        },
        reqwest::StatusCode::NotFound => {
            bail!("disc not present in database");
        },
        _ => {
            bail!("error when fetching bin file");
        },
    }
}

pub fn unpack_ar_bin(ar_bin_data: Vec<u8>) -> Result<Vec<(DiscInfo, )>, failure::Error> {
    let expected_end_pos = ar_bin_data.len() as u64;
    let mut reader = io::Cursor::new(ar_bin_data);

    let results = vec![];

    loop {
        // Check if we are at the expected end position.
        if reader.position() >= expected_end_pos {
            break;
        }

        // Unpack header/disc info.
        let num_tracks: u8 = reader.read_u8()?;
        let id_1: u32 = reader.read_u32::<LittleEndian>()?;
        let id_2: u32 = reader.read_u32::<LittleEndian>()?;
        let cddb_id: u32 = reader.read_u32::<LittleEndian>()?;

        // Use number of tracks to determine how many track results to try and unpack.
    }

    Ok(results)
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
