//! Handles calculating frame offset and disc ids from music files.

use std::path::Path;

use metaflac::Tag;
use metaflac::block::{Block, BlockType};
use failure::Error;

const SAMPLES_PER_FRAME: u64 = 588;

#[derive(Clone, Copy)]
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

    Ok(DiscInfo { id_1: 0, id_2: 0, cddb_id: 0, })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::iter::Sum;

    use super::get_num_frames;
    use super::get_frame_offsets;

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
}
