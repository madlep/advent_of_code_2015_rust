pub fn part1(data: &str) -> String {
    parser::parse(&data)
        .iter()
        .map(|present| present.wrapping_paper())
        .sum::<Feet>()
        .to_string()
}

pub fn part2(data: &str) -> String {
    parser::parse(&data)
        .iter()
        .map(|present| present.ribbon())
        .sum::<Feet>()
        .to_string()
}

type Feet = u32;

pub struct Present(Feet, Feet, Feet);

impl Present {
    fn wrapping_paper(&self) -> Feet {
        let Present(l, w, h) = self;
        let sides = [l * w, w * h, h * l];
        let smallest_side = sides.iter().min().unwrap();
        sides.iter().fold(0, |total, side| total + 2 * side) + smallest_side
    }

    fn ribbon(&self) -> Feet {
        let mut dims = [self.0, self.1, self.2];
        dims.sort();
        let wrap = dims[0] * 2 + dims[1] * 2;
        let bow: Feet = dims.iter().product();

        wrap + bow
    }
}

mod parser {
    use super::Present;
    use nom::{
        character::complete::char, character::complete::u32, combinator::map,
        multi::separated_list1, IResult,
    };

    pub fn parse(s: &str) -> Vec<Present> {
        let (_rest, dimensions) = presents(s).unwrap();
        dimensions
    }

    fn presents(s: &str) -> IResult<&str, Vec<Present>> {
        separated_list1(char('\n'), present)(s)
    }

    fn present(s: &str) -> IResult<&str, Present> {
        let p = separated_list1(char('x'), u32);
        let mut p = map(p, |dims| {
            assert!(dims.len() == 3);
            Present(dims[0], dims[1], dims[2])
        });
        p(s)
    }
}
