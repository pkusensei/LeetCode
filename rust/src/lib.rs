mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_distances_in_tree(n: i32, edges: &[[i32; 2]]) -> Vec<i32> {
    let n = n as usize;
    let mut res = vec![0; n];
    if edges.is_empty() {
        return res;
    }
    let mut tree = vec![vec![]; n];
    for e in edges.iter() {
        tree[e[0] as usize].push(e[1] as usize);
        tree[e[1] as usize].push(e[0] as usize);
    }
    let mut count = vec![1; n]; // subtree size from each node
    postorder(0, 1 + n, &tree, &mut res, &mut count);
    preorder(0, 1 + n, n, &tree, &mut res, &mut count);
    res
}

fn postorder(node: usize, parent: usize, tree: &[Vec<usize>], dist: &mut [i32], count: &mut [i32]) {
    for &child in tree[node].iter() {
        if child == parent {
            continue; // avoid going back to parent
        }
        // count subtrees first
        postorder(child, node, tree, dist, count);
        // Example: 0->1->[2,3],
        // For subtree 1->[2,3], dist[1]==2, count[1]==3
        // To link subtree back to parent 0,
        // Each child node's dist has to increment 1
        // Add in the dist 1 for node 1
        // They add up to count[child]
        dist[node] += dist[child] + count[child];
        count[node] += count[child];
    }
}

fn preorder(
    node: usize,
    parent: usize,
    n: usize,
    tree: &[Vec<usize>],
    dist: &mut [i32],
    count: &mut [i32],
) {
    for &child in tree[node].iter() {
        if child == parent {
            continue;
        }
        // Still 0->1->[2,3]
        // When moving root from 0->1
        // Temporarily dist[1]=dist[0]
        // For subtree 1->[2,3], dist[1]-=count[1]
        // If node 0 has another branch 0->5
        // dist[1] +=
        // (count[0]-count[1]) == (n - count[1])
        dist[child] = dist[node] - count[child] + (n as i32 - count[child]);
        preorder(child, node, n, tree, dist, count);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            sum_of_distances_in_tree(6, &[[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]]),
            [8, 12, 6, 10, 10, 10]
        );
        debug_assert_eq!(sum_of_distances_in_tree(1, &[]), [0]);
        debug_assert_eq!(sum_of_distances_in_tree(2, &[[1, 0]]), [1, 1]);
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
