include!(concat!(env!("OUT_DIR"), "/words.rs"));

pub fn key_to_name(input: &[u8; 32]) -> String {
    let mut buf = [0u8; 8];

    buf.copy_from_slice(&input[8..16]);
    let adjective = ADJECTIVES[usize::from_le_bytes(buf) % ADJECTIVES.len()];

    buf.copy_from_slice(&input[..8]);
    let animal = ANIMALS[usize::from_le_bytes(buf) % ANIMALS.len()];

    format!("{}{}", adjective, animal).replace(" ", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_name() {
        let result = key_to_name(&[
            132, 122, 1, 1, 1, 1, 1, 1, 32, 2, 2, 2, 3, 4, 5, 6, 7, 8, 2, 3, 4, 5, 3, 4, 4, 5, 6,
            4, 5, 6, 3, 2,
        ]);
        assert_eq!(result, "greyishLemur".to_string());

        let result = key_to_name(&[
            132, 12, 1, 1, 1, 1, 1, 1, 32, 2, 2, 2, 3, 5, 6, 6, 7, 8, 2, 3, 4, 5, 3, 4, 4, 5, 6, 4,
            5, 8, 3, 2,
        ]);
        assert_eq!(result, "staminalAyeAye".to_string());
    }
}
