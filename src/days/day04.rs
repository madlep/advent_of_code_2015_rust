use md5;

pub fn part1(data: &str) -> String {
    let data = data.trim();

    let mut i = 0;
    let byte3_bits = 0b11110000u8;
    loop {
        let padded = data.to_owned() + &i.to_string();

        let result = md5::compute(padded);

        if &result[0..=1] == [0, 0] && &result[2] & byte3_bits == 0 {
            return i.to_string();
        }
        i += 1;
    }
}

pub fn part2(data: &str) -> String {
    let data = data.trim();

    let mut i = 0;
    loop {
        let padded = data.to_owned() + &i.to_string();

        let result = md5::compute(padded);

        if &result[0..=2] == [0, 0, 0] {
            return i.to_string();
        }
        i += 1;
    }
}
