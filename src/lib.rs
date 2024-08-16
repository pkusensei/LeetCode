mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(arrays: &[&[i32]]) -> i32 {
    if arrays.len() < 2 {
        return 0;
    }
    let curr_min = arrays[0][0];
    let curr_max = arrays[0].last().copied().unwrap_or(curr_min);
    arrays
        .iter()
        .skip(1)
        .fold((curr_min, curr_max, 0), |(curr_min, curr_max, res), arr| {
            let temp_min = arr[0];
            let temp_max = arr.last().copied().unwrap_or(temp_min);
            let res = res
                .max(temp_max.abs_diff(curr_min) as i32)
                .max(curr_max.abs_diff(temp_min) as i32);
            (curr_min.min(temp_min), curr_max.max(temp_max), res)
        })
        .2
}

// pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_distance(&[&[1, 2, 3], &[4, 5], &[1, 2, 3]]), 4);
        debug_assert_eq!(max_distance(&[&[1], &[1]]), 0);
        // debug_assert_eq!(fraction_to_decimal(1, 2), "0.5");
        // debug_assert_eq!(fraction_to_decimal(2, 1), "2");
        // debug_assert_eq!(fraction_to_decimal(4, 333), "0.(012)");
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            max_distance(&[
                &[-8, -7, -7, -5, 1, 1, 3, 4],
                &[-2],
                &[-10, -10, -7, 0, 1, 3],
                &[2]
            ]),
            14
        );
    }
}
