mod helper;
mod trie;

use std::{cmp::Ordering, collections::HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn outer_trees(mut trees: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    if trees.len() < 3 {
        return trees;
    }
    // // Most bottom-then-left point
    // let start = trees.iter().copied().min().unwrap();
    // let i = trees.iter().position(|p| *p == start).unwrap();
    // trees.swap_remove(i);

    // // sort all points according to start in counter-clockwise fashion
    // trees.sort_unstable_by(|a, b| {
    //     let order = orientation(start, *a, *b);
    //     if order == Ordering::Equal {
    //         let d1 = (a[0] - start[0]).pow(2) + (a[1] - start[1]).pow(2);
    //         let d2 = (b[0] - start[0]).pow(2) + (b[1] - start[1]).pow(2);
    //         d1.cmp(&d2)
    //     } else {
    //         order
    //     }
    // });

    // let mut stack = vec![start];
    // stack.extend_from_slice(&trees[..2]);
    // for &p in trees.iter().skip(2) {
    //     while stack.len() > 1
    //         && orientation(stack[stack.len() - 2], stack[stack.len() - 1], p) == Ordering::Greater
    //     {
    //         // Check is current stack.last() forms a clockwise orientation, i.e. == Greater
    //         // Pop last if it does
    //         // Linear is kept to allow passthru
    //         // Otherwise use != Less
    //         stack.pop();
    //     }
    //     stack.push(p);
    // }
    // stack

    // All above as regular Graham's scan
    // But it handles linear cases poorly
    // Hence doing forward and backward scans below
    // And using a set to remove dups
    trees.sort_unstable();
    let st1 = produce(trees.iter().copied());
    let st2 = produce(trees.iter().copied().rev());
    st1.into_iter()
        .chain(st2)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

// monotonic stack
fn produce(points: impl Iterator<Item = [i32; 2]>) -> Vec<[i32; 2]> {
    let mut res = vec![];
    for p in points {
        while res.len() > 1
            && orientation(res[res.len() - 2], res[res.len() - 1], p) == Ordering::Greater
        {
            res.pop();
        }
        res.push(p);
    }
    res
}

fn orientation(p1: [i32; 2], p2: [i32; 2], p3: [i32; 2]) -> Ordering {
    let val = (p2[1] - p1[1]) * (p3[0] - p2[0]) - (p2[0] - p1[0]) * (p3[1] - p2[1]);
    val.cmp(&0)
    // to turn vector p1->p2 to p1->p3
    // Equal => linear
    // Greater => clockwise [0,0], [1,1], [2,0]
    // Less => counterclockwise [0,0], [1,1], [1,2]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            outer_trees(vec![[1, 1], [2, 2], [2, 0], [2, 4], [3, 3], [4, 2]]),
            [[1, 1], [2, 0], [4, 2], [3, 3], [2, 4]],
        );
        sort_eq(
            outer_trees(vec![[1, 2], [2, 2], [4, 2]]),
            [[1, 2], [2, 2], [4, 2]],
        );
    }

    #[test]
    fn test() {
        #[rustfmt::skip]
        sort_eq(
            outer_trees(vec![
                [3, 0],[4, 0],[5, 0],[6, 1],[7, 2],[7, 3],[7, 4],[6, 5],[5, 5],
                [4, 5],[3, 5],[2, 5],[1, 4],[1, 3],[1, 2],[2, 1],[4, 2],[0, 3],
            ]),
            [
                [4, 5],[2, 5],[6, 1],[3, 5],[2, 1],[1, 4],[1, 2],[7, 4],
                [7, 3],[7, 2],[3, 0],[0, 3],[5, 0],[5, 5],[4, 0],[6, 5],
            ],
        );
        sort_eq(
            outer_trees(vec![[0, 0], [0, 100], [100, 100], [100, 0], [50, 50]]),
            [[0, 100], [100, 0], [100, 100], [0, 0]],
        );
    }

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
