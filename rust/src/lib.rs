mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn num_buses_to_destination(routes: &[&[i32]], source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }
    let buses: HashMap<i32, Vec<usize>> =
        routes
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, v)| {
                for &num in v.iter() {
                    acc.entry(num).or_default().push(i);
                }
                acc
            });
    let Some(mut queue) = buses
        .get(&source)
        .map(|v| v.iter().map(|&i| (i, 1)).collect::<VecDeque<_>>())
    else {
        return -1;
    };
    let mut seen = HashSet::new();
    while let Some((bus, dist)) = queue.pop_front() {
        if !seen.insert(bus) {
            continue;
        }
        for &num in routes[bus].iter() {
            if num == target {
                return dist;
            }
            let Some(next) = buses.get(&num) else {
                continue;
            };
            for &v in next.iter().filter(|&&v| v != bus) {
                queue.push_back((v, 1 + dist));
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_buses_to_destination(&[&[1, 2, 7], &[3, 6, 7]], 1, 6), 2);
        debug_assert_eq!(
            num_buses_to_destination(
                &[&[7, 12], &[4, 5, 15], &[6], &[15, 19], &[9, 12, 13]],
                15,
                12
            ),
            -1
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
