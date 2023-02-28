use std::cell::RefCell;
use std::collections::HashMap;

pub fn part1(data: &str) -> String {
    let instructions = parser::parse(data);
    let wire_connections = WireConnections::new(&instructions);
    let a = wire_connections.value("a");
    a.to_string()
}

pub fn part2(data: &str) -> String {
    let instructions = parser::parse(data);
    let wire_connections = WireConnections::new(&instructions);
    let a = wire_connections.value("a");

    let mut wire_connections_2 = WireConnections::new(&instructions);
    wire_connections_2.override_value("b", a);
    let a2 = wire_connections_2.value("a");
    a2.to_string()
}

struct WireConnections {
    connections: HashMap<WireLabel, Connection>,
}

impl WireConnections {
    fn new(instructions: &Vec<Instruction>) -> Self {
        let mut connections = HashMap::new();

        for ins in instructions.iter() {
            let inserted =
                connections.insert(ins.output().to_owned(), Connection::new(ins.clone()));
            assert!(inserted.is_none());
        }

        Self { connections }
    }

    fn value(&self, wire_label: &str) -> u16 {
        self.connections.get(wire_label).unwrap().value(self)
    }

    fn override_value(&mut self, wire_label: &str, value: u16) -> () {
        self.connections.insert(
            wire_label.to_string(),
            Connection {
                value: RefCell::new(ConnectionValue::Resolved(value)),
            },
        );
    }
}

#[derive(Debug)]
struct Connection {
    value: RefCell<ConnectionValue>,
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Self {
            value: RefCell::new(self.value.borrow().clone()),
        }
    }
}

impl Connection {
    fn value(&self, wire_connections: &WireConnections) -> u16 {
        if !self.value.borrow().is_resolved() {
            self.value.replace_with(|v| v.resolve(wire_connections));
        }

        self.value.borrow().unwrap_value()
    }
}

impl Connection {
    fn new(instruction: Instruction) -> Self {
        Self {
            value: RefCell::new(ConnectionValue::Unresolved(instruction)),
        }
    }
}

#[derive(Debug, Clone)]
enum ConnectionValue {
    Unresolved(Instruction),
    Resolved(u16),
}

impl ConnectionValue {
    fn is_resolved(&self) -> bool {
        match self {
            Self::Resolved(_) => true,
            _ => false,
        }
    }

    fn unwrap_value(&self) -> u16 {
        match self {
            Self::Unresolved(_) => panic!("connection value not resolved"),
            Self::Resolved(v) => *v,
        }
    }

    fn resolve(&self, wire_connections: &WireConnections) -> Self {
        match self {
            Self::Resolved(_) => panic!("already resolved"),
            Self::Unresolved(i) => Self::Resolved(i.value(wire_connections)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    And(Input, Input, WireLabel),
    Or(Input, Input, WireLabel),
    LShift(Input, Input, WireLabel),
    RShift(Input, Input, WireLabel),
    Not(Input, WireLabel),
    Direct(Input, WireLabel),
}

impl Instruction {
    fn output(&self) -> &WireLabel {
        match self {
            Instruction::And(_, _, o) => o,
            Instruction::Or(_, _, o) => o,
            Instruction::LShift(_, _, o) => o,
            Instruction::RShift(_, _, o) => o,
            Instruction::Not(_, o) => o,
            Instruction::Direct(_, o) => o,
        }
    }

    fn value(&self, wire_connections: &WireConnections) -> u16 {
        match self {
            Self::And(i1, i2, _) => i1.value(wire_connections) & i2.value(wire_connections),
            Self::Or(i1, i2, _) => i1.value(wire_connections) | i2.value(wire_connections),
            Self::LShift(i1, i2, _) => i1.value(wire_connections) << i2.value(wire_connections),
            Self::RShift(i1, i2, _) => i1.value(wire_connections) >> i2.value(wire_connections),
            Self::Not(i, _) => !i.value(wire_connections),
            Self::Direct(i, _) => i.value(wire_connections),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Input {
    Resolved(u16),
    Unresolved(WireLabel),
}

impl<'a> Input {
    fn value(&self, wire_connections: &WireConnections) -> u16 {
        match self {
            Self::Resolved(v) => *v,
            Self::Unresolved(wl) => wire_connections.value(wl),
        }
    }
}

type WireLabel = String;

mod parser {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, u16},
        combinator::{map, verify},
        multi::separated_list0,
        sequence::{preceded, separated_pair},
        IResult,
    };

    use super::*;

    pub fn parse(s: &str) -> Vec<Instruction> {
        let (rest, ins) = instructions(s).unwrap();
        assert!(rest.is_empty());
        ins
    }

    fn instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
        let mut p = separated_list0(tag("\n"), instruction);
        p(s)
    }

    fn instruction(s: &str) -> IResult<&str, Instruction> {
        alt((and, or, not, lshift, rshift, direct))(s)
    }

    fn and(s: &str) -> IResult<&str, Instruction> {
        let gate = separated_pair(input, tag(" AND "), input);
        let p = separated_pair(gate, tag(" -> "), io);
        let mut p = map(p, |((i1, i2), o)| Instruction::And(i1, i2, o.to_owned()));
        p(s)
    }

    fn or(s: &str) -> IResult<&str, Instruction> {
        let gate = separated_pair(input, tag(" OR "), input);
        let p = separated_pair(gate, tag(" -> "), io);
        let mut p = map(p, |((i1, i2), o)| Instruction::Or(i1, i2, o.to_owned()));
        p(s)
    }

    fn lshift(s: &str) -> IResult<&str, Instruction> {
        let gate = separated_pair(input, tag(" LSHIFT "), input);
        let p = separated_pair(gate, tag(" -> "), io);
        let mut p = map(p, |((i1, i2), o)| Instruction::LShift(i1, i2, o.to_owned()));
        p(s)
    }

    fn rshift(s: &str) -> IResult<&str, Instruction> {
        let gate = separated_pair(input, tag(" RSHIFT "), input);
        let p = separated_pair(gate, tag(" -> "), io);
        let mut p = map(p, |((i1, i2), o)| Instruction::RShift(i1, i2, o.to_owned()));
        p(s)
    }

    fn not(s: &str) -> IResult<&str, Instruction> {
        let gate = preceded(tag("NOT "), input);
        let p = separated_pair(gate, tag(" -> "), io);
        let mut p = map(p, |(i, o)| Instruction::Not(i, o.to_owned()));
        p(s)
    }

    fn direct(s: &str) -> IResult<&str, Instruction> {
        let gate = input;
        let p = separated_pair(gate, tag(" -> "), io);
        let mut p = map(p, |(i, o)| Instruction::Direct(i, o.to_owned()));
        p(s)
    }

    fn input(s: &str) -> IResult<&str, Input> {
        alt((unresolved, resolved))(s)
    }

    fn unresolved(s: &str) -> IResult<&str, Input> {
        let mut p = map(io, |wire_label: &str| {
            Input::Unresolved(wire_label.to_owned())
        });
        p(s)
    }

    fn resolved(s: &str) -> IResult<&str, Input> {
        let mut p = map(u16, |value: u16| Input::Resolved(value));
        p(s)
    }

    fn io(s: &str) -> IResult<&str, &str> {
        let mut p = verify(alpha1, |wire: &str| wire.chars().all(|c| c.is_lowercase()));
        p(s)
    }
}
