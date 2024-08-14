mod helper;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn max_points(points: &[[i32; 2]]) -> i32 {
    if points.len() <= 2 {
        return points.len() as i32;
    }

    let mut lines = HashMap::new();
    for (i, a) in points.iter().enumerate() {
        for b in points.iter().skip(i + 1) {
            let (x0, y0) = (a[0], a[1]);
            let (x1, y1) = (b[0], b[1]);
            let key = if x0 == x1 {
                format!("n/a,{x0}")
            } else {
                let slope = (y1 - y0) as f64 / (x1 - x0) as f64;
                let b = y0 as f64 - x0 as f64 * slope;
                format!("{slope},{b}")
            };
            lines
                .entry(key.clone())
                .or_insert(HashSet::new())
                .insert((x0, y0));
            lines.get_mut(&key).unwrap().insert((x1, y1));
        }
    }
    lines.into_values().map(|s| s.len()).max().unwrap_or(2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_points(&[[1, 1], [2, 2], [3, 3]]), 3);
        debug_assert_eq!(
            max_points(&[[1, 1], [3, 2], [5, 3], [4, 1], [2, 3], [1, 4]]),
            4
        );
    }

    #[test]
    fn test() {}
}
