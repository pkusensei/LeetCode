mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn closest_room(rooms: &mut [[i32; 2]], queries: &[[i32; 2]]) -> Vec<i32> {
    let n = rooms.len();
    // sort by size, then by id
    rooms.sort_unstable_by(|a, b| a[1].cmp(&b[1]).then(a[0].cmp(&b[0])));
    let mut res = vec![];
    for q in queries.iter() {
        let [pref, min] = [q[0], q[1]];
        let i = rooms.partition_point(|v| v[1] < min);
        if i == n {
            res.push(-1);
            continue;
        }
        let mut curr = 0;
        let mut diff = u32::MAX;
        for r in rooms[i..].iter() {
            let id = r[0];
            match id.abs_diff(pref).cmp(&diff) {
                std::cmp::Ordering::Less => {
                    curr = id;
                    diff = id.abs_diff(pref);
                }
                std::cmp::Ordering::Equal => curr = curr.min(id),
                std::cmp::Ordering::Greater => (),
            }
        }
        res.push(curr);
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
    fn basics() {
        assert_eq!(
            closest_room(&mut [[2, 2], [1, 2], [3, 2]], &[[3, 1], [3, 3], [5, 2]]),
            [3, -1, 3]
        );
        assert_eq!(
            closest_room(
                &mut [[1, 4], [2, 3], [3, 5], [4, 1], [5, 2]],
                &[[2, 3], [2, 4], [2, 5]]
            ),
            [2, 1, 3]
        );
    }

    #[test]
    fn test() {}
}
