//! Calculates AccurateRip (v1 and v2) CRCs for local audio files.

use std::io;

use failure;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;

// Note that 'frame' == 'sector'.
const BYTES_PER_FRAME: u32 = 2352;
const WORDS_PER_FRAME: u32 = BYTES_PER_FRAME / 4;

pub type CRC = u32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CRCs {
    v1: CRC,
    v2: CRC,
}

pub fn calc_ar_crcs(track_audio_bytes: &[u8], is_first: bool, is_last: bool) -> Result<CRCs, failure::Error> {
    let mut reader = io::Cursor::new(track_audio_bytes);

    let head_pos = 0u64 + (if is_first { WORDS_PER_FRAME as u64 * 5 } else { 0 });
    let tail_pos = track_audio_bytes.len() as u64 - (if is_last { WORDS_PER_FRAME as u64 * 5 } else { 0 });

    let mut ar_crc_v1 = 0u32;
    let mut ar_crc_v2 = 0u32;
    let mut pos_multi = 1u32;

    while reader.position() < tail_pos {
        // One audio sample is 4 bytes from the reader.
        let sample: u32 = reader.read_u32::<LittleEndian>()?;

        if pos_multi as u64 >= head_pos && pos_multi as u64 <= tail_pos {
            // Version 1 CRC.
            ar_crc_v1 = ar_crc_v1.wrapping_add(pos_multi.wrapping_mul(sample));

            // Version 2 CRC.
            let calc: u64 = sample as u64 * pos_multi as u64;
            let calc_lo: u32 = (calc & 0xFFFFFFFF) as u32;
            let calc_hi: u32 = (calc / 0x100000000) as u32;

            ar_crc_v2 = ar_crc_v2.wrapping_add(calc_hi);
            ar_crc_v2 = ar_crc_v2.wrapping_add(calc_lo);
        }

        pos_multi += 1;
    }

    Ok(CRCs { v1: ar_crc_v1, v2: ar_crc_v2 })
}

pub fn calc_ar_v1_crc(track_audio_bytes: &[u8], is_first: bool, is_last: bool) -> Result<u32, failure::Error> {
    let mut reader = io::Cursor::new(track_audio_bytes);

    let head_pos = 0u64 + (if is_first { WORDS_PER_FRAME as u64 * 5 } else { 0 });
    let tail_pos = track_audio_bytes.len() as u64 - (if is_last { WORDS_PER_FRAME as u64 * 5 } else { 0 });

    let mut ar_crc = 0u32;
    let mut pos_multi = 1u32;

    while reader.position() < tail_pos {
        // One audio sample is 4 bytes from the reader.
        let sample: u32 = reader.read_u32::<LittleEndian>()?;

        if pos_multi as u64 >= head_pos && pos_multi as u64 <= tail_pos {
            ar_crc = ar_crc.wrapping_add(pos_multi.wrapping_mul(sample));
        }

        pos_multi += 1;
    }

    Ok(ar_crc)
}

pub fn calc_ar_v2_crc(track_audio_bytes: &[u8], is_first: bool, is_last: bool) -> Result<u32, failure::Error> {
    let mut reader = io::Cursor::new(track_audio_bytes);

    let head_pos = 0u64 + (if is_first { WORDS_PER_FRAME as u64 * 5 } else { 0 });
    let tail_pos = track_audio_bytes.len() as u64 - (if is_last { WORDS_PER_FRAME as u64 * 5 } else { 0 });

    let mut ar_crc = 0u32;
    let mut pos_multi = 1u32;

    while reader.position() < tail_pos {
        // One audio sample is 4 bytes from the reader.
        let sample: u32 = reader.read_u32::<LittleEndian>()?;

        if pos_multi as u64 >= head_pos && pos_multi as u64 <= tail_pos {
            let calc: u64 = sample as u64 * pos_multi as u64;
            let calc_lo: u32 = (calc & 0xFFFFFFFF) as u32;
            let calc_hi: u32 = (calc / 0x100000000) as u32;

            ar_crc = ar_crc.wrapping_add(calc_hi);
            ar_crc = ar_crc.wrapping_add(calc_lo);
        }

        pos_multi += 1;
    }

    Ok(ar_crc)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::calc_ar_v1_crc;
    use super::calc_ar_v2_crc;
    use super::calc_ar_crcs;
    use super::CRCs;

    use test_util::load_bytes;

    #[test]
    #[ignore("long test")]
    fn test_calc_ar_crcs() {
        let raw_dir = PathBuf::from("test_util").join("input").join("raw_samples");

        let inputs_and_expected = vec![
            (
                (load_bytes(raw_dir.join("01.raw")), true, false),
                CRCs { v1: 0xde813995u32, v2: 0xf7d6be16u32, },
            ),
            (
                (load_bytes(raw_dir.join("05.raw")), false, false),
                CRCs { v1: 0x322B77C7u32, v2: 0x2CC415DAu32, },
            ),
            (
                (load_bytes(raw_dir.join("10.raw")), false, true),
                CRCs { v1: 0xA547A3F4u32, v2: 0x6B47A018u32, },
            ),
        ];

        for ((bytes, is_first, is_last), expected) in inputs_and_expected {
            let produced = calc_ar_crcs(&bytes, is_first, is_last).unwrap();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    #[ignore("long test")]
    fn test_calc_ar_v1_crc() {
        let raw_dir = PathBuf::from("test_util").join("input").join("raw_samples");

        let inputs_and_expected = vec![
            (
                (load_bytes(raw_dir.join("01.raw")), true, false),
                0xde813995u32,
            ),
            (
                (load_bytes(raw_dir.join("05.raw")), false, false),
                0x322B77C7u32,
            ),
            (
                (load_bytes(raw_dir.join("10.raw")), false, true),
                0xA547A3F4u32,
            ),
        ];

        for ((bytes, is_first, is_last), expected) in inputs_and_expected {
            let produced = calc_ar_v1_crc(&bytes, is_first, is_last).unwrap();
            assert_eq!(expected, produced);
        }
    }

    #[test]
    #[ignore("long test")]
    fn test_calc_ar_v2_crc() {
        let raw_dir = PathBuf::from("test_util").join("input").join("raw_samples");

        let inputs_and_expected = vec![
            (
                (load_bytes(raw_dir.join("01.raw")), true, false),
                0xf7d6be16u32,
            ),
            (
                (load_bytes(raw_dir.join("05.raw")), false, false),
                0x2CC415DAu32,
            ),
            (
                (load_bytes(raw_dir.join("10.raw")), false, true),
                0x6B47A018u32,
            ),
        ];

        for ((bytes, is_first, is_last), expected) in inputs_and_expected {
            let produced = calc_ar_v2_crc(&bytes, is_first, is_last).unwrap();
            assert_eq!(expected, produced);
        }
    }
}
