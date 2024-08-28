mod helper;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn get_skyline(buildings: &[[i32; 3]]) -> Vec<[i32; 2]> {
    let mut points = buildings.iter().fold(vec![], |mut acc, &[x1, x2, y]| {
        acc.push([x1, -y]);
        acc.push([x2, y]);
        acc
    });
    points.sort_unstable();

    let mut res = vec![];
    let mut heights = BTreeMap::new();
    let mut curr_height = 0;

    for [curr_x, curr_y] in points {
        if curr_y < 0 {
            *heights.entry(-curr_y).or_insert(0) += 1;
        } else if let Some(v) = heights.get_mut(&curr_y) {
            *v -= 1;
        }

        heights.retain(|_, v| *v > 0);
        let height = heights.keys().last().copied().unwrap_or(0);
        if curr_height != height {
            curr_height = height;
            res.push([curr_x, curr_height])
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            get_skyline(&[
                [2, 9, 10],
                [3, 7, 15],
                [5, 12, 12],
                [15, 20, 10],
                [19, 24, 8],
            ]),
            [
                [2, 10],
                [3, 15],
                [7, 12],
                [12, 0],
                [15, 10],
                [20, 8],
                [24, 0]
            ]
        );
        debug_assert_eq!(get_skyline(&[[0, 2, 3], [2, 5, 3]]), [[0, 3], [5, 0]])
    }

    #[test]
    fn test() {}
}
