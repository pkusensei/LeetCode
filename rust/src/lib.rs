mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn exclusive_time(n: i32, logs: &[&str]) -> Vec<i32> {
    let mut states: Vec<_> = logs.iter().map(|s| parse(s).unwrap()).collect();
    states.sort_unstable_by_key(|s| s.1);
    let n = n as usize;
    let mut stack: Vec<(usize, i32, bool)> = Vec::with_capacity(n / 2);
    let mut res = vec![0; n];
    for state in states {
        if state.2 {
            if let Some(prev) = stack.last_mut() {
                res[prev.0] += state.1 - prev.1; // pause prev
            }
            stack.push(state);
        } else {
            let Some((i, s, _)) = stack.pop() else {
                break;
            };
            debug_assert_eq!(i, state.0);
            res[i] += state.1 - s + 1;
            if let Some(prev) = stack.last_mut() {
                prev.1 = state.1 + 1; // resume
            }
        }
    }
    res
}

fn parse(s: &str) -> Option<(usize, i32, bool)> {
    let mut it = s.split(':');
    let id = it.next().map(|s| s.parse().ok())??; // ???
    let start = it.next() == Some("start");
    let time = it.next().map(|s| s.parse().ok())??;
    Some((id, time, start))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[rustfmt::skip]
    #[test]
    fn basics() {
        debug_assert_eq!(
            exclusive_time(2, &["0:start:0", "1:start:2", "1:end:5", "0:end:6"]),
            [3, 4]
        );

        debug_assert_eq!(
            exclusive_time(1,
                &["0:start:0","0:start:2","0:end:5","0:start:6","0:end:6","0:end:7"]),
            [8]
        );
        debug_assert_eq!(
            exclusive_time(2, 
                &["0:start:0","0:start:2","0:end:5","1:start:6","1:end:6","0:end:7"]),
            [7,1]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
