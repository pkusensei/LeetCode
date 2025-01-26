mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
    let n = favorite.len();
    let innodes = favorite
        .iter()
        .enumerate()
        .fold(vec![vec![]; n], |mut acc, (i, &v)| {
            acc[v as usize].push(i);
            acc
        });
    let mut seen = vec![false; n];
    // Cycle a->b->c->d->e->a
    // Chains a->b->c<->f<-g<-h
    // Multiple 2-chain groups can be all fit in
    // a->b->c<->d<-e + f->g<->h<-i<-j + ...
    let [mut cycle_len, mut chain_len] = [0, 0];
    for node in 0..n {
        if seen[node] {
            continue;
        }
        let mut curr = node;
        let mut dist = 0;
        let mut dists = HashMap::new();
        while !seen[curr] {
            seen[curr] = true;
            dists.insert(curr, dist);
            dist += 1;
            let next = favorite[curr] as usize;
            if let Some(prev) = dists.get(&next) {
                let d = dist - prev;
                cycle_len = cycle_len.max(d);
                if d == 2 {
                    let mut inchain = vec![false; n];
                    inchain[curr] = true;
                    inchain[next] = true;
                    chain_len +=
                        bfs(&innodes, &mut inchain, curr) + bfs(&innodes, &mut inchain, next);
                }
            }
            curr = next;
        }
    }
    cycle_len.max(chain_len)
}

fn bfs(innodes: &[Vec<usize>], inchain: &mut [bool], start: usize) -> i32 {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut res = 0;
    while let Some((curr, dist)) = queue.pop_front() {
        res = res.max(dist);
        for &next in innodes[curr].iter() {
            if !inchain[next] {
                inchain[next] = true;
                queue.push_back((next, 1 + dist));
            }
        }
    }
    1 + res
}

pub fn with_topo_sort(favorite: Vec<i32>) -> i32 {
    let n = favorite.len();
    let mut indegs = favorite.iter().fold(vec![0; n], |mut acc, &fav| {
        acc[fav as usize] += 1;
        acc
    });
    let mut queue: VecDeque<_> = indegs
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v == 0 { Some(i) } else { None })
        .collect();
    let mut depth = vec![1; n];
    while let Some(node) = queue.pop_front() {
        let fav = favorite[node] as usize;
        depth[fav] = depth[fav].max(1 + depth[node]);
        indegs[fav] -= 1;
        if indegs[fav] == 0 {
            queue.push_back(fav);
        }
    }
    let [mut cycle_len, mut chain_len] = [0, 0];
    for node in 0..n {
        if indegs[node] == 0 {
            continue;
        }
        let mut curr = node;
        let mut curr_len = 0;
        while indegs[curr] > 0 {
            indegs[curr] = 0;
            curr_len += 1;
            curr = favorite[curr] as usize;
        }
        if curr_len == 2 {
            chain_len += depth[node] + depth[favorite[node] as usize];
        } else {
            cycle_len = cycle_len.max(curr_len);
        }
    }
    cycle_len.max(chain_len)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(maximum_invitations(vec![2, 2, 1, 2]), 3);
        assert_eq!(maximum_invitations(vec![1, 2, 0]), 3);
        assert_eq!(maximum_invitations(vec![3, 0, 1, 4, 1]), 4);

        assert_eq!(with_topo_sort(vec![2, 2, 1, 2]), 3);
        assert_eq!(with_topo_sort(vec![1, 2, 0]), 3);
        assert_eq!(with_topo_sort(vec![3, 0, 1, 4, 1]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(
            maximum_invitations(vec![1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10]),
            11
        );

        assert_eq!(
            with_topo_sort(vec![1, 0, 3, 2, 5, 6, 7, 4, 9, 8, 11, 10, 11, 12, 10]),
            11
        );
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
