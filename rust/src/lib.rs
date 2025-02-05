mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_collision_times(cars: &[[i32; 2]]) -> Vec<f64> {
    let n = cars.len();
    let mut stack: Vec<usize> = vec![];
    let mut res = vec![-1.0; n];
    for (idx, c) in cars.iter().enumerate().rev() {
        let [pos, speed] = c[..] else {
            unreachable!();
        };
        // When viewed from the right/in for-loop sequence
        // 1) Maintain a mono-increasing stack on speed
        // e.g with speeds 4 2 5 => 4 catches 2, but none catches 5
        // Hence stack [5] => [2] => [4,2]
        // 2) a mono-decreasing stack on time
        // Front already collided and this car takes longer to catch up
        while let Some(&prev) = stack.last() {
            // [pos2, speed2] is in front
            // 1) speed2 is higher => never reaches => pop!
            // 2) takes longer to catch up than its "collision" time => pop!
            let [pos2, speed2] = cars[prev][..] else {
                unreachable!()
            };
            if speed <= speed2 {
                stack.pop();
            } else {
                let catch = f64::from(pos2 - pos) / f64::from(speed - speed2);
                if res[prev] > 0.0 && catch >= res[prev] {
                    stack.pop();
                } else {
                    res[idx] = catch;
                    break;
                }
            }
        }
        stack.push(idx);
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
        for (a, b) in get_collision_times(&[[1, 2], [2, 1], [4, 3], [7, 2]])
            .into_iter()
            .zip([1.00000, -1.00000, 3.00000, -1.00000])
        {
            float_eq!(a, b)
        }
        for (a, b) in get_collision_times(&[[3, 4], [5, 4], [6, 3], [9, 1]])
            .into_iter()
            .zip([2.00000, 1.00000, 1.50000, -1.00000])
        {
            float_eq!(a, b)
        }
    }

    #[test]
    fn test() {}
}
