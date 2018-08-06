use reqwest;

use offset::DiscInfo;

const ACCURATERIP_DB_URL: &str = "http://www.accuraterip.com/accuraterip";

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

pub fn get_ar_bin(disc_info: &DiscInfo) {

}

#[cfg(test)]
mod tests {
    use offset::DiscInfo;

    use super::create_ar_bin_url;

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
}
