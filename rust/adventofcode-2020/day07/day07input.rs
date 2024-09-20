use std::io;
use std::collections::HashMap;

fn main() {
    let mut buffer = String::new();
    let mut color_ids: HashMap<String, u32> = HashMap::new();
    let target_id = get_col_id(&mut color_ids, "shiny_gold");
    println!("~~~mermaid");
    println!("flowchart LR");
    println!("n{}{{TGT}}", target_id);
    while io::stdin().read_line(&mut buffer).unwrap() > 0 {
        let (parent, children) = parse_line(&buffer, &mut color_ids);
        for c in children {
            println!("n{} -->|{}|n{}", parent, c.1, c.0);
        }
        buffer.clear();
    }
    println!("~~~");
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