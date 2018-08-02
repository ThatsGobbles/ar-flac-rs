use std::path::Path;

use metaflac::Tag;
use metaflac::block::{Block, BlockType};

const SAMPLES_PER_FRAME: u64 = 588;

#[derive(Clone, Copy)]
pub struct FrameInfo {
    length: u64,
    offset: u64,
}

pub fn get_track_num_frames<P: AsRef<Path>>(flac_path: P) -> u64 {
    let flac_tag = Tag::read_from_path(flac_path).unwrap();

    let info_blocks = flac_tag.get_blocks(BlockType::StreamInfo);

    if let Some(info_block) = info_blocks.first() {
        match info_block {
            Block::StreamInfo(ref stream_info_block) => {
                let num_samples: u64 = stream_info_block.total_samples;

                let num_frames = (num_samples / SAMPLES_PER_FRAME)
                            + (if num_samples % SAMPLES_PER_FRAME == 0 {0} else {1});

                num_frames
            }
            _ => 0,
        }
    }
    else {0}
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::get_track_num_frames;

    #[test]
    fn test_get_track_num_frames() {
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
            let produced = get_track_num_frames(input);
            assert_eq!(expected, produced);
        }
    }
}
