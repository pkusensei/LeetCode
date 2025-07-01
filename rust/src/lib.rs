mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_area(coords: Vec<Vec<i32>>) -> i64 {
    let [mut x_axes, mut y_axes] = [HashMap::new(), HashMap::new()];
    let [mut xmin, mut ymin] = [i32::MAX; 2];
    let [mut xmax, mut ymax] = [i32::MIN; 2];
    for c in coords.iter() {
        let [x, y] = c[..] else { unreachable!() };
        xmin = xmin.min(x);
        xmax = xmax.max(x);
        ymin = ymin.min(y);
        ymax = ymax.max(y);
        insert(&mut x_axes, x, y);
        insert(&mut y_axes, y, x);
    }
    let res = f(&x_axes, xmin, xmax).max(f(&y_axes, ymin, ymax));
    if res > 0 { res } else { -1 }
}

fn f(map: &HashMap<i32, [i32; 2]>, hmin: i32, hmax: i32) -> i64 {
    let mut res = 0;
    for (base, &[min, max]) in map.iter() {
        res = res
            .max(i64::from(max - min) * i64::from(base.abs_diff(hmin)))
            .max(i64::from(max - min) * i64::from(base.abs_diff(hmax)));
    }
    res
}

fn insert(map: &mut HashMap<i32, [i32; 2]>, a: i32, b: i32) {
    let v = map.entry(a).or_insert([i32::MAX, i32::MIN]);
    v[0] = v[0].min(b);
    v[1] = v[1].max(b);
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn basics() {}

    #[test]
    fn test() {}
}
