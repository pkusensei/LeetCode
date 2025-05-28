mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shift_distance(s: &str, t: &str, next_cost: &[i32], previous_cost: &[i32]) -> i64 {
    const N: usize = 26;
    let mut nexts = [0; N];
    let mut prevs = [0; N];
    nexts[0] = i64::from(next_cost[N - 1]);
    prevs[N - 1] = i64::from(previous_cost[0]);
    for i in 1..N {
        nexts[i] = nexts[i - 1] + i64::from(next_cost[i - 1]);
        prevs[N - 1 - i] = prevs[N - i] + i64::from(previous_cost[N - i]);
    }
    let mut res = 0;
    for (b1, b2) in s.bytes().zip(t.bytes()) {
        let [i1, i2] = [b1, b2].map(|b| usize::from(b - b'a'));
        res += if b1 < b2 {
            (nexts[i2] - nexts[i1]).min(prevs[0] + prevs[i2] - prevs[i1])
        } else {
            (prevs[i2] - prevs[i1]).min(nexts[N - 1] + nexts[i2] - nexts[i1])
        };
    }
    res
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
