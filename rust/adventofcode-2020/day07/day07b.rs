use std::io;
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    let mut color_ids: HashMap<String, u32> = HashMap::new();
    let mut graph: HashMap<u32, Vec<(u32, u32)>> = HashMap::new();
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let (parent, children) = parse_line(&buffer, &mut color_ids);
        add_to_graph(&mut graph, parent, &children);
        buffer.clear();
    }
    let target_id = get_col_id(&mut color_ids, "shiny_gold");
    show_graph(&graph, target_id);
    show_col_ids(&color_ids, target_id);
    let max_col = color_ids.len() as u32;
    let packages = packages_contains_target(&graph, max_col, target_id);
    println!("{:?} count: {}", packages, packages.len());
    let count = count_content(&graph, target_id);
    println!("content {}", count);
}

fn parse_line(buffer: &str, col_ids: &mut HashMap<String, u32>) -> (u32, Vec<(u32, u32)>) {
    let parts: Vec<&str> = buffer.trim_end().split(" ").collect();
    let parent_color = get_col_name(parts[0], parts[1]);
    let parent_id = get_col_id(col_ids, &parent_color);
    let mut children: Vec<(u32, u32)> = Vec::new();
    let mut part_pos = 4;
    if parts[part_pos] == "no" {
        return (parent_id, children);
    }
    while part_pos < parts.len() {
        let count: u32 = parts[part_pos].parse().unwrap();
        let child_color = get_col_name(parts[part_pos+1], parts[part_pos+2]);
        let child_id = get_col_id(col_ids, &child_color);
        children.push((child_id, count));
        part_pos += 4;
    }
    (parent_id, children)
}

fn get_col_name(s0: &str, s1: &str) -> String {
    let mut res = String::from(s0);
    res.push('_');
    res.push_str(s1);
    res
}

fn get_col_id(col_ids: &mut HashMap<String, u32>, color: &str) -> u32 {
    if !col_ids.contains_key(color) {
        col_ids.insert(String::from(color), col_ids.len() as u32);
    }
    *col_ids.get(color).unwrap()
}

fn add_to_graph(
    graph: &mut HashMap<u32, Vec<(u32, u32)>>,
    parent_id: u32,
    children: &[(u32, u32)],
) {
    for (c_id, cnt) in children {
        if let Some(child_list) = graph.get_mut(&parent_id) {
            child_list.push((*c_id, *cnt));
        } else {
            graph.insert(parent_id, vec![(*c_id, *cnt)]);
        }
    }
}

fn show_graph(graph: &HashMap<u32, Vec<(u32, u32)>>, tgt_id: u32) {
    println!("~~~mermaid");
    println!("flowchart LR");
    println!("n{}{{Tgt}}", tgt_id);
    for (p_id, children) in graph {
        for (c_id, cnt) in children {
            println!("n{} -->|{}|n{}", p_id, cnt, c_id);
        }
    }
    println!("~~~");
}

fn show_col_ids(col_ids: &HashMap<String, u32>, tgt_id: u32) {
    println!("Colors:");
    let mut col_names = HashMap::new();
    for (key, val) in col_ids {
        col_names.insert(*val, String::from(key));
    }
    let ll = col_ids.len() as u32;
    for col_id in 0..ll {
        let mark = if col_id == tgt_id {
            "(target)"
        } else {
            ""
        };
        println!("- {}: {} {}",
            col_id, 
            col_names.get(&col_id).unwrap(),
            mark,
        );
    }
}

fn packages_contains_target(
    graph: &HashMap<u32, Vec<(u32, u32)>>,
    max_col: u32,
    tgt_id: u32,
) -> Vec<u32> {
    let mut res = Vec::new();
    for p in 0 .. max_col {
        if contains_target(graph, p, tgt_id) {
            res.push(p);
        }
    }
    res
}

fn contains_target(
    graph: &HashMap<u32, Vec<(u32, u32)>>,
    parent: u32,
    tgt_id: u32
) -> bool {
    if let Some(children) = graph.get(&parent) {
        for (c_id, _cnt) in children {
            if *c_id == tgt_id {
                return true;
            }
            if contains_target(graph, *c_id, tgt_id) {
                return true;
            }
        }
    }
    false
}

fn count_content(
    graph: &HashMap<u32, Vec<(u32, u32)>>,
    parent_id: u32,
) -> u32 {
    let mut res = 0;
    if let Some(children) = graph.get(&parent_id) {
        for (c_id, c_cnt) in children {
            res += c_cnt * (1 + count_content(graph, *c_id));
        }
    }
    res
}