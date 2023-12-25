use anyhow::Result;

use crate::common;
use std::borrow::ToOwned;
use std::collections::HashMap;

pub fn main() -> Result<(usize, usize)> {
    let lines = common::read_lines("inputs/25.txt")?;

    let mut wires: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let line = line?;
        let line = line.trim();

        let (name, connections) = line.split_once(": ").unwrap();
        let connections = connections
            .split(' ')
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>();
        wires
            .entry(name.to_owned())
            .or_default()
            .extend(connections.clone());
        for connection in connections {
            wires.entry(connection).or_default().push(name.to_owned());
        }
    }

    let solution_a;
    'outer_loop: loop {
        let contracted = karger_stein(wires.clone());
        for (wire, edges) in contracted {
            if edges.len() == 3 {
                let a = wire.split(',').count();
                solution_a = a * (wires.len() - a);
                break 'outer_loop;
            }
        }
    }

    Ok((solution_a, 0))
}

fn contract(mut wires: HashMap<String, Vec<String>>, t: usize) -> HashMap<String, Vec<String>> {
    while wires.len() > t {
        let (wire, edges) = wires
            .iter()
            .nth(rand::random::<usize>() % wires.len())
            .unwrap();
        let wire = wire.clone();
        let edge_with = edges[rand::random::<usize>() % edges.len()].clone();
        wires = contract_edge(wires, &wire, &edge_with);
    }
    wires
}

fn contract_edge(
    mut wires: HashMap<String, Vec<String>>,
    wire: &str,
    edge_with: &str,
) -> HashMap<String, Vec<String>> {
    let new_name = format!("({wire}, {edge_with})");
    if let Some(old_edges_wire) = wires.remove(wire) {
        for edge in old_edges_wire {
            if edge != edge_with {
                let set = wires.entry(edge.clone()).or_default();
                *set.iter_mut().find(|w| *w == wire).unwrap() = new_name.clone();
                wires.entry(new_name.clone()).or_default().push(edge);
            }
        }
    }
    if let Some(old_edges_edge_with) = wires.remove(edge_with) {
        for edge in old_edges_edge_with {
            if edge != wire {
                let set = wires.entry(edge.clone()).or_default();
                *set.iter_mut().find(|w| *w == edge_with).unwrap() = new_name.clone();
                wires.entry(new_name.clone()).or_default().push(edge);
            }
        }
    }
    wires
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
fn karger_stein(wires: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    if wires.len() <= 6 {
        contract(wires, 2)
    } else {
        let t = (1f32 + wires.len() as f32 / 2f32.sqrt()).ceil() as usize;
        let g1 = contract(wires.clone(), t);
        let g2 = contract(wires, t);
        if g1.values().map(Vec::len).sum::<usize>() > g2.values().map(Vec::len).sum::<usize>() {
            karger_stein(g2)
        } else {
            karger_stein(g1)
        }
    }
}
