use std::path::Path;

use metaflac::Tag;
use metaflac::block::{Block, BlockType};

const SAMPLES_PER_FRAME: u64 = 588;

pub fn get_track_frame_offsets<P: AsRef<Path>>(flac_paths: &Vec<P>) -> Vec<u64> {
    let mut curr_frame_offset = 0u64;
    let mut frame_offsets: Vec<u64> = vec![curr_frame_offset];

    for flac_path in flac_paths {
        let flac_tag = Tag::read_from_path(flac_path).unwrap();

        let info_blocks = flac_tag.get_blocks(BlockType::StreamInfo);

        if let Some(info_block) = info_blocks.first() {
            match info_block {
                Block::StreamInfo(ref stream_info_block) => {
                    let num_samples: u64 = stream_info_block.total_samples;
                    let num_frames = (num_samples / SAMPLES_PER_FRAME)
                               + (if num_samples % SAMPLES_PER_FRAME == 0 {0} else {1});

                    curr_frame_offset += num_frames;

                    frame_offsets.push(curr_frame_offset);
                }
                _ => {},
            }
        }
    }

    frame_offsets
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::get_track_frame_offsets;

    #[test]
    fn test_get_track_frame_offsets() {
        // Current working dir is crate root, same dir Cargo.toml is in.
        let flac_dir = PathBuf::from("test_util").join("input").join("flac");

        let flac_paths = vec![
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
        ];

        let expected = vec![
            0u64,
            16435,
            39395,
            58569,
            78260,
            91700,
            110010,
            124495,
            143180,
            159915,
            181825,
            203435,
            237570,
        ];

        let produced = get_track_frame_offsets(&flac_paths);

        assert_eq!(expected, produced);
        assert_eq!(expected.len(), flac_paths.len() + 1);
    }
}
