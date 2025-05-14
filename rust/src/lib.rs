mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

use std::collections::BTreeSet;

pub fn number_of_alternating_groups(mut colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let n = colors.len();
    let mut tree = FenwickTree::new(n);
    let mut spans = BTreeSet::new();

    if let Some(start) = (0..n).position(|i| colors[(i + n - 1) % n] == colors[i]) {
        let mut left = start; // start of any range
        loop {
            spans.insert(left); // add the start of the interval
            // find end of interval
            let mut right = (left + 1) % n;
            while colors[right] != colors[(right + n - 1) % n] {
                right = (right + 1) % n;
            }
            let len = (right + n - left) % n;
            tree.update(if len == 0 { n } else { len }, 1);
            left = right;
            if left == start {
                break;
            }
        }
    } else {
        // a single consecutive interval, this is a special case
        spans.insert(0);
        tree.update(n, 1);
    }
    let mut res = vec![];
    for q in queries.iter() {
        if let [1, size] = q[..] {
            res.push(query(size as usize, &tree, &spans));
        } else if let [2, i, c] = q[..] {
            toggle(&mut tree, &mut spans, &mut colors, i as usize, c);
        }
    }
    res
}

fn toggle(
    tree: &mut FenwickTree,
    spans: &mut BTreeSet<usize>,
    colors: &mut [i32],
    i: usize,
    c: i32,
) {
    if colors[i] == c {
        return;
    }
    // switch color
    colors[i] = c;
    // find the interval [a,b) containing i
    let n = colors.len();
    let [a, b] = find_containing_interval(spans, i);
    let s = (b + n - a) % n;
    if a == b {
        // there is only one interval (of size n) -> special case
        if n & 1 == 1 {
            // odd cycle
            if (i + 1) % n == a {
                // odd cycle, the non-matching end is shifted by -1, the number of intervals does not change
                spans.clear();
                spans.insert(i);
            } else if i == a {
                // odd cycle, the non-matching end is shifted by +1, the number of intervals does not change
                spans.clear();
                spans.insert((i + 1) % n);
            } else {
                // insert a singleton, splitting the rest
                spans.insert(i);
                spans.insert((i + 1) % n);
                let left_s = (i + n - a) % n;
                let right_s = (b + n - (i + 1)) % n;
                tree.update(n, -1);
                tree.update(left_s, 1);
                tree.update(right_s, 1);
            }
        } else {
            // even cycle
            tree.update(n, -1);
            tree.update(n - 1, 1);
            tree.update(1, 1);
            spans.clear();
            spans.insert(i);
            spans.insert((i + 1) % n);
        }
    } else if s == 1 {
        // i is currently a singleton, switching its color connects *both* adjacent intervals
        // we simply remove both boundaries
        spans.remove(&a);
        spans.remove(&b);
        if spans.is_empty() {
            // special case, this was the last singleton!!
            spans.insert(0);
            tree.update(1, -1);
            tree.update(n - 1, -1);
            tree.update(n, 1);
            return;
        }
        let [new_a, new_b] = find_containing_interval(spans, i);
        let left_s = (i + n - new_a) % n;
        let right_s = (new_b + n - (i + 1)) % n;
        let mut new_s = (new_b + n - new_a) % n;
        if new_s == 0 {
            // special case: only one interval remaining
            new_s = n
        };
        tree.update(1, -1);
        tree.update(left_s, -1);
        tree.update(right_s, -1);
        tree.update(new_s, 1);
    } else if i == a {
        // i is at the left boundary
        // i switches to the interval to the left
        let [left_a, left_b] = find_containing_interval(spans, (i + n - 1) % n);
        let left_s = (left_b + n - left_a) % n;
        tree.update(s, -1);
        tree.update(left_s, -1);
        tree.update(s - 1, 1);
        tree.update(left_s + 1, 1);
        spans.remove(&i);
        spans.insert((i + 1) % n);
    } else if (i + 1) % n == b {
        // i is at the right boundary
        // i switches to the interval to the right
        let [right_a, right_b] = find_containing_interval(spans, (i + 1) % n);
        let right_s = (right_b + n - right_a) % n;
        tree.update(s, -1);
        tree.update(right_s, -1);
        tree.update(s - 1, 1);
        tree.update(right_s + 1, 1);
        spans.remove(&((i + 1) % n));
        spans.insert(i);
    } else {
        // is in the middle of [a,b)
        // this will split the interval in two halfs [a,i), [i+1, b) and one singleton [i,i+1)
        let left_s = (i + n - a) % n;
        let right_s = (b + n - (i + 1)) % n;
        tree.update(s, -1);
        tree.update(1, 1);
        tree.update(left_s, 1);
        tree.update(right_s, 1);
        spans.insert(i);
        spans.insert((i + 1) % n);
    }
}

fn find_containing_interval(spans: &BTreeSet<usize>, i: usize) -> [usize; 2] {
    let left = *spans
        .range(..=i)
        .next_back()
        .or_else(|| spans.last())
        .unwrap();
    let right = *spans
        .range(i + 1..)
        .next()
        .or_else(|| spans.first())
        .unwrap();
    [left, right]
}

fn query(i: usize, tree: &FenwickTree, spans: &BTreeSet<usize>) -> i32 {
    if tree.n & 1 == 0 && spans.len() <= 1 {
        return tree.n as i32;
    }
    let p1 = tree.prefix(tree.n);
    let p2 = tree.prefix(i - 1);
    let y = p1[1] - p2[1];
    let x = p1[0] - p2[0];
    y - (i - 1) as i32 * x
}

struct FenwickTree {
    tree: Vec<[i32; 2]>,
    n: usize,
}

impl FenwickTree {
    fn new(n: usize) -> FenwickTree {
        FenwickTree {
            tree: vec![[0, 0]; n + 1],
            n,
        }
    }

    fn update(&mut self, i: usize, val: i32) {
        let mut idx = i;
        while idx < self.tree.len() {
            self.tree[idx][0] += val;
            self.tree[idx][1] += val * i as i32;
            idx += idx & idx.wrapping_neg();
        }
    }

    // Return A(0,i]
    fn prefix(&self, mut idx: usize) -> [i32; 2] {
        let mut sum0 = 0;
        let mut sum1 = 0;
        while idx > 0 {
            sum0 += self.tree[idx][0];
            sum1 += self.tree[idx][1];
            idx -= idx & idx.wrapping_neg();
        }
        [sum0, sum1]
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
        assert_eq!(
            number_of_alternating_groups(vec![0, 1, 1, 0, 1], vec![vec![2, 1, 0], vec![1, 4]]),
            [2]
        );
        assert_eq!(
            number_of_alternating_groups(
                vec![0, 0, 1, 0, 1, 1],
                vec![vec![1, 3], vec![2, 3, 0], vec![1, 5]]
            ),
            [2, 0]
        );
    }

    #[test]
    fn test() {}
}
