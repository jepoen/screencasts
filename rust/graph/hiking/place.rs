//! mod place
#![allow(dead_code)]

use graph::Graph;
use std::fmt;

#[derive(Debug)]
pub enum PlaceType {
    Start, // Start point
    Poi,   // Point of Interest
}

#[derive(Debug)]
pub struct Place {
    name: String,
    kind: PlaceType,
}

impl Place {
    pub fn new(name: String, kind: PlaceType) -> Self {
        Place { name, kind }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_start(&self) -> bool {
        match self.kind {
            PlaceType::Start => true,
            _ => false,
        }
    }
    pub fn is_poi(&self) -> bool {
        match self.kind {
            PlaceType::Poi => true,
            _ => false,
        }
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            PlaceType::Start => write!(f, "{}(S)", self.name),
            PlaceType::Poi => write!(f, "{}", self.name),
        }
    }
}

pub struct HikingGraph {
    places: Vec<Place>,
    graph: Graph,
}

impl HikingGraph {
    pub fn new(places: Vec<Place>) -> Self {
        let node_count = places.len();
        HikingGraph {
            places,
            graph: Graph::new(node_count),
        }
    }

    pub fn places(&self) -> &[Place] {
        &self.places
    }

    fn place_idx(&self, place_name: &str) -> Option<usize> {
        for idx in 0..self.places.len() {
            if self.places[idx].name == place_name {
                return Some(idx);
            }
        }
        None
    }

    pub fn add_connection(&mut self, p1: &str, p2: &str) -> bool {
        let o1 = self.place_idx(p1);
        let o2 = self.place_idx(p2);
        if o1.is_some() && o2.is_some() {
            self.graph.add_edge(o1.unwrap(), o2.unwrap());
            true
        } else {
            false
        }
    }

    pub fn is_connection(&self, p1: &str, p2: &str) -> bool {
        let o1 = self.place_idx(p1);
        let o2 = self.place_idx(p2);
        if o1.is_some() && o2.is_some() {
            self.graph.is_edge(o1.unwrap(), o2.unwrap())
        } else {
            false
        }
    }

    pub fn mermaid(&self) {
        println!("~~~mermaid");
        println!("flowchart LR");
        for idx in 0..self.places.len() {
            let p = &self.places[idx];
            match p.kind {
                PlaceType::Start => println!("v_{}[[{}]]", idx, p.name),
                PlaceType::Poi => println!("v_{}({})", idx, p.name),
            }
        }
        let n = self.graph.node_count();
        for r in 0..n {
            for c in r + 1..n {
                if self.graph.is_edge(r, c) {
                    println!("v_{} <--> v_{}", r, c);
                }
            }
        }
        println!("~~~");
    }

    pub fn neighbors(&self, name: &str) -> Vec<&Place> {
        let mut res = Vec::new();
        if let Some(from_idx) = self.place_idx(name) {
            for to_idx in 0..self.graph.node_count() {
                if self.graph.is_edge(from_idx, to_idx) {
                    res.push(&self.places[to_idx]);
                }
            }
        }
        res
    }

    pub fn connection_hops(&self, name1: &str, name2: &str) -> Vec<&Place> {
        let o1 = self.place_idx(name1);
        let o2 = self.place_idx(name2);
        let mut res = Vec::new();
        if o1.is_some() && o2.is_some() {
            let idx1 = o1.unwrap();
            let idx2 = o2.unwrap();
            for hop in 0..self.graph.node_count() {
                if self.graph.is_edge(idx1, hop) && self.graph.is_edge(hop, idx2) {
                    res.push(&self.places[hop]);
                }
            }
        }
        res
    }
}
