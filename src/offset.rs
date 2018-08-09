//! Handles calculating frame offset and disc ids from music files.

use std::path::Path;

use metaflac::Tag;
use metaflac::block::{Block, BlockType};
use failure::Error;
use std::cmp;

use util::sum_digits;
use util::LookaheadExt;

const SAMPLES_PER_SECOND: u64 = 44100;
const SAMPLES_PER_FRAME: u64 = 588;  // 44100 / 75
const FRAMES_PER_SECOND: u64 = 75;

pub type FrameLength = u64;
pub type FrameOffset = u64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DiscInfo {
    pub id_1: u32,
    pub id_2: u32,
    pub cddb_id: u32,
    pub num_tracks: u8,
}

pub fn get_frame_lengths<P: AsRef<Path>>(flac_path: P) -> Result<u64, Error> {
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

pub fn calc_frame_offsets<II: IntoIterator<Item = FrameLength>>(frame_lengths: II) -> Vec<FrameOffset> {
    let mut curr_frame_offset = 0u64;
    let mut frame_offsets: Vec<FrameOffset> = vec![];

    for frame_length in frame_lengths {
        curr_frame_offset += frame_length;
        frame_offsets.push(curr_frame_offset)
    }

    frame_offsets
}

pub fn calc_disc_info<II: IntoIterator<Item = FrameOffset>>(frame_offsets: II) -> DiscInfo {
    let mut id_1: u64 = 0;
    let mut id_2: u64 = 1;
    let mut cddb_id: u64 = 2;

    let mut track_count: u8 = 0;

    for (lookahead_pos, frame_offset) in frame_offsets.into_iter().lookahead() {
        // println!("{}, {}, {}", id_1, id_2, cddb_id);
        track_count += 1;

        id_1 += frame_offset;
        id_2 += cmp::max(frame_offset, 1u64) * (track_count + 1) as u64;

        // If this is not the last frame offset, adjust the CDDB id.
        if !lookahead_pos.is_end() {
            cddb_id += sum_digits(frame_offset / FRAMES_PER_SECOND + 2);
        }
        else {
            cddb_id = ((cddb_id % 255) << 24)
                    + ((frame_offset / FRAMES_PER_SECOND) << 8)
                    + track_count as u64;
        }
    }
    // println!("{}, {}, {}", id_1, id_2, cddb_id);

    id_1 &= 0xFFFFFFFF;
    id_2 &= 0xFFFFFFFF;
    cddb_id &= 0xFFFFFFFF;

    DiscInfo {
        id_1: id_1 as u32,
        id_2: id_2 as u32,
        cddb_id: cddb_id as u32,
        num_tracks: track_count,
    }
}

pub fn calc_disc_info_for_files<P: AsRef<Path>, II: IntoIterator<Item = P>>(flac_paths: II) -> Result<DiscInfo, Error> {
    let frame_lengths = flac_paths.into_iter().map(get_frame_lengths).collect::<Result<Vec<_>, _>>()?;
    let frame_offsets = calc_frame_offsets(frame_lengths);
    let disc_info = calc_disc_info(frame_offsets);

    Ok(disc_info)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::FrameLength;
    use super::FrameOffset;
    use super::get_frame_lengths;
    use super::calc_frame_offsets;
    use super::calc_disc_info;
    use super::DiscInfo;

    const EXPECTED_LENGTHS: &[FrameLength] = &[
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

    const EXPECTED_OFFSETS: &[FrameOffset] = &[
        24882,
        46210,
        71827,
        90982,
        107870,
        133382,
        157067,
        177227,
        200745,
        217247,
    ];

    #[test]
    fn test_get_frame_lengths() {
        // Current working dir is crate root, same dir Cargo.toml is in.
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        let inputs_and_expected = vec![
            (flac_dir.join("01.flac"), EXPECTED_LENGTHS[0]),
            (flac_dir.join("02.flac"), EXPECTED_LENGTHS[1]),
            (flac_dir.join("03.flac"), EXPECTED_LENGTHS[2]),
            (flac_dir.join("04.flac"), EXPECTED_LENGTHS[3]),
            (flac_dir.join("05.flac"), EXPECTED_LENGTHS[4]),
            (flac_dir.join("06.flac"), EXPECTED_LENGTHS[5]),
            (flac_dir.join("07.flac"), EXPECTED_LENGTHS[6]),
            (flac_dir.join("08.flac"), EXPECTED_LENGTHS[7]),
            (flac_dir.join("09.flac"), EXPECTED_LENGTHS[8]),
            (flac_dir.join("10.flac"), EXPECTED_LENGTHS[9]),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = get_frame_lengths(input).unwrap();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn test_calc_frame_offsets() {
        // Current working dir is crate root, same dir Cargo.toml is in.
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        let inputs_and_expected: Vec<(Vec<u64>, Vec<u64>)> = vec![
            (
                EXPECTED_LENGTHS[..=1].to_vec(),
                vec![
                    EXPECTED_LENGTHS[..=0].iter().sum(),
                    EXPECTED_LENGTHS[..=1].iter().sum(),
                ],
            ),
            (
                EXPECTED_LENGTHS[..=9].to_vec(),
                vec![
                    EXPECTED_LENGTHS[..=0].iter().sum(),
                    EXPECTED_LENGTHS[..=1].iter().sum(),
                    EXPECTED_LENGTHS[..=2].iter().sum(),
                    EXPECTED_LENGTHS[..=3].iter().sum(),
                    EXPECTED_LENGTHS[..=4].iter().sum(),
                    EXPECTED_LENGTHS[..=5].iter().sum(),
                    EXPECTED_LENGTHS[..=6].iter().sum(),
                    EXPECTED_LENGTHS[..=7].iter().sum(),
                    EXPECTED_LENGTHS[..=8].iter().sum(),
                    EXPECTED_LENGTHS[..=9].iter().sum(),
                ],
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = calc_frame_offsets(input);
            assert_eq!(expected, produced);
        }
    }

    #[test]
    fn test_calc_disc_info() {
        let inputs_and_expected = vec![
            (
                EXPECTED_OFFSETS.to_vec(),
                DiscInfo {
                    id_1: 1227439,
                    id_2: 9760253,
                    cddb_id: 2332774410,
                    num_tracks: 10,
                },
            ),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = calc_disc_info(input);
            assert_eq!(expected, produced);
        }
    }
}
