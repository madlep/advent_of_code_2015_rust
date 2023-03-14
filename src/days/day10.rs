use std::num::ParseIntError;
use std::str::FromStr;

pub fn part1(data: &str) -> String {
    run(data, 40).to_string()
}

pub fn part2(data: &str) -> String {
    run(data, 50).to_string()
}

fn run(data: &str, n: u32) -> u32 {
    LookAndSay::from_str(data).unwrap().expand_n(n).len()
}

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

enum CountNAppendResult {
    Combined(CountN),
    Boundary(CountN, CountN),
}

#[derive(Debug, PartialEq)]
struct LookAndSay {
    count_ns: Vec<CountN>,
}

impl LookAndSay {
    fn collapse(self) -> Self {
        let (mut count_ns, last_count_n) =
            self.count_ns
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
        count_ns.push(last_count_n.unwrap());
        Self { count_ns }
    }

    fn expand_n(self, n: u32) -> Self {
        let mut current = self;
        for _ in 0..n {
            current = current.expand();
        }
        current
    }

    fn expand(self) -> Self {
        let count_ns = self
            .count_ns
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

        Self { count_ns }.collapse()
    }

    fn len(&self) -> u32 {
        self.count_ns.iter().map(|cn| cn.count).sum()
    }
}

impl FromStr for LookAndSay {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count_ns = s
            .chars()
            .map(|c| c.to_string().parse().and_then(|n| Ok(CountN::unit(n))))
            .into_iter()
            .collect::<Result<Vec<CountN>, Self::Err>>()?;

        Ok(Self { count_ns }.collapse())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_test() {
        // 1 becomes 11 (1 copy of digit 1).
        assert_eq!(
            LookAndSay::from_str("1").unwrap(),
            LookAndSay {
                count_ns: vec![CountN::unit(1)]
            }
        );

        // 11 becomes 21 (2 copies of digit 1).
        assert_eq!(
            LookAndSay::from_str("11").unwrap(),
            LookAndSay {
                count_ns: vec![CountN::new(2, 1)]
            }
        );

        // 21 becomes 1211 (one 2 followed by one 1).
        assert_eq!(
            LookAndSay::from_str("21").unwrap(),
            LookAndSay {
                count_ns: vec![CountN::unit(2), CountN::unit(1)]
            }
        );

        // 1211 becomes 111221 (one 1, one 2, and two 1s).
        assert_eq!(
            LookAndSay::from_str("1211").unwrap(),
            LookAndSay {
                count_ns: vec![CountN::unit(1), CountN::unit(2), CountN::new(2, 1),]
            }
        );

        // 111221 becomes 312211 (three 1s, two 2s, and one 1).
        assert_eq!(
            LookAndSay::from_str("111221").unwrap(),
            LookAndSay {
                count_ns: vec![CountN::new(3, 1), CountN::new(2, 2), CountN::unit(1)]
            }
        );

        assert_eq!(
            LookAndSay::from_str("123a456").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }

    #[test]
    fn collapse_test() {
        // 1 becomes 11 (1 copies of digit 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::unit(1)]
            }
            .collapse(),
            LookAndSay {
                count_ns: vec![CountN::unit(1)]
            }
        );

        // 11 becomes 21 (2 copies of digit 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::unit(1), CountN::unit(1)]
            }
            .collapse(),
            LookAndSay {
                count_ns: vec![CountN::new(2, 1)]
            }
        );

        // 21 becomes 1211 (one 2 followed by one 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::unit(2), CountN::unit(1)]
            }
            .collapse(),
            LookAndSay {
                count_ns: vec![CountN::unit(2), CountN::unit(1)]
            }
        );

        // 1211 becomes 111221 (one 1, one 2, and two 1s).
        assert_eq!(
            LookAndSay {
                count_ns: vec![
                    CountN::unit(1),
                    CountN::unit(2),
                    CountN::unit(1),
                    CountN::unit(1)
                ]
            }
            .collapse(),
            LookAndSay {
                count_ns: vec![CountN::unit(1), CountN::unit(2), CountN::new(2, 1)]
            }
        );

        // 111221 becomes 312211 (three 1s, two 2s, and one 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![
                    CountN::unit(1),
                    CountN::unit(1),
                    CountN::unit(1),
                    CountN::unit(2),
                    CountN::unit(2),
                    CountN::unit(1)
                ]
            }
            .collapse(),
            LookAndSay {
                count_ns: vec![CountN::new(3, 1), CountN::new(2, 2), CountN::unit(1)]
            }
        );
    }

    #[test]
    fn expand_test() {
        // 1 becomes 11 (2 copies of digit 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::unit(1)]
            }
            .expand(),
            LookAndSay {
                count_ns: vec![CountN::new(2, 1)]
            }
        );

        // 11 becomes 21 (2 copies of digit 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::new(2, 1)]
            }
            .expand(),
            LookAndSay {
                count_ns: vec![CountN::unit(2), CountN::unit(1)]
            }
        );

        // 21 becomes 1211 (one 2 followed by one 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::unit(2), CountN::unit(1)]
            }
            .expand(),
            LookAndSay {
                count_ns: vec![CountN::unit(1), CountN::unit(2), CountN::new(2, 1)]
            }
        );

        // 1211 becomes 111221 (one 1, one 2, and two 1s).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::unit(1), CountN::unit(2), CountN::new(2, 1)]
            }
            .expand(),
            LookAndSay {
                count_ns: vec![CountN::new(3, 1), CountN::new(2, 2), CountN::unit(1)]
            }
        );

        // 111221 becomes 312211 (three 1s, two 2s, and one 1).
        assert_eq!(
            LookAndSay {
                count_ns: vec![CountN::new(3, 1), CountN::new(2, 2), CountN::unit(1)]
            }
            .expand(),
            LookAndSay {
                count_ns: vec![
                    CountN::unit(3),
                    CountN::unit(1),
                    CountN::new(2, 2),
                    CountN::new(2, 1)
                ]
            }
        );
    }
}
