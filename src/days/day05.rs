use std::collections::HashMap;

pub fn part1(data: &str) -> String {
    data.lines().filter(|s| is_nice(s)).count().to_string()
}

pub fn part2(data: &str) -> String {
    data.lines().filter(|s| is_nice2(s)).count().to_string()
}

const VOWELS: [u8; 5] = ['a' as u8, 'e' as u8, 'i' as u8, 'o' as u8, 'u' as u8];
const EXPECTED_VOWELS: u8 = 3;
const ILLEGAL_PAIRS: [&[u8]; 4] = [
    &"ab".as_bytes(),
    &"cd".as_bytes(),
    &"pq".as_bytes(),
    &"xy".as_bytes(),
];

fn is_nice(s: &str) -> bool {
    let mut vowel_count = 0;
    let mut repeated = false;

    let bytes = s.as_bytes();

    for (i, b) in bytes.iter().enumerate() {
        if i + 1 < bytes.len() {
            let pair = &bytes[i..=i + 1];

            if ILLEGAL_PAIRS.contains(&pair) {
                return false;
            }

            if pair[0] == pair[1] {
                repeated = true;
            }
        }

        if vowel_count < EXPECTED_VOWELS && VOWELS.contains(b) {
            vowel_count += 1;
        }
    }
    return vowel_count == EXPECTED_VOWELS && repeated;
}

fn is_nice2(s: &str) -> bool {
    let mut seen_pairs: HashMap<[u8; 2], usize> = HashMap::new();
    let mut duplicate_pair_found = false;
    let mut surrounded_char_found = false;

    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if !duplicate_pair_found && i + 1 < bytes.len() {
            let c1 = bytes[i];
            let c2 = bytes[i + 1];

            let pair = [c1, c2];

            match seen_pairs.get(&pair) {
                None => {
                    seen_pairs.insert(pair, i);
                }
                Some(position) => {
                    duplicate_pair_found = i - position > 1;
                }
            }
        }

        if !surrounded_char_found && i + 2 < bytes.len() {
            surrounded_char_found = bytes[i] == bytes[i + 2];
        }
        if duplicate_pair_found && surrounded_char_found {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nice_examples() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn is_nice2_examples() {
        assert!(is_nice2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice2("xxyxx"));
        assert!(!is_nice2("uurcxstgmygtbstg"));
        assert!(!is_nice2("ieodomkazucvgmuy"));
        assert!(!is_nice2("aaa"));
        assert!(is_nice2("aaaa"));
        assert!(is_nice2("xyxy"));
    }
}
