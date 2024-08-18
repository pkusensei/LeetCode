mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn nth_ugly_number(n: i32) -> i32 {
    fn with_btreeset(n: i32) -> i32 {
        let mut curr: i32 = 1;
        let mut seen = std::collections::BTreeSet::from([curr]);
        for _ in 0..n {
            curr = seen.pop_first().unwrap();
            for v in [2, 3, 5].into_iter().filter_map(|n| curr.checked_mul(n)) {
                seen.insert(v);
            }
        }
        curr
    }

    let mut nums = Vec::with_capacity(n as usize);
    nums.push(1);

    // indexes into nums vec
    // where next multiples of 2, 3, 5 should occur 
    // as nums[idx] * 2(or 3 or 5)
    let mut idx = [0; 3];
    let mut multiples = [2, 3, 5]; // next multiple of 2, 3, 5

    for _ in 1..n {
        let curr = multiples.into_iter().min().unwrap();
        nums.push(curr);

        if curr == multiples[0] {
            // multiple of 2
            idx[0] += 1;
            multiples[0] = nums[idx[0]] * 2; // generate next multiple of 2
        }
        if curr == multiples[1] {
            // multiple of 3
            idx[1] += 1;
            multiples[1] = nums[idx[1]] * 3; // generate next multiple of 3
        }
        if curr == multiples[2] {
            // multiple of 5
            idx[2] += 1;
            multiples[2] = nums[idx[2]] * 5; // generate next multiple of 5
        }
    }
    nums.last().copied().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(nth_ugly_number(10), 12);
        debug_assert_eq!(nth_ugly_number(1), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(nth_ugly_number(1407), 536870912);
    }
}
