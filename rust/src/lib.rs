mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_task_assign(tasks: &mut [i32], workers: &mut [i32], pills: i32, strength: i32) -> i32 {
    tasks.sort_unstable();
    workers.sort_unstable_by(|a, b| b.cmp(a));
    let [mut left, mut right] = [0, tasks.len()];
    while left < right {
        let mid = left + (right + 1 - left) / 2;
        if check(tasks, workers, pills, strength, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

fn check(tasks: &[i32], workers: &[i32], mut pills: i32, strength: i32, k: usize) -> bool {
    if k > workers.len() {
        return false;
    }
    let mut taski = 0;
    let mut queue = std::collections::VecDeque::new();
    for idx in (0..k).rev() {
        if queue.is_empty() && taski < k {
            queue.push_front(tasks[taski]); // decreasing queue
            taski += 1;
        }
        if queue.back().is_some_and(|&v| v <= workers[idx]) {
            queue.pop_back();
        } else {
            if pills == 0 {
                return false;
            }
            if queue.back().is_some_and(|&v| v > workers[idx] + strength) {
                return false;
            }
            while taski < k && tasks[taski] <= workers[idx] + strength {
                queue.push_front(tasks[taski]);
                taski += 1;
            }
            queue.pop_front(); // this one is done by current worker
            pills -= 1;
        }
    }
    true
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
        assert_eq!(max_task_assign(&mut [3, 2, 1], &mut [0, 3, 3], 1, 1), 3);
        assert_eq!(max_task_assign(&mut [5, 4], &mut [0, 0, 0], 1, 5), 1);
        assert_eq!(
            max_task_assign(&mut [10, 15, 30], &mut [0, 10, 10, 10, 10], 3, 10),
            2
        );
    }

    #[test]
    fn test() {}
}
