use std::io;

use reqwest;
use failure;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

use offset::DiscInfo;

const ACCURATERIP_DB_URL: &str = "http://www.accuraterip.com/accuraterip";

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TrackResult {
    confidence: u8,
    crc: u32,
    // _unused: u32,
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

pub fn unpack_ar_bin(ar_bin_data: &[u8]) -> Result<Vec<(DiscInfo, Vec<TrackResult>)>, failure::Error> {
    let expected_end_pos = ar_bin_data.len() as u64;
    let mut reader = io::Cursor::new(ar_bin_data);

    let mut results = vec![];

    // Multiple CRC sets from different pressings of the same album can be in the file.
    while reader.position() < expected_end_pos {
        // Unpack header/disc info.
        let num_tracks: u8 = reader.read_u8()?;
        let id_1: u32 = reader.read_u32::<LittleEndian>()?;
        let id_2: u32 = reader.read_u32::<LittleEndian>()?;
        let cddb_id: u32 = reader.read_u32::<LittleEndian>()?;

        let mut track_results = vec![];

        // Use number of tracks to determine how many track results to try and unpack.
        for _ in 0..num_tracks {
            let confidence: u8 = reader.read_u8()?;
            let crc: u32 = reader.read_u32::<LittleEndian>()?;

            // Consume an extra u32, this field is unused.
            let _ = reader.read_u32::<LittleEndian>()?;

            track_results.push(TrackResult { confidence, crc });
        }

        let result = (
            DiscInfo { num_tracks, id_1, id_2, cddb_id, },
            track_results,
        );

        results.push(result);
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
    use super::unpack_ar_bin;
    use super::TrackResult;

    use test_util::load_bytes;

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

    #[test]
    fn test_unpack_ar_bin() {
        let bin_dir = PathBuf::from("test_util").join("input").join("bin");

        let disc_info = DiscInfo {
            id_1: 1227439,
            id_2: 9760253,
            cddb_id: 2332774410,
            num_tracks: 10,
        };

        let inputs_and_expected = vec![
            (
                load_bytes(bin_dir.join("dBAR-010-0012baaf-0094edfd-8b0b500a.bin")),
                vec![
                    (disc_info, vec![TrackResult { confidence: 122, crc: 4158045718 }, TrackResult { confidence: 123, crc: 3175593300 }, TrackResult { confidence: 125, crc: 1895033188 }, TrackResult { confidence: 123, crc: 1209064292 }, TrackResult { confidence: 123, crc: 751048154 }, TrackResult { confidence: 122,crc: 2692720149 }, TrackResult { confidence: 122, crc: 3342672821 }, TrackResult { confidence: 119, crc: 41310113 }, TrackResult { confidence: 121, crc: 3288026773 }, TrackResult {confidence: 122, crc: 2772935668 }]),
                    (disc_info, vec![TrackResult { confidence: 119, crc: 3733010837 }, TrackResult { confidence: 120, crc: 3824549311 }, TrackResult { confidence: 119, crc: 1038071824 }, TrackResult { confidence: 120, crc: 1723505091 }, TrackResult { confidence: 122, crc: 841709511 }, TrackResult { confidence: 121,crc: 3087785059 }, TrackResult { confidence: 121, crc: 2819070029 }, TrackResult { confidence: 119, crc: 4152591618 }, TrackResult { confidence: 119, crc: 2834344192 }, TrackResult{ confidence: 120, crc: 1799856152 }]),
                    (disc_info, vec![TrackResult { confidence: 7, crc: 195270588 }, TrackResult { confidence: 7, crc: 1406996299 }, TrackResult { confidence: 7, crc: 2856919522 }, TrackResult { confidence: 7, crc: 922847482 }, TrackResult { confidence: 7, crc: 1817308841 }, TrackResult { confidence: 7, crc: 591909482 }, TrackResult { confidence: 7, crc: 4293473513 }, TrackResult { confidence: 7, crc: 1782062631 }, TrackResult { confidence: 7, crc: 1690457673 }, TrackResult { confidence: 7, crc: 267635404 }]),
                    (disc_info, vec![TrackResult { confidence: 6, crc: 599927819 }, TrackResult { confidence: 6, crc: 4186141418 }, TrackResult { confidence: 6, crc: 3997774640 }, TrackResult { confidence: 6, crc: 1094577568 }, TrackResult { confidence: 6, crc: 1231949243 }, TrackResult { confidence: 6, crc: 1230441369 }, TrackResult { confidence: 6, crc: 831851045 }, TrackResult { confidence: 6, crc: 4173584957 }, TrackResult { confidence: 6, crc: 2648999133 }, TrackResult { confidence: 6, crc: 52828644 }]),
                    (disc_info, vec![TrackResult { confidence: 5, crc: 1010580098 }, TrackResult { confidence: 5, crc: 527883804 }, TrackResult { confidence: 5, crc: 3126032370 }, TrackResult { confidence: 5, crc: 1598025112 }, TrackResult { confidence: 5, crc: 1131500734 }, TrackResult { confidence: 5, crc: 820524809 }, TrackResult { confidence: 5, crc: 1341858179 }, TrackResult { confidence: 5, crc: 3978224706 }, TrackResult { confidence: 5, crc: 2181907945 }, TrackResult { confidence: 5, crc: 3365060760 }]),
                    (disc_info, vec![TrackResult { confidence: 4, crc: 4176293296 }, TrackResult { confidence: 3, crc: 2025336919 }, TrackResult { confidence: 3, crc: 1963009237 }, TrackResult { confidence: 3, crc: 1409804699 }, TrackResult { confidence: 3, crc: 1702176030 }, TrackResult { confidence: 3, crc: 159713874 }, TrackResult { confidence: 3, crc: 488119717 }, TrackResult { confidence: 3, crc: 1569193125 }, TrackResult { confidence: 3, crc: 1203253894 }, TrackResult { confidence: 3, crc: 3565382153 }]),
                    (disc_info, vec![TrackResult { confidence: 3, crc: 584351520 }, TrackResult { confidence: 2, crc: 1405618354 }, TrackResult { confidence: 2, crc: 2029118016 }, TrackResult { confidence: 2, crc: 352129008 }, TrackResult { confidence: 2, crc: 1222628267 }, TrackResult { confidence: 2, crc: 1111697633 }, TrackResult { confidence: 2, crc: 3588528965 }, TrackResult { confidence: 2, crc: 1310084365 }, TrackResult { confidence: 2, crc: 301586749 }, TrackResult { confidence: 2, crc: 627203876 }]),
                    (disc_info, vec![TrackResult { confidence: 2, crc: 183228499 }, TrackResult { confidence: 2, crc: 1503510192 }, TrackResult { confidence: 2, crc: 507105173 }, TrackResult { confidence: 2, crc: 371887312 }, TrackResult { confidence: 2, crc: 691486092 }, TrackResult { confidence: 2, crc: 48272091 }, TrackResult { confidence: 2, crc: 3500325872 }, TrackResult { confidence: 2, crc: 601112659 }, TrackResult { confidence: 2, crc: 3539484266 }, TrackResult { confidence: 2, crc: 3514527732 }]),
                    (disc_info, vec![TrackResult { confidence: 2, crc: 4256079886 }, TrackResult { confidence: 0, crc: 0 }, TrackResult{ confidence: 0, crc: 0 }, TrackResult { confidence: 0, crc: 0 }, TrackResult { confidence: 0, crc: 0 }, TrackResult { confidence: 0, crc: 0 }, TrackResult { confidence: 0, crc: 0 }, TrackResult { confidence: 0, crc: 0 }, TrackResult { confidence: 0, crc: 0 }, TrackResult { confidence: 0, crc: 0 }]),
                ],
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = unpack_ar_bin(&input).unwrap();
            assert_eq!(expected, produced);
        }
    }
}
