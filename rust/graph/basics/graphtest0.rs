//! graphtest0.rs
mod graph;

use graph::Graph;

fn main() {
    let mut g = Graph::new(6);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 2);
    g.add_edge(1, 3);
    g.add_edge(4, 5);
    println!("## Adjacency matrix");
    show_adj(&g);
    println!("## Graph");
    show_mermaid(&g);
    println!("## Statics");
    println!("- Node count: {}", g.node_count());
    println!("- Edge count: {}", g.edge_count());
    println!("- Number of neighbors:");
    for v in 0..g.node_count() {
        println!("  - Node {}: {}", v, g.neighbor_count(v));
    }
    println!("- Neighbors:");
    println!("~~~");
    for v in 0..g.node_count() {
        println!("    Node {}: {:?}", v, g.neighbors(v));
    }
    println!("~~~");
}

fn show_adj(g: &Graph) {
    let n = g.node_count();
    //let adj = g.adj();
    println!("~~~");
    print!("  |");
    for to in 0..n {
        print!(" {:2}", to);
    }
    println!();
    print!("--|");
    for _ in 0..n {
        print!("---");
    }
    println!();
    for from in 0..n {
        print!("{:2}|", from);
        for to in 0..n {
            let c = if g.is_edge(from, to) {
                //adj[from][to] {
                'x'
            } else {
                '-'
            };
            print!(" {:>2}", c);
        }
        println!();
    }
    println!("~~~");
}

fn show_mermaid(g: &Graph) {
    let n = g.node_count();
    println!("~~~mermaid");
    println!("flowchart TB");
    for i in 0..n {
        println!("v{}(({}))", i, i);
    }
    for from in 0..n {
        for to in from + 1..n {
            if g.is_edge(from, to) {
                println!("v{} <--> v{}", from, to);
            }
        }
    }
    println!("~~~");
}
