mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(side: i32, points: &[[i32; 2]], k: i32) -> i32 {
    use itertools::{Itertools, chain};
    let [mut left, mut right, mut top, mut bot] = [const { vec![] }; 4];
    for p in points.iter() {
        let [x, y] = p[..] else { unreachable!() };
        if x == 0 && y > 0 {
            left.push([x, y]);
        } else if x == side && y < side {
            right.push([x, y]);
        } else if x > 0 && y == side {
            top.push([x, y]);
        } else {
            bot.push([x, y]);
        }
    }
    left.sort_unstable();
    top.sort_unstable();
    right.sort_unstable();
    bot.sort_unstable();
    let sorted = chain!(left, top, right.into_iter().rev(), bot.into_iter().rev()).collect_vec();

    let mut low = 0;
    let mut high = side;
    while low < high {
        let mid = low + (high - low + 1) / 2;
        if pick(&sorted, mid) >= k {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    low
}

#[derive(Clone, Copy)]
struct Item {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    count: i32,
}

fn pick(sorted: &[[i32; 2]], mid: i32) -> i32 {
    let mut queue = std::collections::VecDeque::from([Item {
        x1: sorted[0][0],
        y1: sorted[0][1],
        x2: sorted[0][0],
        y2: sorted[0][1],
        count: 1,
    }]);
    let mut max_count = 1;
    for &[x, y] in &sorted[1..] {
        let [mut start_x, mut start_y] = [x, y];
        let mut curr_count = 1;
        while queue
            .front()
            .is_some_and(|&Item { x2, y2, .. }| (x - x2).abs() + (y - y2).abs() >= mid)
        {
            let Item { x1, y1, count, .. } = queue.pop_front().unwrap();
            if (x - x1).abs() + (y - y1).abs() >= mid && count + 1 >= curr_count {
                start_x = x1;
                start_y = y1;
                curr_count = count + 1;
                max_count = max_count.max(curr_count)
            }
        }
        queue.push_back(Item {
            x1: start_x,
            y1: start_y,
            x2: x,
            y2: y,
            count: curr_count,
        });
    }
    max_count
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
        assert_eq!(max_distance(2, &[[0, 2], [2, 0], [2, 2], [0, 0]], 4), 2);
        assert_eq!(
            max_distance(2, &[[0, 0], [1, 2], [2, 0], [2, 2], [2, 1]], 4),
            1
        );
        assert_eq!(
            max_distance(
                2,
                &[[0, 0], [0, 1], [0, 2], [1, 2], [2, 0], [2, 2], [2, 1]],
                5
            ),
            1
        );
    }

    #[test]
    fn test() {}
}
