pub fn part1(data: &str) -> String {
    run(data, 40).to_string()
}

pub fn part2(data: &str) -> String {
    run(data, 50).to_string()
}

fn run(data: &str, n: u32) -> u32 {
    let nums = collapse(parse(data));
    let expanded = expand_n(nums, n);

    expanded.iter().map(|cn| cn.count).sum()
}

const ASCII_TO_INT: u8 = 48u8;
type Count = u32;
type N = u8;

#[derive(Debug, PartialEq)]
struct CountN {
    count: Count,
    n: N,
}

impl CountN {
    fn new(count: Count, n: N) -> Self {
        Self { count, n }
    }
    fn unit(n: N) -> Self {
        Self::new(1, n)
    }

    fn append(self, other: Self) -> CountNAppendResult {
        if self.n == other.n {
            CountNAppendResult::Combined(Self {
                count: self.count + other.count,
                n: self.n,
            })
        } else {
            CountNAppendResult::Boundary(self, other)
        }
    }
}

impl ToString for CountN {
    fn to_string(&self) -> String {
        format!("{}{}", self.count, self.n)
    }
}

enum CountNAppendResult {
    Combined(CountN),
    Boundary(CountN, CountN),
}

fn parse(s: &str) -> Vec<CountN> {
    s.bytes()
        .map(|c| {
            assert!(c >= ASCII_TO_INT && c <= ASCII_TO_INT + 9);
            let n = (c - ASCII_TO_INT) as N;
            CountN::unit(n)
        })
        .collect()
}

fn collapse(count_ns: Vec<CountN>) -> Vec<CountN> {
    let (mut combined_count_ns, last_count_n) =
        count_ns
            .into_iter()
            .fold((vec![], None), |(mut acc, prev), count_n| match prev {
                None => (acc, Some(count_n)),
                Some(prev_count_n) => match prev_count_n.append(count_n) {
                    CountNAppendResult::Combined(new_count_n) => (acc, Some(new_count_n)),
                    CountNAppendResult::Boundary(old_count_n, new_count_n) => {
                        acc.push(old_count_n);
                        (acc, Some(new_count_n))
                    }
                },
            });
    combined_count_ns.push(last_count_n.unwrap());
    combined_count_ns
}

fn expand_n(count_ns: Vec<CountN>, n: u32) -> Vec<CountN> {
    let mut current = count_ns;
    for _ in 0..n {
        current = expand(current);
    }
    current
}

fn expand(count_ns: Vec<CountN>) -> Vec<CountN> {
    let expanded = count_ns
        .iter()
        .flat_map(|count_n| {
            let mut count_digits = vec![];
            let mut current_count = count_n.count;
            loop {
                let digit = current_count % 10;
                count_digits.push(CountN::unit(digit.try_into().unwrap()));
                current_count = current_count / 10;
                if current_count == 0 {
                    break;
                }
            }
            count_digits.reverse();
            count_digits.push(CountN::unit(count_n.n));
            count_digits
        })
        .collect();
    collapse(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        // 1 becomes 11 (1 copy of digit 1).
        assert_eq!(parse("1"), vec![CountN::unit(1)]);

        // 11 becomes 21 (2 copies of digit 1).
        assert_eq!(parse("11"), vec![CountN::unit(1), CountN::unit(1)]);

        // 21 becomes 1211 (one 2 followed by one 1).
        assert_eq!(parse("21"), vec![CountN::unit(2), CountN::unit(1)]);

        // 1211 becomes 111221 (one 1, one 2, and two 1s).
        assert_eq!(
            parse("1211"),
            vec![
                CountN::unit(1),
                CountN::unit(2),
                CountN::unit(1),
                CountN::unit(1)
            ]
        );

        // 111221 becomes 312211 (three 1s, two 2s, and one 1).
        assert_eq!(
            parse("111221"),
            vec![
                CountN::unit(1),
                CountN::unit(1),
                CountN::unit(1),
                CountN::unit(2),
                CountN::unit(2),
                CountN::unit(1)
            ]
        );
    }

    #[test]
    fn collapse_test() {
        // 1 becomes 11 (1 copies of digit 1).
        assert_eq!(collapse(vec![CountN::unit(1)]), vec![CountN::unit(1)]);

        // 11 becomes 21 (2 copies of digit 1).
        assert_eq!(
            collapse(vec![CountN::unit(1), CountN::unit(1)]),
            vec![CountN::new(2, 1)]
        );

        // 21 becomes 1211 (one 2 followed by one 1).
        assert_eq!(
            collapse(vec![CountN::unit(2), CountN::unit(1)]),
            vec![CountN::unit(2), CountN::unit(1)]
        );

        // 1211 becomes 111221 (one 1, one 2, and two 1s).
        assert_eq!(
            collapse(vec![
                CountN::unit(1),
                CountN::unit(2),
                CountN::unit(1),
                CountN::unit(1)
            ]),
            vec![CountN::unit(1), CountN::unit(2), CountN::new(2, 1)]
        );

        // 111221 becomes 312211 (three 1s, two 2s, and one 1).
        assert_eq!(
            collapse(vec![
                CountN::unit(1),
                CountN::unit(1),
                CountN::unit(1),
                CountN::unit(2),
                CountN::unit(2),
                CountN::unit(1)
            ]),
            vec![CountN::new(3, 1), CountN::new(2, 2), CountN::unit(1)]
        );
    }

    #[test]
    fn expand_test() {
        // 1 becomes 11 (2 copies of digit 1).
        assert_eq!(expand(vec![CountN::unit(1)]), vec![CountN::new(2, 1)]);

        // 11 becomes 21 (2 copies of digit 1).
        assert_eq!(
            expand(vec![CountN::new(2, 1)]),
            vec![CountN::unit(2), CountN::unit(1)]
        );

        // 21 becomes 1211 (one 2 followed by one 1).
        assert_eq!(
            expand(vec![CountN::unit(2), CountN::unit(1)]),
            vec![CountN::unit(1), CountN::unit(2), CountN::new(2, 1)]
        );

        // 1211 becomes 111221 (one 1, one 2, and two 1s).
        assert_eq!(
            expand(vec![CountN::unit(1), CountN::unit(2), CountN::new(2, 1)]),
            vec![CountN::new(3, 1), CountN::new(2, 2), CountN::unit(1)]
        );

        // 111221 becomes 312211 (three 1s, two 2s, and one 1).
        assert_eq!(
            expand(vec![CountN::new(3, 1), CountN::new(2, 2), CountN::unit(1)]),
            vec![
                CountN::unit(3),
                CountN::unit(1),
                CountN::new(2, 2),
                CountN::new(2, 1)
            ]
        );
    }
}
