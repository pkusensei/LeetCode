mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn gcd_sort(nums: &[i32]) -> bool {
    let max = nums.iter().max().copied().unwrap_or(2);
    let mut dsu = DSU::new(1 + max as usize);
    let mut seen = vec![false; 1 + max as usize];
    for v in nums.iter() {
        let mut num = *v;
        let mut prime: i32 = 2;
        while !seen[num as usize] && prime.pow(2) <= num {
            if num % prime == 0 {
                dsu.union(*v as usize, prime as usize);
                while num % prime == 0 {
                    num /= prime;
                }
            }
            prime += 1;
        }
        if num > 1 {
            dsu.union(*v as usize, num as usize);
        }
        seen[*v as usize] = true;
    }
    let mut sorted = nums.to_vec();
    sorted.sort_unstable();
    nums.iter()
        .zip(sorted)
        .all(|(a, b)| dsu.find(*a as usize) == dsu.find(b as usize))
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v])
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
    }
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
        assert!(gcd_sort(&[7, 21, 3]));
        assert!(!gcd_sort(&[5, 2, 6, 2]));
        assert!(gcd_sort(&[10, 5, 9, 3, 15]));
    }

    #[test]
    fn test() {}
}
