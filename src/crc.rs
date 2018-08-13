//! Calculates AccurateRip (v1 and v2) CRCs for local audio files.

use std::io;
use std::io::Read;

use failure;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use rayon::prelude::*;

// Note that 'frame' == 'sector'.
const BYTES_PER_FRAME: usize = 2352;
// const WORDS_PER_FRAME: u32 = BYTES_PER_FRAME / 4;

pub type CRC = u32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CRCs {
    v1: CRC,
    v2: CRC,
}

pub fn calc_ar_crcs(track_audio_bytes: &[u8], is_first: bool, is_last: bool) -> Result<CRCs, failure::Error> {
    let head_offset = if is_first { BYTES_PER_FRAME * 5 } else { 0 };
    let tail_offset = track_audio_bytes.len() - (if is_last { BYTES_PER_FRAME * 5 } else { 0 });

    let result = &track_audio_bytes.par_chunks(4).zip(1usize..usize::max_value()).map(
        |(chunk, multi)| {
            let mut cursor = io::Cursor::new(chunk);
            let sample = cursor.read_u32::<LittleEndian>()?;

            if multi >= head_offset && multi <= tail_offset {
                // Version 1 CRC.
                let ar_crc_v1 = (multi as u32).wrapping_mul(sample);

                // Version 2 CRC.
                let calc: u64 = sample as u64 * multi as u64;
                let calc_lo: u32 = (calc & 0xFFFFFFFF) as u32;
                let calc_hi: u32 = (calc / 0x100000000) as u32;

                let ar_crc_v2 = calc_hi.wrapping_add(calc_lo);

                Ok((ar_crc_v1, ar_crc_v2))
            }
            else {
                Ok((0, 0))
            }
        }
    ).try_fold_with((0u32, 0u32), |(ar_crc_v1_a, ar_crc_v2_a): (u32, u32), res_ar_crcs_b: Result<(u32, u32), failure::Error>| {
        res_ar_crcs_b.map(|(ar_crc_v1_b, ar_crc_v2_b)| { (ar_crc_v1_a.wrapping_add(ar_crc_v1_b), ar_crc_v2_a.wrapping_add(ar_crc_v2_b)) })
    }).try_reduce(|| (0, 0), |(v1_a, v2_a), (v1_b, v2_b)| {
        Ok((v1_a.wrapping_add(v1_b), v2_a.wrapping_add(v2_b)))
    })?;

    Ok(CRCs { v1: result.0, v2: result.1 })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

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
}
