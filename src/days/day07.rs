use std::cell::RefCell;
use std::collections::HashMap;

pub fn part1(data: &str) -> String {
    let instructions = parser::parse(data);
    let wire_connections = WireConnections::new(&instructions);
    wire_connections.value("a").to_string()
}

pub fn part2(data: &str) -> String {
    let instructions = parser::parse(data);
    let wire_connections = WireConnections::new(&instructions);

    let mut wire_connections_2 = WireConnections::new(&instructions);
    wire_connections_2.override_value("b", wire_connections.value("a"));
    wire_connections_2.value("a").to_string()
}

struct WireConnections {
    connections: HashMap<WireLabel, Connection>,
}

impl WireConnections {
    fn new(instructions: &Vec<Instruction>) -> Self {
        let mut connections = HashMap::new();

        for ins in instructions.iter() {
            connections.insert(ins.1.clone(), Connection::new(ins.0.clone()));
        }

        Self { connections }
    }

    fn value(&self, wire_label: &str) -> u16 {
        self.connections.get(wire_label).unwrap().value(self)
    }

    fn override_value(&mut self, wire_label: &str, value: u16) -> () {
        self.connections
            .insert(wire_label.to_string(), Connection::new_with_value(value));
    }
}

type WireLabel = String;

#[derive(Debug)]
struct Connection {
    value: RefCell<ConnectionState>,
}

impl Connection {
    fn new(gate: Gate) -> Self {
        let value = RefCell::new(ConnectionState::Unresolved(gate));
        Self { value }
    }

    fn new_with_value(value: u16) -> Self {
        let value = RefCell::new(ConnectionState::Resolved(value));
        Self { value }
    }

    fn value(&self, wire_connections: &WireConnections) -> u16 {
        if !self.value.borrow().is_resolved() {
            self.value.replace_with(|v| v.resolve(wire_connections));
        }

        self.value.borrow().unwrap_value()
    }
}

#[derive(Debug)]
enum ConnectionState {
    Unresolved(Gate),
    Resolved(u16),
}

impl ConnectionState {
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
            Self::Unresolved(i) => Self::Resolved(i.value(wire_connections)),
            Self::Resolved(_) => panic!("already resolved"),
        }
    }
}

#[derive(Debug)]
pub struct Instruction(Gate, WireLabel);

#[derive(Debug, Clone)]
pub enum Gate {
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, Input),
    RShift(Input, Input),
    Not(Input),
    Direct(Input),
}

impl Gate {
    fn value(&self, wire_connections: &WireConnections) -> u16 {
        match self {
            Self::And(i1, i2) => i1.value(wire_connections) & i2.value(wire_connections),
            Self::Or(i1, i2) => i1.value(wire_connections) | i2.value(wire_connections),
            Self::LShift(i1, i2) => i1.value(wire_connections) << i2.value(wire_connections),
            Self::RShift(i1, i2) => i1.value(wire_connections) >> i2.value(wire_connections),
            Self::Not(i) => !i.value(wire_connections),
            Self::Direct(i) => i.value(wire_connections),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Input {
    Unresolved(WireLabel),
    Resolved(u16),
}

impl<'a> Input {
    fn value(&self, wire_connections: &WireConnections) -> u16 {
        match self {
            Self::Unresolved(wl) => wire_connections.value(wl),
            Self::Resolved(v) => *v,
        }
    }
}

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
        let p = separated_pair(gate, tag(" -> "), wire);
        let mut p = map(p, |((i1, i2), o)| {
            Instruction(Gate::And(i1, i2), o.to_string())
        });
        p(s)
    }

    fn or(s: &str) -> IResult<&str, Instruction> {
        let gate = separated_pair(input, tag(" OR "), input);
        let p = separated_pair(gate, tag(" -> "), wire);
        let mut p = map(p, |((i1, i2), o)| {
            Instruction(Gate::Or(i1, i2), o.to_string())
        });
        p(s)
    }

    fn lshift(s: &str) -> IResult<&str, Instruction> {
        let gate = separated_pair(input, tag(" LSHIFT "), input);
        let p = separated_pair(gate, tag(" -> "), wire);
        let mut p = map(p, |((i1, i2), o)| {
            Instruction(Gate::LShift(i1, i2), o.to_string())
        });
        p(s)
    }

    fn rshift(s: &str) -> IResult<&str, Instruction> {
        let gate = separated_pair(input, tag(" RSHIFT "), input);
        let p = separated_pair(gate, tag(" -> "), wire);
        let mut p = map(p, |((i1, i2), o)| {
            Instruction(Gate::RShift(i1, i2), o.to_string())
        });
        p(s)
    }

    fn not(s: &str) -> IResult<&str, Instruction> {
        let gate = preceded(tag("NOT "), input);
        let p = separated_pair(gate, tag(" -> "), wire);
        let mut p = map(p, |(i, o)| Instruction(Gate::Not(i), o.to_string()));
        p(s)
    }

    fn direct(s: &str) -> IResult<&str, Instruction> {
        let gate = input;
        let p = separated_pair(gate, tag(" -> "), wire);
        let mut p = map(p, |(i, o)| Instruction(Gate::Direct(i), o.to_string()));
        p(s)
    }

    fn input(s: &str) -> IResult<&str, Input> {
        alt((unresolved, resolved))(s)
    }

    fn unresolved(s: &str) -> IResult<&str, Input> {
        let mut p = map(wire, |wire_label: &str| {
            Input::Unresolved(wire_label.to_owned())
        });
        p(s)
    }

    fn resolved(s: &str) -> IResult<&str, Input> {
        let mut p = map(u16, |value: u16| Input::Resolved(value));
        p(s)
    }

    fn wire(s: &str) -> IResult<&str, &str> {
        let mut p = verify(alpha1, |wire: &str| wire.chars().all(|c| c.is_lowercase()));
        p(s)
    }
}
