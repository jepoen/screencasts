//! hiking1.rs
//! Einlesen der Daten

use std::io;

mod graph;

mod place;
use place::{HikingGraph, Place, PlaceType};

fn main() {
    let (hiking_graph, queries) = read_input();
    println!("## Orte");
    for p in hiking_graph.places() {
        println!("- {}", p);
    }
    println!("## Graph");
    hiking_graph.mermaid();
    println!("## Verbindungen");
    for (p1, p2) in &queries {
        println!("- {} – {}: {}", p1, p2, hiking_graph.is_connection(p1, p2));
    }
}

fn read_input() -> (HikingGraph, Vec<(String, String)>) {
    enum State {
        Start,
        Places,
        Connections,
        Queries,
    }
    let mut places = Vec::new();
    let mut hiking_graph = HikingGraph::new(Vec::new());
    let mut queries = Vec::new();
    let mut state = State::Start;
    for line in io::stdin().lines() {
        let line = line.unwrap();
        let trimmed_line = line.trim();
        if trimmed_line == "" {
            continue;
        }
        match state {
            State::Start => {
                if trimmed_line == "#Places" {
                    state = State::Places;
                }
            }
            State::Places => {
                if trimmed_line == "#Connections" {
                    state = State::Connections;
                    hiking_graph = HikingGraph::new(places);
                    places = Vec::new(); // dummy
                } else {
                    let place = parse_place(&trimmed_line);
                    places.push(place);
                }
            }
            State::Connections => {
                if trimmed_line == "#Queries" {
                    state = State::Queries;
                } else {
                    let (name1, name2) = parse_place_pair(&trimmed_line);
                    if !hiking_graph.add_connection(name1, name2) {
                        println!("{} or {} unknown", name1, name2);
                    }
                }
            }
            State::Queries => {
                let (name1, name2) = parse_place_pair(&trimmed_line);
                queries.push((name1.to_string(), name2.to_string()));
            }
        }
    }
    (hiking_graph, queries)
}

fn parse_place(line: &str) -> Place {
    let parts: Vec<_> = line.split(":").collect();
    let name = parts[0];
    let kind = if parts.len() > 1 && parts[1] == "S" {
        PlaceType::Start
    } else {
        PlaceType::Poi
    };
    Place::new(name.to_string(), kind)
}

fn parse_place_pair(line: &str) -> (&str, &str) {
    let parts: Vec<_> = line.split("-").collect();
    (parts[0], parts[1])
}
