//! Modul graph
#![allow(dead_code)]

pub struct Graph {
    adj: Vec<Vec<bool>>,
}

impl Graph {
    pub fn new(n_nodes: usize) -> Self {
        Graph {
            adj: vec![vec![false; n_nodes]; n_nodes],
        }
    }

    // Getter
    pub fn adj(&self) -> &[Vec<bool>] {
        &self.adj
    }

    pub fn node_count(&self) -> usize {
        self.adj.len()
    }

    pub fn is_edge(&self, from: usize, to: usize) -> bool {
        self.adj[from][to]
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        let n = self.node_count();
        if from < n && to < n {
            self.adj[from][to] = true;
            self.adj[to][from] = true;
        }
    }

    pub fn edge_count(&self) -> usize {
        let n = self.node_count();
        let mut count = 0;
        for from in 0..n {
            for to in from + 1..n {
                if self.adj[from][to] {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn neighbor_count(&self, v: usize) -> usize {
        let n = self.node_count();
        if v >= n {
            return 0;
        }
        let mut count = 0;
        for to in 0..n {
            if self.adj[v][to] {
                count += 1;
            }
        }
        count
    }

    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        let mut res = Vec::new();
        let n = self.node_count();
        if v >= n {
            return res;
        }
        for to in 0..n {
            if self.adj[v][to] {
                res.push(to);
            }
        }
        res
    }
}
