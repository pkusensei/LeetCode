mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn second_minimum(n: i32, edges: &[[i32; 2]], time: i32, change: i32) -> i32 {
    let n = n as usize;
    let adj = edges.iter().fold(vec![vec![]; n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].push(b);
        acc[b].push(a);
        acc
    });
    let [mut dist1, mut dist2] = [0, 1].map(|_| vec![-1; n]);
    dist1[0] = 0;
    let mut queue = std::collections::VecDeque::from([(0, 1)]);
    while let Some((node, freq)) = queue.pop_front() {
        let mut time_taken = if freq == 1 { dist1[node] } else { dist2[node] };
        if (time_taken / change) & 1 == 1 {
            time_taken = (time_taken / change + 1) * change + time; // red light
        } else {
            time_taken += time;
        }
        for &next in adj[node].iter() {
            if dist1[next] == -1 {
                dist1[next] = time_taken;
                queue.push_back((next, 1));
            } else if dist2[next] == -1 && dist1[next] != time_taken {
                if next == n - 1 {
                    return time_taken;
                }
                dist2[next] = time_taken;
                queue.push_back((next, 2));
            }
        }
    }
    -1
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
            second_minimum(5, &[[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3, 5),
            13
        );
        assert_eq!(second_minimum(2, &[[1, 2]], 3, 2), 11);
    }

    #[test]
    fn test() {}
}
