pub fn part1(data: &str) -> String {
    let instructions = parser::parse(data);

    let mut turned_on_count = 0u32;
    for x in 0..=999 {
        for y in 0..=999 {
            let coord = Coord2(x, y);
            let mut house = false;
            for instruction in instructions.iter() {
                if instruction.1.is_coord_contained(&coord) {
                    house = match instruction.0 {
                        Op::TurnOn => true,
                        Op::TurnOff => false,
                        Op::Toggle => !house,
                    }
                }
            }
            if house {
                turned_on_count += 1;
            }
        }
    }

    turned_on_count.to_string()
}

pub fn part2(data: &str) -> String {
    let instructions = parser::parse(data);

    let mut total_brightness = 0u32;
    for x in 0..=999 {
        for y in 0..=999 {
            let coord = Coord2(x, y);
            let mut house_brightness = 0u32;
            for instruction in instructions.iter() {
                if instruction.1.is_coord_contained(&coord) {
                    match instruction.0 {
                        Op::TurnOn => house_brightness += 1,
                        Op::TurnOff => {
                            if house_brightness > 0 {
                                house_brightness -= 1
                            }
                        }
                        Op::Toggle => house_brightness += 2,
                    }
                }
            }
            total_brightness += house_brightness;
        }
    }

    total_brightness.to_string()
}

#[derive(Debug)]
pub struct Instruction(Op, Area);

#[derive(Clone, Debug)]
enum Op {
    TurnOn,
    TurnOff,
    Toggle,
}

type Coord = u32;

#[derive(Debug)]
pub struct Coord2(Coord, Coord);
impl Coord2 {
    fn x(&self) -> Coord {
        self.0
    }

    fn y(&self) -> Coord {
        self.1
    }
}

#[derive(Debug)]
pub struct Area(Coord2, Coord2);

impl Area {
    fn is_coord_contained(&self, coord: &Coord2) -> bool {
        coord.x() >= self.0.x()
            && coord.x() <= self.1.x()
            && coord.y() >= self.0.y()
            && coord.y() <= self.1.y()
    }
}

mod parser {
    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, u32},
        combinator::{map, value},
        multi::separated_list0,
        sequence::separated_pair,
        IResult,
    };

    pub fn parse(s: &str) -> Vec<Instruction> {
        let (rest, ins) = instructions(s).unwrap();
        assert!(rest.is_empty());
        ins
    }

    fn instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list0(char('\n'), instruction)(s)
    }

    fn instruction(s: &str) -> IResult<&str, Instruction> {
        let p = separated_pair(op, char(' '), area);
        map(p, |(op, area)| Instruction(op, area))(s)
    }

    fn op(s: &str) -> IResult<&str, Op> {
        alt((turn_on, turn_off, toggle))(s)
    }

    fn turn_on(s: &str) -> IResult<&str, Op> {
        value(Op::TurnOn, tag("turn on"))(s)
    }

    fn turn_off(s: &str) -> IResult<&str, Op> {
        value(Op::TurnOff, tag("turn off"))(s)
    }

    fn toggle(s: &str) -> IResult<&str, Op> {
        value(Op::Toggle, tag("toggle"))(s)
    }

    fn area(s: &str) -> IResult<&str, Area> {
        let p = separated_pair(coord, tag(" through "), coord);
        map(p, |(c1, c2)| Area(c1, c2))(s)
    }

    fn coord(s: &str) -> IResult<&str, Coord2> {
        let p = separated_pair(u32, char(','), u32);
        map(p, |(x, y)| Coord2(x, y))(s)
    }
}
