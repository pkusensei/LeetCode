#![allow(dead_code)]

pub fn is_palindrome<I>(it: I) -> bool
where
    I: Clone + IntoIterator,
    I::Item: PartialEq,
    I::IntoIter: DoubleEndedIterator,
{
    it.clone()
        .into_iter()
        .zip(it.into_iter().rev())
        .all(|(a, b)| a == b)
}

pub fn get_dimensions<T, U: AsRef<[T]>>(grid: &[U]) -> [usize; 2] {
    let (row, col) = (
        grid.len(),
        grid.first().map(|r| r.as_ref().len()).unwrap_or(0),
    );
    [row, col]
}

pub type Coord = [usize; 2];

pub fn neighbors([a, b]: Coord) -> impl Iterator<Item = Coord> {
    [
        [a.saturating_sub(1), b],
        [a + 1, b],
        [a, b.saturating_sub(1)],
        [a, b + 1],
    ]
    .into_iter()
    .filter(move |&p| p != [a, b])
}

pub const ALL_DIRS: [[i32; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

pub fn around(x: i32, y: i32) -> impl Iterator<Item = Coord> {
    [
        [x - 1, y - 1],
        [x, y - 1],
        [x + 1, y - 1],
        [x - 1, y],
        [x + 1, y],
        [x - 1, y + 1],
        [x, y + 1],
        [x + 1, y + 1],
    ]
    .into_iter()
    .filter_map(|[x, y]| {
        if x >= 0 && y >= 0 {
            Some([x as usize, y as usize])
        } else {
            None
        }
    })
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + std::ops::Rem<Output = T> + PartialEq + From<bool>,
{
    if a == false.into() { b } else { gcd(b % a, a) }
}

pub fn fact_inv(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![1];
    for i in 1..=n {
        fact.push(fact[i - 1] * i as i64 % m);
    }
    let mut invf = vec![0; 1 + n];
    invf[n] = mod_pow(fact[n], m - 2, m);
    for i in (0..n).rev() {
        invf[i] = invf[1 + i] * (1 + i) as i64 % m;
    }
    (fact, invf)
}

pub const fn mod_pow(mut base: i64, mut exp: i64, modu: i64) -> i64 {
    let mut res = 1;
    base %= modu;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base) % modu;
        }
        exp /= 2;
        base = base.pow(2) % modu;
    }
    res
}

pub const fn mod_pow_rec(base: i64, exp: i64, modu: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow_rec(base * base % modu, exp >> 1, modu)
    } else {
        base * mod_pow_rec(base * base % modu, exp >> 1, modu) % modu
    }
}

pub fn kmp<T: PartialEq>(s: &[T], t: &[T]) -> [Vec<usize>; 2] {
    let n = s.len();
    let mut lps = vec![0; n];
    let mut len = 0;
    for idx in 1..n {
        while len > 0 && s[idx] != s[len] {
            len = lps[len - 1];
        }
        if s[idx] == s[len] {
            len += 1;
        }
        lps[idx] = len;
    }
    len = 0;
    // Prefix s[..arr[i]], its length==arr[i]
    // is matched on substr ending on t[i]
    // A better name is matched_lengths
    let mut arr = Vec::with_capacity(t.len());
    for val in t {
        while len > 0 && (len == n || val != &s[len]) {
            len = lps[len - 1];
        }
        if s.get(len).is_some_and(|v| v == val) {
            len += 1;
        }
        arr.push(len);
    }
    [lps, arr]
}

pub fn z_function<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let [mut left, mut right] = [0, 0];
    let mut z = vec![0; n];
    for i in 1..n {
        if i <= right {
            z[i] = (right + 1 - i).min(z[i - left]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > right {
            left = i;
            right = i + z[i] - 1;
        }
    }
    z
}

#[cfg(test)]
mod tests {
    use super::kmp;

    #[test]
    fn test_basic_match() {
        let s = b"ab";
        let t = b"ababcab";
        let [lps, arr] = kmp(s, t);

        assert_eq!(lps, vec![0, 0]);
        assert_eq!(arr, vec![1, 2, 1, 2, 0, 1, 2]);
    }

    #[test]
    fn test_no_match() {
        let s = b"abc";
        let t = b"defgh";
        let [lps, arr] = kmp(s, t);

        assert_eq!(lps, vec![0, 0, 0]);
        assert_eq!(arr, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_partial_overlap() {
        let s = b"abab";
        let t = b"abababab";
        let [lps, arr] = kmp(s, t);

        assert_eq!(lps, vec![0, 0, 1, 2]); // 'ab' repeat
        assert_eq!(arr, vec![1, 2, 3, 4, 3, 4, 3, 4]);
    }

    #[test]
    fn test_single_char() {
        let s = b"a";
        let t = b"aaaa";
        let [lps, arr] = kmp(s, t);

        assert_eq!(lps, vec![0]);
        assert_eq!(arr, vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_empty_pattern() {
        let s: &[u8] = b"";
        let t: &[u8] = b"abcde";
        let [lps, arr] = kmp(s, t);

        assert_eq!(lps, vec![]);
        assert_eq!(arr, vec![0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_empty_text() {
        let s: &[u8] = b"abc";
        let t: &[u8] = b"";
        let [lps, arr] = kmp(s, t);

        assert_eq!(lps, vec![0, 0, 0]);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_both_empty() {
        let s: &[u8] = b"";
        let t: &[u8] = b"";
        let [lps, arr] = kmp(s, t);

        assert_eq!(lps, vec![]);
        assert_eq!(arr, vec![]);
    }
}
