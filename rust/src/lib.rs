mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
    let mut set = std::collections::HashSet::new();
    for c in circles.iter() {
        let [cx, cy, r] = c[..] else { unreachable!() };
        for x in cx - r..=cx + r {
            for y in cy - r..=cy + r {
                if (x - cx).pow(2) + (y - cy).pow(2) <= r.pow(2) {
                    set.insert([x, y]);
                }
            }
        }
    }
    set.len() as i32
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
