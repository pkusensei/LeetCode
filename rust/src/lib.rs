mod helper;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn min_mutation(start_gene: &str, end_gene: &str, bank: &[&str]) -> i32 {
    if !bank.contains(&end_gene) {
        return -1;
    }
    if start_gene == end_gene {
        return 0;
    }
    let mut queue = VecDeque::from([(start_gene, 0)]);
    let mut seen = HashSet::new();
    while let Some((curr, dist)) = queue.pop_front() {
        if !seen.insert(curr) {
            continue;
        }
        if curr == end_gene {
            return dist;
        }
        for n in bank.iter().filter(|s| is_neighbor(curr, s)) {
            queue.push_back((n, dist + 1));
        }
    }
    -1
}

fn is_neighbor(a: &str, b: &str) -> bool {
    a.len() == b.len() && a.bytes().zip(b.bytes()).filter(|(x, y)| x != y).count() == 1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_mutation("AACCGGTT", "AACCGGTA", &["AACCGGTA"]), 1);
        debug_assert_eq!(
            min_mutation(
                "AACCGGTT",
                "AAACGGTA",
                &["AACCGGTA", "AACCGCTA", "AAACGGTA"]
            ),
            2
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
