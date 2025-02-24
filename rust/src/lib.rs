mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_two_events(events: &mut [[i32; 3]]) -> i32 {
    events.sort_unstable_by_key(|e| e[0]);
    let mut temp = 0;
    let mut suf_max = vec![];
    for e in events.iter().rev() {
        temp = temp.max(e[2]);
        suf_max.push(temp);
    }
    suf_max.reverse();
    let mut res = 0;
    for e in events.iter() {
        let [_start, end, val] = e[..] else {
            unreachable!()
        };
        let i = events.partition_point(|e| e[0] <= end);
        res = res.max(val + suf_max.get(i).unwrap_or(&0));
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
