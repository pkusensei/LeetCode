mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let mut line = [0; 102];
    for v in nums.iter() {
        let [a, b] = v[..] else { unreachable!() };
        line[a as usize] += 1;
        line[1 + b as usize] -= 1;
    }
    for i in 1..102 {
        line[i] += line[i - 1];
    }
    line.iter().filter(|&&v| v > 0).count() as i32
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
