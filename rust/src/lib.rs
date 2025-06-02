mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_length(nums: &[i32]) -> i32 {
    const MAX: i64 = 3_628_800;
    let mut res = 1;
    'outer: for (left, &num) in nums.iter().enumerate() {
        let [mut prod, mut gcd_, mut lcm_] = [i64::from(num); 3];
        let mut count = 1;
        for &num in &nums[1 + left..] {
            prod *= i64::from(num);
            if prod > MAX {
                continue 'outer;
            }
            gcd_ = gcd(gcd_, num.into());
            lcm_ = lcm(lcm_, num.into());
            if prod == gcd_ * lcm_ {
                count += 1;
                res = res.max(count);
            }
        }
    }
    res
}

const fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
}

const fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
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
