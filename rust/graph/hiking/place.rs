//! mod place

use graph::Graph;

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

    pub fn is_connection(&mut self, p1: &str, p2: &str) -> bool {
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
}
