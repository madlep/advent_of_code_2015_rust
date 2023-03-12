use array2d::Array2D;
use core::hash::Hash;
use std::collections::HashMap;

pub fn part1(data: &str) -> String {
    let costs = build_costs(parser::parse(data));

    (0..costs.row_len())
        .fold(u32::MAX, |cheapest_found, from_id| {
            min_visit_cost(
                from_id,
                costs.column_len(),
                &vec![false; costs.column_len()],
                0,
                &costs,
                cheapest_found,
            )
        })
        .to_string()
}

pub fn part2(data: &str) -> String {
    let costs = build_costs(parser::parse(data));

    (0..costs.row_len())
        .fold(u32::MIN, |costliest_found, from_id| {
            max_visit_cost(
                from_id,
                costs.column_len(),
                &vec![false; costs.column_len()],
                0,
                &costs,
                costliest_found,
            )
        })
        .to_string()
}

fn build_costs(connections: Vec<Connection>) -> Array2D<Cost> {
    let mut registry: Registry<Location> = Registry::new();
    for c in connections.iter() {
        registry.add(c.from.to_owned());
        registry.add(c.to.to_owned());
    }

    let location_count = registry.len();
    let mut costs: Array2D<Cost> = Array2D::filled_with(0, location_count, location_count);

    for c in connections.iter() {
        let from_id = registry.get(&c.from);
        let to_id = registry.get(&c.to);
        // connections are bi-directional
        costs[(from_id, to_id)] = c.cost;
        costs[(to_id, from_id)] = c.cost;
    }
    costs
}

fn min_visit_cost(
    from_id: usize,
    location_count: usize,
    visited: &Vec<bool>,
    current_cost: Cost,
    costs: &Array2D<Cost>,
    cheapest_found: Cost,
) -> u32 {
    let mut cheapest_found = cheapest_found;
    let mut visited = visited.clone();
    visited[from_id] = true;

    // terminal case - we've visited everything
    if visited.iter().all(|v| *v) {
        return current_cost;
    }

    for to_id in 0..location_count {
        // continue if from/to are the same location, or we've already visited
        if to_id == from_id || visited[to_id] {
            continue;
        }

        let connection_cost = costs[(from_id, to_id)];
        // cost 0 means no connection
        assert!(connection_cost != 0);

        let to_cost = current_cost + connection_cost;
        if to_cost < cheapest_found {
            cheapest_found = min_visit_cost(
                to_id,
                location_count,
                &visited,
                to_cost,
                costs,
                cheapest_found,
            )
            .min(cheapest_found);
        }
    }
    // if we haven't visited
    cheapest_found
}

fn max_visit_cost(
    from_id: usize,
    location_count: usize,
    visited: &Vec<bool>,
    current_cost: Cost,
    costs: &Array2D<Cost>,
    costliest_found: Cost,
) -> u32 {
    let mut costliest_found = costliest_found;
    let mut visited = visited.clone();
    visited[from_id] = true;

    // terminal case - we've visited everything
    if visited.iter().all(|v| *v) {
        return current_cost;
    }

    for to_id in 0..location_count {
        // continue if from/to are the same location, or we've already visited
        if to_id == from_id || visited[to_id] {
            continue;
        }

        let connection_cost = costs[(from_id, to_id)];
        // cost 0 means no connection
        assert!(connection_cost != 0);

        let to_cost = current_cost + connection_cost;
        costliest_found = max_visit_cost(
            to_id,
            location_count,
            &visited,
            to_cost,
            costs,
            costliest_found,
        )
        .max(costliest_found);
    }
    // if we haven't visited
    costliest_found
}

#[derive(Debug)]
pub struct Connection {
    from: Location,
    to: Location,
    cost: Cost,
}

type Location = String;
type Cost = u32;

#[derive(Debug)]
struct Registry<T> {
    values: HashMap<T, usize>,
    next_id: usize,
}

impl<T: Eq + Hash> Registry<T> {
    fn new() -> Self {
        Registry {
            values: HashMap::new(),
            next_id: 0,
        }
    }

    fn add(&mut self, key: T) -> () {
        if !self.values.contains_key(&key) {
            self.values.insert(key, self.next_id);
            self.next_id += 1;
        }
    }

    fn get(&self, key: &T) -> usize {
        *(self.values.get(key).unwrap())
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, u8},
        combinator::map,
        multi::separated_list0,
        sequence::separated_pair,
        IResult,
    };

    use super::*;

    pub fn parse(s: &str) -> Vec<Connection> {
        let (rest, conns) = connections(s).unwrap();
        assert!(rest.is_empty());
        conns
    }

    fn connections(s: &str) -> IResult<&str, Vec<Connection>> {
        separated_list0(tag("\n"), connection)(s)
    }

    fn connection(s: &str) -> IResult<&str, Connection> {
        map(
            separated_pair(from_to, tag(" = "), u8),
            |((from, to), cost)| Connection {
                from,
                to,
                cost: cost as u32,
            },
        )(s)
    }

    fn from_to(s: &str) -> IResult<&str, (Location, Location)> {
        map(
            separated_pair(location, tag(" to "), location),
            |(from, to)| (from.to_string(), to.to_string()),
        )(s)
    }

    fn location(s: &str) -> IResult<&str, &str> {
        alpha1(s)
    }
}
