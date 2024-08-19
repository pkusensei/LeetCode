mod helper;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn find_order(num_courses: i32, prerequisites: &[[i32; 2]]) -> Vec<i32> {
    let mut graph = vec![vec![]; num_courses as usize];
    let mut in_degree = vec![0; num_courses as usize];
    for nums in prerequisites.iter() {
        graph[nums[1] as usize].push(nums[0]);
        in_degree[nums[0] as usize] += 1;
    }
    let mut queue: VecDeque<_> = in_degree
        .iter()
        .enumerate()
        .filter_map(|(idx, &degree)| if degree == 0 { Some(idx as i32) } else { None })
        .collect();
    let mut res = vec![];
    while let Some(num) = queue.pop_front() {
        res.push(num);
        for &n in graph[num as usize].iter() {
            in_degree[n as usize] -= 1;
            if in_degree[n as usize] == 0 {
                queue.push_back(n);
            }
        }
    }
    if res.len() == num_courses as usize {
        res
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_order(2, &[[1, 0]]), [0, 1]);
        debug_assert_eq!(
            find_order(4, &[[1, 0], [2, 0], [3, 1], [3, 2]]),
            [0, 1, 2, 3]
        );
        debug_assert_eq!(find_order(1, &[]), [0]);
    }

    #[test]
    fn test() {}
}
