mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_minimum_time(mut tasks: Vec<[i32; 3]>) -> i32 {
    tasks.sort_unstable_by_key(|t| t[1]);
    let max = tasks.last().unwrap()[1];
    let mut slots = vec![false; 1 + max as usize];
    for t in tasks.iter() {
        let [tstart, tend, mut tdur] = [t[0], t[1], t[2]];
        let count = slots[tstart as usize..=tend as usize]
            .iter()
            .filter(|&&v| v)
            .count() as i32;
        tdur -= tdur.min(count);
        let mut idx = tend;
        while tdur > 0 && idx >= tstart {
            if !slots[idx as usize] {
                slots[idx as usize] = true;
                tdur -= 1;
            }
            idx -= 1;
        }
    }
    slots.into_iter().map(i32::from).sum()
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
        assert_eq!(find_minimum_time(vec![[2, 3, 1], [4, 5, 1], [1, 5, 2]]), 2);
        assert_eq!(find_minimum_time(vec![[1, 3, 2], [2, 5, 3], [5, 6, 2]]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(
            find_minimum_time(vec![[10, 16, 3], [10, 20, 5], [1, 12, 4], [8, 11, 2]]),
            6
        );
        assert_eq!(
            find_minimum_time(vec![[1, 10, 7], [4, 11, 1], [3, 19, 7], [10, 15, 2]]),
            8
        );
    }
}
