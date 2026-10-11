mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

#[derive(Clone, Copy, Default)]
struct Seat {
    window: i32,
    middle: i32,
}

// AB CD
struct BusBooking {
    inner: Vec<[Seat; 2]>,
    total: i32,
}

impl BusBooking {
    fn new(n: i32) -> Self {
        Self {
            inner: vec![[Seat::default(); 2]; 1 + n as usize],
            total: 0,
        }
    }

    fn toggle(&mut self, seat: String) {
        let row: usize = seat[1..].parse().unwrap();
        let s = seat.as_bytes();
        let is_window = matches!(s[0], b'A' | b'D');
        let slot = usize::from(s[0] > b'B'); // AB -> 0; CD -> 1
        let seat = &mut self.inner[row][slot];
        if is_window {
            if seat.window > 0 {
                self.total -= seat.window;
                seat.window = 0;
            } else {
                let val = if seat.middle > 0 { 3 } else { 1 };
                seat.window = val;
                self.total += val;
            }
        } else {
            if seat.middle > 0 {
                self.total -= seat.middle;
                seat.middle = 0;
            } else {
                seat.middle = 1;
                self.total += 1;
            }
        }
    }

    fn get_total_time(&self) -> i32 {
        self.total
    }

    fn get_min_time(&self) -> i32 {
        self.inner
            .iter()
            .flat_map(|v| v.iter().map(|s| s.middle + i32::from(s.window > 0)))
            .sum()
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
