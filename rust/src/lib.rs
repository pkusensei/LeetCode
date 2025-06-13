mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

struct Spreadsheet {
    grid: Vec<[i32; 26]>,
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            grid: vec![[0; 26]; 1 + rows as usize],
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        let [row, col] = get_dims(&cell);
        self.grid[row][col] = value;
    }

    fn reset_cell(&mut self, cell: String) {
        let [row, col] = get_dims(&cell);
        self.grid[row][col] = 0;
    }

    fn get_value(&self, formula: String) -> i32 {
        let Some((a, b)) = formula.split_once('+') else {
            return 0;
        };
        [&a[1..], b]
            .map(|s| {
                s.parse::<i32>().unwrap_or_else(|_| {
                    let [row, col] = get_dims(s);
                    self.grid[row][col]
                })
            })
            .iter()
            .sum()
    }
}

fn get_dims(cell: &str) -> [usize; 2] {
    let col = usize::from(cell.as_bytes()[0] - b'A');
    let row = cell[1..].parse::<usize>().unwrap_or(0);
    [row, col]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
