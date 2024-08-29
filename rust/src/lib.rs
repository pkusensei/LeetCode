mod helper;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn remove_stones(stones: &[[i32; 2]]) -> i32 {
    let (xs, ys): (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) = stones.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut xs, mut ys), &[x, y]| {
            xs.entry(x).or_default().push(y);
            ys.entry(y).or_default().push(x);
            (xs, ys)
        },
    );
    let mut seen = HashSet::new();
    stones.iter().fold(0, |acc, &stone| {
        acc + find_cluster(stone, &xs, &ys, &mut seen)
    })
}

fn find_cluster(
    stone: [i32; 2],
    xs: &HashMap<i32, Vec<i32>>,
    ys: &HashMap<i32, Vec<i32>>,
    seen: &mut HashSet<[i32; 2]>,
) -> i32 {
    if !seen.insert(stone) {
        return 0;
    }
    let mut queue = VecDeque::from([stone]);
    let mut res = 0;
    while let Some([curr_x, curr_y]) = queue.pop_front() {
        res += 1;
        for &y in xs[&curr_x].iter() {
            if seen.insert([curr_x, y]) {
                queue.push_back([curr_x, y]);
            }
        }
        for &x in ys[&curr_y].iter() {
            if seen.insert([x, curr_y]) {
                queue.push_back([x, curr_y]);
            }
        }
    }
    res - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            remove_stones(&[[0, 0], [0, 1], [1, 0], [1, 2], [2, 1], [2, 2]]),
            5
        );
        debug_assert_eq!(remove_stones(&[[0, 0], [0, 2], [1, 1], [2, 0], [2, 2]]), 3);
    }

    #[test]
    fn test() {}
}
