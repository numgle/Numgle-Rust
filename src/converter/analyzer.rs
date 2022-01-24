use std::result::Iter;

use rocket::figment::value::Num;

pub fn get_onset_index(char: char) -> u32 {
    (char as u32 - '가' as u32) / 28 / 21
}

pub fn get_nucleus_index(char: char) -> u32 {
    (char as u32 - '가' as u32) / 28 % 21
}

pub fn get_coda_index(char: char) -> u32 {
    (char as u32 - '가' as u32) % 28
}

pub fn get_not_complete_hangul_index(char: char) -> u32 {
    (char as u32 - 'ㄱ' as u32)
}

pub fn get_lower_engish_index(char: char) -> u32 {
    char as u32 - 'a' as u32
}

pub fn get_upper_engish_index(char: char) -> u32 {
    char as u32 - 'A' as u32
}

pub fn get_number_index(char: char) -> u32 {
    char as u32 - '1' as u32
}

pub fn is_char_in_range(range: (u32, u32)) -> impl Fn(char) -> bool {
    move |char| {
        let int = char as u32;
        range.0 <= int && int <= range.1
    }
}

pub fn is_char_in_array(array:Vec<u32>) -> impl Fn(char) -> bool{
    move |char| {
        let int = char as u32;
        array.contains(&int)
    }
}

#[cfg(test)]
mod test {

    use crate::converter::{
        analyzer::{get_coda_index, get_nucleus_index, get_number_index, get_onset_index},
    };

    use super::is_char_in_range;

    const COMPLETE_HANGUL_RANGE: (u32, u32) = (44032, 55203);
    const NOT_COMPLETE_HANGUL_RANGE: (u32, u32) = (0x3131, 0x3163);
    const UPPER_RANGE: (u32, u32) = (65, 90);
    const LOWER_RANGE: (u32, u32) = (97, 122);
    const NUMBER_RANGE: (u32, u32) = (48, 57);
    const TEST_HANGUL_CHARS:&str = "가나다라마바사힣힠힡힢힣";
    const ALPHABET_CHARS:&str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const NUBMERS:&str = "1234567890";
    #[test]
    fn check_number_offset() {
        assert_eq!(get_number_index('1'), 0);
    }
    #[test]
    fn test_onset() {
        assert_eq!(0,get_onset_index('가'));
        assert_eq!(get_onset_index('구'),get_onset_index('굴'));
        assert_eq!(get_onset_index('선'),get_onset_index('생'));
    }
    #[test]
    fn test_nuclues() {
        assert_eq!(0, get_nucleus_index('각'));
        assert_eq!(get_coda_index('각'),get_coda_index('박'));
        assert_eq!(get_coda_index('중'),get_coda_index('궁'));
    }
    #[test]
    fn test_coda() {
        assert_eq!(1, get_coda_index('각'));
        assert_eq!(get_coda_index('각'),get_coda_index('박'));
        assert_eq!(get_coda_index('힣'),get_coda_index('즇'));
    }

    #[test]
    fn complete_hangul_in_range() {
        TEST_HANGUL_CHARS.chars().for_each(|c|{
            assert!(is_char_in_range(COMPLETE_HANGUL_RANGE)(c))
        });
        ALPHABET_CHARS.chars().for_each(|c|{
            assert!(!is_char_in_range(COMPLETE_HANGUL_RANGE)(c))
        });
    }
    #[test]
    fn english_in_range() {
        let upper = is_char_in_range(UPPER_RANGE);
        let lower = is_char_in_range(LOWER_RANGE);
        ALPHABET_CHARS.chars().for_each(|c|{
            assert!(upper(c)||lower(c));
        });
    }
    #[test]
    fn number_in_range() {
        let num = is_char_in_range(NUMBER_RANGE);
        NUBMERS.chars().for_each(|c|{
            assert!(num(c));
        });
    }
}
