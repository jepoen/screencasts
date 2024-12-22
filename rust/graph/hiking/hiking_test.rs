//! hiking_test.rs

mod graph;

mod place;
use place::{HikingGraph, Place, PlaceType};

fn main() {
    let mut places = Vec::new();
    places.push(Place::new("Wehlen".to_string(), PlaceType::Start));
    places.push(Place::new("Bastei".to_string(), PlaceType::Poi));
    places.push(Place::new("Rathen".to_string(), PlaceType::Start));

    println!("{:?}", places);
    let mut hiking_graph = HikingGraph::new(places);
    for (p1, p2) in vec![
        ("Wehlen", "Bastei"),
        ("Rathen", "Bastei"),
        ("Wehlen", "Uttewalde"),
    ] {
        if !hiking_graph.add_connection(p1, p2) {
            println!("Missing {} or {}", p1, p2);
        }
    }
    println!("## Graph");
    hiking_graph.mermaid();
    println!("## Verbindungen");
    for (p1, p2) in vec![
        ("Wehlen", "Bastei"),
        ("Bastei", "Wehlen"),
        ("Rathen", "Wehlen"),
        ("Wehlen", "Uttewalde"),
    ] {
        println!("- {} – {}: {}", p1, p2, hiking_graph.is_connection(p1, p2));
    }
}
