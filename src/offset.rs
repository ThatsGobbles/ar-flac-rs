use std::path::Path;

use metaflac::Tag;
use metaflac::block::BlockType;

pub fn _get_track_offsets<P: AsRef<Path>>(flac_paths: Vec<P>) -> Vec<u32> {
    for flac_path in flac_paths {
        let flac_tag = Tag::read_from_path(flac_path).unwrap();

        let _stream_info = flac_tag.get_blocks(BlockType::StreamInfo);
    }

    vec![]
}
