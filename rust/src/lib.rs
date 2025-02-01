mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_time_required(jobs: &mut [i32], k: i32) -> i32 {
    jobs.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    let mut res = jobs.iter().sum::<i32>();
    let mut times = vec![0; k as usize];
    dfs(jobs, k as _, 0, &mut times, &mut res);
    res
}

fn dfs(jobs: &[i32], k: usize, idx: usize, times: &mut [i32], res: &mut i32) {
    if idx >= jobs.len() {
        let max = times.iter().copied().max().unwrap_or(i32::MAX);
        *res = (*res).min(max);
        return;
    }
    for i in 0..k {
        if times[i] + jobs[idx] < *res {
            times[i] += jobs[idx];
            dfs(jobs, k, 1 + idx, times, res);
            times[i] -= jobs[idx];
        }
        if times[i] == 0 {
            break; // Assign job to time==0 only once
        }
    }
}

pub fn with_binary_search(jobs: &mut [i32], k: i32) -> i32 {
    fn dfs(jobs: &[i32], times: &mut [i32], k: usize, max: i32, idx: usize) -> bool {
        if idx == jobs.len() {
            return true;
        }
        for i in 0..k {
            if times[i] + jobs[idx] <= max {
                times[i] += jobs[idx];
                if dfs(jobs, times, k, max, 1 + idx) {
                    return true;
                }
                times[i] -= jobs[idx];
                if times[i] == 0 {
                    break;
                }
            }
        }
        false
    }

    let k = k as usize;
    jobs.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    let mut left = jobs[0];
    let mut right = jobs.iter().sum::<i32>();
    while left < right {
        let mid = left + (right - left) / 2;
        let mut times = vec![0; k];
        if dfs(jobs, &mut times, k, mid, 0) {
            right = mid
        } else {
            left = 1 + mid;
        }
    }
    left
}

#[cfg(test)]
mod tests {

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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(minimum_time_required(&mut [3, 2, 3], 3), 3);
        assert_eq!(minimum_time_required(&mut [1, 2, 4, 7, 8], 2), 11);

        assert_eq!(with_binary_search(&mut [3, 2, 3], 3), 3);
        assert_eq!(with_binary_search(&mut [1, 2, 4, 7, 8], 2), 11);
    }

    #[test]
    fn test() {}
}
