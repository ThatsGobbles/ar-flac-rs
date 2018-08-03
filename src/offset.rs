//! Handles calculating frame offset and disc ids from music files.

use std::path::Path;

use metaflac::Tag;
use metaflac::block::{Block, BlockType};
use failure::Error;

use util::sum_digits;

const SAMPLES_PER_FRAME: u64 = 588;
const FRAMES_PER_SECOND: u64 = 75;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiscInfo {
    id_1: u32,
    id_2: u32,
    cddb_id: u32,
}

pub fn get_num_frames<P: AsRef<Path>>(flac_path: P) -> Result<u64, Error> {
    let flac_tag = Tag::read_from_path(flac_path)?;

    let info_blocks = flac_tag.get_blocks(BlockType::StreamInfo);

    if let Some(Block::StreamInfo(stream_info_block)) = info_blocks.first() {
        let num_samples: u64 = stream_info_block.total_samples;

        let num_frames = (num_samples / SAMPLES_PER_FRAME)
                        + (if num_samples % SAMPLES_PER_FRAME == 0 {0} else {1});

        Ok(num_frames)
    }
    else {
        bail!("no stream info block found");
    }
}

pub fn get_frame_offsets<P: AsRef<Path>, II: IntoIterator<Item = P>>(flac_paths: II) -> Result<Vec<u64>, Error> {
    let mut curr_frame_offset = 0u64;
    let mut frame_offsets: Vec<u64> = vec![];

    for flac_path in flac_paths {
        let num_frames = get_num_frames(flac_path)?;
        curr_frame_offset += num_frames;
        frame_offsets.push(curr_frame_offset)
    }

    Ok(frame_offsets)
}

pub fn get_disc_ids<P: AsRef<Path>, II: IntoIterator<Item = P>>(flac_paths: II) -> Result<DiscInfo, Error> {
    let frame_offsets = get_frame_offsets(flac_paths)?;
    let num_tracks = frame_offsets.len();

    let mut id_1: u64 = 0;
    let mut id_2: u64 = 1;
    let mut cddb_id: u64 = 2;

    for (i, frame_offset) in frame_offsets.iter().enumerate() {
        id_1 += frame_offset;
        id_2 += (if *frame_offset > 0 {*frame_offset} else {1u64}) as u64 * (i as u64 + 2);

        // If this is not the last frame offset, adjust the CDDB id.
        if i < (num_tracks - 1) {
            cddb_id += sum_digits(frame_offset / FRAMES_PER_SECOND + 2);
        }
    }

    // Some additional magic on CDDB id.
    if let Some(last_frame_offset) = frame_offsets.last() {
        cddb_id = ((cddb_id % 255) << 24)
                + ((last_frame_offset / FRAMES_PER_SECOND) << 8)
                + num_tracks as u64;
    }

    id_1 &= 0xFFFFFFFF;
    id_2 &= 0xFFFFFFFF;
    cddb_id &= 0xFFFFFFFF;

    Ok(DiscInfo {
        id_1: id_1 as u32,
        id_2: id_2 as u32,
        cddb_id: cddb_id as u32,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::iter::Sum;

    use super::get_num_frames;
    use super::get_frame_offsets;
    use super::get_disc_ids;
    use super::DiscInfo;

    const EXPECTED_OFFSETS: &[u64] = &[
        24882,
        21328,
        25617,
        19155,
        16888,
        25512,
        23685,
        20160,
        23518,
        16502,
    ];

    #[test]
    fn test_get_num_frames() {
        // Current working dir is crate root, same dir Cargo.toml is in.
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        let inputs_and_expected = vec![
            (flac_dir.join("01.flac"), EXPECTED_OFFSETS[0]),
            (flac_dir.join("02.flac"), EXPECTED_OFFSETS[1]),
            (flac_dir.join("03.flac"), EXPECTED_OFFSETS[2]),
            (flac_dir.join("04.flac"), EXPECTED_OFFSETS[3]),
            (flac_dir.join("05.flac"), EXPECTED_OFFSETS[4]),
            (flac_dir.join("06.flac"), EXPECTED_OFFSETS[5]),
            (flac_dir.join("07.flac"), EXPECTED_OFFSETS[6]),
            (flac_dir.join("08.flac"), EXPECTED_OFFSETS[7]),
            (flac_dir.join("09.flac"), EXPECTED_OFFSETS[8]),
            (flac_dir.join("10.flac"), EXPECTED_OFFSETS[9]),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = get_num_frames(input).unwrap();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn test_get_frame_offsets() {
        // Current working dir is crate root, same dir Cargo.toml is in.
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        let inputs_and_expected: Vec<(_, Vec<u64>)> = vec![
            (
                vec![
                    flac_dir.join("01.flac"),
                    flac_dir.join("02.flac"),
                ],
                vec![
                    EXPECTED_OFFSETS[..=0].iter().sum(),
                    EXPECTED_OFFSETS[..=1].iter().sum(),
                ],
            ),
            (
                vec![
                    flac_dir.join("01.flac"),
                    flac_dir.join("02.flac"),
                    flac_dir.join("03.flac"),
                    flac_dir.join("04.flac"),
                    flac_dir.join("05.flac"),
                    flac_dir.join("06.flac"),
                    flac_dir.join("07.flac"),
                    flac_dir.join("08.flac"),
                    flac_dir.join("09.flac"),
                    flac_dir.join("10.flac"),
                ],
                vec![
                    EXPECTED_OFFSETS[..=0].iter().sum(),
                    EXPECTED_OFFSETS[..=1].iter().sum(),
                    EXPECTED_OFFSETS[..=2].iter().sum(),
                    EXPECTED_OFFSETS[..=3].iter().sum(),
                    EXPECTED_OFFSETS[..=4].iter().sum(),
                    EXPECTED_OFFSETS[..=5].iter().sum(),
                    EXPECTED_OFFSETS[..=6].iter().sum(),
                    EXPECTED_OFFSETS[..=7].iter().sum(),
                    EXPECTED_OFFSETS[..=8].iter().sum(),
                    EXPECTED_OFFSETS[..=9].iter().sum(),
                ],
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = get_frame_offsets(input).unwrap();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn test_get_disc_ids() {
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        let inputs_and_expected = vec![
            (
                vec![
                    flac_dir.join("01.flac"),
                    flac_dir.join("02.flac"),
                    flac_dir.join("03.flac"),
                    flac_dir.join("04.flac"),
                    flac_dir.join("05.flac"),
                    flac_dir.join("06.flac"),
                    flac_dir.join("07.flac"),
                    flac_dir.join("08.flac"),
                    flac_dir.join("09.flac"),
                    flac_dir.join("10.flac"),
                ],
                DiscInfo {
                    id_1: 1227439,
                    id_2: 9760253,
                    cddb_id: 2332774410,
                },
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = get_disc_ids(input).unwrap();
            assert_eq!(expected, produced);
        }
    }
}
