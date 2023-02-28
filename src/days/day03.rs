use std::collections::HashSet;

pub fn part1(data: &str) -> String {
    let mut houses = Houses::new();
    houses.visit(&Coord::origin());

    parser::parse(data)
        .iter()
        .fold(Coord::origin(), |current_coord, dir| {
            let new_coord = current_coord.translate(dir);
            houses.visit(&new_coord);
            new_coord
        });

    houses.visited_count().to_string()
}

pub fn part2(data: &str) -> String {
    let mut houses = Houses::new();
    houses.visit(&Coord::origin());

    let workers = 2;

    parser::parse(data)
        .chunks(workers)
        .fold([Coord::origin()].repeat(workers), |coords, dirs| {
            let new_coords: Vec<Coord> = coords
                .iter()
                .zip(dirs.iter())
                .map(|(coord, dir)| coord.translate(dir))
                .collect();

            new_coords.iter().for_each(|coord| houses.visit(&coord));

            new_coords
        });

    houses.visited_count().to_string()
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn translate(self, dir: &Dir) -> Self {
        match dir {
            Dir::North => Self {
                y: self.y - 1,
                ..self
            },
            Dir::South => Self {
                y: self.y + 1,
                ..self
            },
            Dir::East => Self {
                x: self.x + 1,
                ..self
            },
            Dir::West => Self {
                x: self.x - 1,
                ..self
            },
        }
    }
}

struct Houses {
    visited: HashSet<Coord>,
}

impl Houses {
    fn new() -> Self {
        Self {
            visited: HashSet::new(),
        }
    }

    fn visit(&mut self, coord: &Coord) -> () {
        self.visited.insert(*coord);
    }

    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

#[derive(Clone)]
pub enum Dir {
    North,
    South,
    East,
    West,
}

mod parser {
    use super::Dir;
    use nom::{branch::alt, character::complete::char, combinator::value, multi::many1, IResult};

    pub fn parse(s: &str) -> Vec<Dir> {
        let (_rest, ds) = dirs(s).unwrap();
        ds
    }

    fn dirs(s: &str) -> IResult<&str, Vec<Dir>> {
        many1(dir)(s)
    }

    fn dir(s: &str) -> IResult<&str, Dir> {
        alt((north, south, west, east))(s)
    }

    fn north(s: &str) -> IResult<&str, Dir> {
        value(Dir::North, char('^'))(s)
    }

    fn south(s: &str) -> IResult<&str, Dir> {
        value(Dir::South, char('v'))(s)
    }

    fn east(s: &str) -> IResult<&str, Dir> {
        value(Dir::East, char('>'))(s)
    }

    fn west(s: &str) -> IResult<&str, Dir> {
        value(Dir::West, char('<'))(s)
    }
}
