// mod energy_mix

pub struct EnergyMix {
    kinds: Vec<String>,
    years: Vec<u32>,
    data: Vec<Vec<f64>>,
}

impl EnergyMix {
    pub fn new(kinds: Vec<String>, years: Vec<u32>, data: Vec<Vec<f64>>) -> Self {
        EnergyMix { kinds, years, data }
    }
    pub fn show(&self) {
        // head
        println!("~~~");
        print!("{:20}", "Art");
        for year in &self.years {
            print!("{:8}", year);
        }
        println!();
        // Table content
        for row_idx in 0..self.kinds.len() {
            print!("{:20}", self.kinds[row_idx]);
            for col_idx in 0..self.years.len() {
                print!("{:8.1}", self.data[row_idx][col_idx]);
            }
            println!();
        }
        println!("~~~");
    }

    pub fn mermaid(&self, min_energy: f64) {
        println!("~~~mermaid");
        println!("flowchart LR");
        println!("subgraph kinds");
        let max_kinds = self.kinds.len();
        for idx_kind in 0..max_kinds {
            println!("  {}", self.kinds[idx_kind]);
        }
        println!("end");
        println!("subgraph years");
        for year in &self.years {
            println!("  v{}({})", year, year);
        }
        println!("end");
        // Edges
        for idx_kind in 0..max_kinds {
            for idx_year in 0..self.years.len() {
                if self.data[idx_kind][idx_year] > min_energy {
                    println!(
                        "{} -->|{}| v{}",
                        self.kinds[idx_kind], self.data[idx_kind][idx_year], self.years[idx_year],
                    )
                }
            }
        }
        println!("~~~");
    }
    fn kind_idx(&self, kind: &str) -> Option<usize> {
        for idx in 0..self.kinds.len() {
            if self.kinds[idx] == kind {
                return Some(idx);
            }
        }
        None
    }

    fn year_idx(&self, year: u32) -> Option<usize> {
        for idx in 0..self.years.len() {
            if self.years[idx] == year {
                return Some(idx);
            }
        }
        None
    }

    pub fn change_relative(&self, kind: &str, start: u32, end: u32) -> Option<f64> {
        let idx_row = self.kind_idx(kind);
        let idx_col1 = self.year_idx(start);
        let idx_col2 = self.year_idx(end);
        if idx_row.is_some() && idx_col1.is_some() && idx_col2.is_some() {
            let row = idx_row.unwrap();
            let col1 = idx_col1.unwrap();
            let col2 = idx_col2.unwrap();
            let val_start = self.data[row][col1];
            let val_end = self.data[row][col2];
            if val_start > 0.0 {
                Some((val_end - val_start) / val_start)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn col_sum(&self, col: usize) -> f64 {
        let mut sum = 0.0;
        for row in 0..self.data.len() {
            sum += self.data[row][col];
        }
        sum
    }

    pub fn ratio(&self, kind: &str, year: u32) -> Option<f64> {
        let idx_row = self.kind_idx(kind);
        let idx_col = self.year_idx(year);
        if idx_row.is_some() && idx_col.is_some() {
            let row0 = idx_row.unwrap();
            let col = idx_col.unwrap();
            // add all kinds for year
            Some(self.data[row0][col] / self.col_sum(col))
        } else {
            None
        }
    }

    pub fn total_energy(&self, year: u32) -> Option<f64> {
        let idx_col = self.year_idx(year);
        if let Some(col) = idx_col {
            Some(self.col_sum(col))
        } else {
            None
        }
    }
}
