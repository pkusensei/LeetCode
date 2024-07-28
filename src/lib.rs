use std::collections::{HashMap, VecDeque};

pub fn second_minimum(n: i32, edges: Vec<[i32; 2]>, time: i32, change: i32) -> i32 {
    let edges: HashMap<i32, Vec<i32>> =
        edges.into_iter().fold(HashMap::new(), |mut acc, [n1, n2]| {
            acc.entry(n1).or_default().push(n2);
            acc.entry(n2).or_default().push(n1);
            acc
        });
    bfs(n, &edges, time, change)
}

fn bfs(n: i32, edges: &HashMap<i32, Vec<i32>>, time: i32, change: i32) -> i32 {
    let mut dist1: Vec<_> = (0..n).map(|_| -1).collect();
    let mut dist2 = dist1.clone();
    let mut queue = VecDeque::new();
    dist1[0] = 0;
    // (node, frequency)
    queue.push_back((1, 1));

    while let Some((node, freq)) = queue.pop_front() {
        let time_taken = {
            let past_time = if freq == 1 {
                dist1[node as usize - 1]
            } else {
                dist2[node as usize - 1]
            };
            if (past_time / change) & 1 == 1 {
                // run into red light
                change * (past_time / change + 1) + time
            } else {
                past_time + time
            }
        };
        for &neighbor in edges[&node].iter() {
            if dist1[neighbor as usize - 1] == -1 {
                dist1[neighbor as usize - 1] = time_taken;
                queue.push_back((neighbor, 1));
            } else if dist2[neighbor as usize - 1] == -1
                && dist1[neighbor as usize - 1] != time_taken
            {
                if neighbor == n {
                    return time_taken;
                }
                dist2[neighbor as usize - 1] = time_taken;
                queue.push_back((neighbor, 2));
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            second_minimum(5, vec![[1, 2], [1, 3], [1, 4], [3, 4], [4, 5]], 3, 5),
            13
        );
        debug_assert_eq!(second_minimum(2, vec![[1, 2]], 3, 2), 11)
    }

    #[test]
    fn test() {}
}
