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
    Ok(DiscInfo { id_1: 0, id_2: 0, cddb_id: 0, })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::get_num_frames;
    use super::get_frame_offsets;

    #[test]
    fn test_get_num_frames() {
        // Current working dir is crate root, same dir Cargo.toml is in.
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        let inputs_and_expected = vec![
            (flac_dir.join("01.flac"), 16435u64),
            (flac_dir.join("02.flac"), 22960),
            (flac_dir.join("03.flac"), 19174),
            (flac_dir.join("04.flac"), 19691),
            (flac_dir.join("05.flac"), 13440),
            (flac_dir.join("06.flac"), 18310),
            (flac_dir.join("07.flac"), 14485),
            (flac_dir.join("08.flac"), 18685),
            (flac_dir.join("09.flac"), 16735),
            (flac_dir.join("10.flac"), 21910),
            (flac_dir.join("11.flac"), 21610),
            (flac_dir.join("12.flac"), 34135),
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

        let inputs_and_expected = vec![
            (vec![flac_dir.join("01.flac")], vec![16435u64]),
            (
                vec![
                    flac_dir.join("01.flac"),
                    flac_dir.join("02.flac"),
                ],
                vec![
                    16435,
                    16435 + 22960,
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
                ],
                vec![
                    16435,
                    16435 + 22960,
                    16435 + 22960 + 19174,
                    16435 + 22960 + 19174 + 19691,
                    16435 + 22960 + 19174 + 19691 + 13440,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310,
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
                    flac_dir.join("11.flac"),
                    flac_dir.join("12.flac"),
                ],
                vec![
                    16435,
                    16435 + 22960,
                    16435 + 22960 + 19174,
                    16435 + 22960 + 19174 + 19691,
                    16435 + 22960 + 19174 + 19691 + 13440,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310
                          + 14485,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310
                          + 14485 + 18685,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310
                          + 14485 + 18685 + 16735,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310
                          + 14485 + 18685 + 16735 + 21910,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310
                          + 14485 + 18685 + 16735 + 21910 + 21610,
                    16435 + 22960 + 19174 + 19691 + 13440 + 18310
                          + 14485 + 18685 + 16735 + 21910 + 21610 + 34135,
                ],
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = get_frame_offsets(input).unwrap();
            assert_eq!(expected, produced);
        }
    }
}
