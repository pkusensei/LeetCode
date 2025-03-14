mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_house_placements(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = n as usize;
    let mut prev = [1, 1]; // init i==0
    for _ in 1..n {
        let mut curr = [0, 0];
        curr[0] = (prev[0] + prev[1]) % MOD; // curr[0] is empty
        curr[1] = prev[0]; // curr[1] is set; take empty prev
        prev = curr;
    }
    ((prev[0] + prev[1]).pow(2) % MOD) as i32
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
    fn basics() {
        assert_eq!(count_house_placements(1), 4);
        assert_eq!(count_house_placements(2), 9);
    }

    #[test]
    fn test() {}
}
