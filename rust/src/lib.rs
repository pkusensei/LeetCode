mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_reach_corner_v2(x_corner: i32, y_corner: i32, circles: &[[i32; 3]]) -> bool {
    let n = circles.len();
    let mut dsu = DSU::new(n + 2); // 0..n-1 for circles, n for left/top, n+1 for right/bot
    for (i1, c1) in circles.iter().enumerate() {
        let &[x1, y1, r1] = c1;
        for (i2, c2) in circles.iter().enumerate().skip(1 + i1) {
            let &[x2, y2, r2] = c2;
            if inter_circles(x1, y1, r1, x2, y2, r2, x_corner, y_corner) {
                dsu.union(i1, i2);
            }
        }
        if inter_circle_seg(x1, y1, r1, 0, 0, 0, y_corner) {
            dsu.union(i1, n);
        }
        if inter_circle_seg(x1, y1, r1, 0, y_corner, x_corner, y_corner) {
            dsu.union(i1, n);
        }
        if inter_circle_seg(x1, y1, r1, 0, 0, x_corner, 0) {
            dsu.union(i1, n + 1);
        }
        if inter_circle_seg(x1, y1, r1, x_corner, 0, x_corner, y_corner) {
            dsu.union(i1, n + 1);
        }
    }
    dsu.find(n) != dsu.find(n + 1)
}

fn inter_circles(
    x1: i32,
    y1: i32,
    r1: i32,
    x2: i32,
    y2: i32,
    r2: i32,
    x_max: i32,
    y_max: i32,
) -> bool {
    let dx = f64::from(x2 - x1);
    let dy = f64::from(y2 - y1);
    let d = (dx * dx + dy * dy).sqrt();
    let r1 = f64::from(r1);
    let r2 = f64::from(r2);
    if d == 0.0 || d < (r1 - r2).abs() || d > r1 + r2 {
        return false;
    }
    let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
    let h = (r1 * r1 - a * a).sqrt();
    let x = f64::from(x1) + a * dx / d;
    let y = f64::from(y1) + a * dy / d;
    let dx_h = h * dy / d;
    let dy_h = h * dx / d;
    let x1_inter = x - dx_h;
    let y1_inter = y + dy_h;
    let x2_inter = x + dx_h;
    let y2_inter = y - dy_h;
    (0.0 < x1_inter && x1_inter < f64::from(x_max) && 0.0 < y1_inter && y1_inter < f64::from(y_max))
        || (0.0 < x2_inter
            && x2_inter < f64::from(x_max)
            && 0.0 < y2_inter
            && y2_inter < f64::from(y_max))
}

fn inter_circle_seg(xc: i32, yc: i32, rc: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    if x1 == x2 {
        let closest_y = yc.clamp(y1, y2);
        let dx = i64::from(xc - x1);
        let dy = i64::from(yc - closest_y);
        dx * dx + dy * dy <= i64::from(rc).pow(2)
    } else {
        let closest_x = xc.clamp(x1, x2);
        let dx = i64::from(xc - closest_x);
        let dy = i64::from(yc - y1);
        dx * dx + dy * dy <= i64::from(rc).pow(2)
    }
}

pub fn can_reach_corner(x_corner: i32, y_corner: i32, circles: &[[i32; 3]]) -> bool {
    let n = circles.len();
    let mut dsu = DSU::new(4 + n);
    let left = [0, 0, 0, y_corner]; // n
    let right = [x_corner, 0, x_corner, y_corner]; // 1+n
    let up = [0, y_corner, x_corner, y_corner]; // 2+n
    let down = [0, 0, x_corner, 0]; // 3+n
    for (i1, c1) in circles.iter().enumerate() {
        if (i64::from(c1[0]) - i64::from(x_corner)).pow(2)
            + (i64::from(c1[1]) - i64::from(y_corner)).pow(2)
            <= i64::from(c1[2]).pow(2)
        {
            return false;
        }
        for (i, line) in [left, right, up, down].iter().enumerate() {
            if touch_line(c1, line) {
                dsu.union(i1, i + n);
            }
        }
        for (i2, c2) in circles.iter().enumerate().skip(1 + i1) {
            if touch_circle(c1, c2, x_corner, y_corner) {
                dsu.union(i1, i2);
            }
        }
    }
    for [a, b] in [[0, 1], [0, 3], [1, 2], [2, 3]] {
        if dsu.find(a + n) == dsu.find(b + n) {
            return false;
        }
    }
    true
}

fn touch_circle(c1: &[i32], c2: &[i32], x_corner: i32, y_corner: i32) -> bool {
    let [cx1, cy1, r1] = c1[..] else {
        return false;
    };
    let [cx2, cy2, r2] = c2[..] else {
        return false;
    };
    let dx = i64::from(cx1) - i64::from(cx2);
    let dy = i64::from(cy1) - i64::from(cy2);
    let d_sq = dx.pow(2) + dy.pow(2);
    if d_sq > (i64::from(r1) + i64::from(r2)).pow(2) {
        return false;
    }
    if d_sq == 0 {
        return overlaps_rect(cx1, cy1, r1.max(r2), x_corner, y_corner);
    }
    let d = (d_sq as f64).sqrt();
    // Compute one of the intersection points of the two circles
    let a = (i64::from(r1).pow(2) - i64::from(r2).pow(2) + d_sq) as f64 / (2.0 * d);
    let h_sq = i64::from(r1).pow(2) as f64 - a * a;
    if h_sq < -1e-6 {
        return false; // Numerical edge case, no intersection
    }
    if h_sq < 0.0 {
        return overlaps_rect(cx1, cy1, r1.max(r2), x_corner, y_corner);
    }
    let h = h_sq.sqrt();
    let xm = f64::from(cx1) + a * f64::from(cx2 - cx1) / d;
    let ym = f64::from(cy1) + a * f64::from(cy2 - cy1) / d;
    // Intersection points
    let rx = -f64::from(cy2 - cy1) * (h / d);
    let ry = f64::from(cx2 - cx1) * (h / d);
    let xi1 = xm + rx;
    let yi1 = ym + ry;
    let xi2 = xm - rx;
    let yi2 = ym - ry;
    rect_contains_point(xi1, yi1, x_corner, y_corner)
        || rect_contains_point(xi2, yi2, x_corner, y_corner)
}

fn rect_contains_point(x: f64, y: f64, x_corner: i32, y_corner: i32) -> bool {
    (0.0..=f64::from(x_corner)).contains(&x) && (0.0..=f64::from(y_corner)).contains(&y)
}

fn overlaps_rect(cx: i32, cy: i32, r: i32, x_corner: i32, y_corner: i32) -> bool {
    let closest_x = cx.clamp(0, x_corner);
    let closest_y = cy.clamp(0, y_corner);
    let dx = i64::from(closest_x) - i64::from(cx);
    let dy = i64::from(closest_y) - i64::from(cy);
    dx.pow(2) + dy.pow(2) <= i64::from(r).pow(2)
}

fn touch_line(circle: &[i32], line: &[i32; 4]) -> bool {
    let [cx, cy, r] = circle[..] else {
        return false;
    };
    let &[x1, y1, x2, y2] = line;
    if x1 == x2 {
        let dx = i64::from(cx.abs_diff(x1));
        if dx > i64::from(r) {
            return false;
        }
        let dy = (f64::from(r).powi(2) - (dx as f64).powi(2)).sqrt();
        0.0_f64.max(f64::from(cy) - dy) <= f64::from(y2).min(f64::from(cy) + dy)
    } else {
        assert_eq!(y1, y2);
        let dy = i64::from(cy.abs_diff(y1));
        if dy > i64::from(r) {
            return false;
        }
        let dx = (f64::from(r).powi(2) - (dy as f64).powi(2)).sqrt();
        0.0_f64.max(f64::from(cx) - dx) <= f64::from(x2).min(f64::from(cx) + dx)
    }
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.rank[rx] += 1;
                self.parent[ry] = rx;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
    }
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
        assert!(can_reach_corner_v2(3, 4, &[[2, 1, 1]]));
        assert!(!can_reach_corner_v2(3, 3, &[[1, 1, 2]]));
        assert!(!can_reach_corner_v2(3, 3, &[[2, 1, 1], [1, 2, 1]]));
        assert!(can_reach_corner_v2(4, 4, &[[5, 5, 1]]));

        assert!(can_reach_corner(3, 4, &[[2, 1, 1]]));
        assert!(!can_reach_corner(3, 3, &[[1, 1, 2]]));
        assert!(!can_reach_corner(3, 3, &[[2, 1, 1], [1, 2, 1]]));
        assert!(can_reach_corner(4, 4, &[[5, 5, 1]]));
    }

    #[test]
    fn test() {
        assert!(can_reach_corner_v2(3, 3, &[[2, 1000, 997], [1000, 2, 997]]));
        assert!(!can_reach_corner_v2(20, 100, &[[1, 102, 18], [50, 60, 48]]));
        assert!(!can_reach_corner_v2(15, 15, &[[2, 20, 13], [20, 2, 13]]));
        assert!(can_reach_corner_v2(
            500_000_000,
            500_000_000,
            &[
                [499_980_000, 699_999_999, 200_000_000],
                [500_020_000, 300_000_001, 200_000_000]
            ]
        ));

        assert!(can_reach_corner(3, 3, &[[2, 1000, 997], [1000, 2, 997]]));
        assert!(!can_reach_corner(20, 100, &[[1, 102, 18], [50, 60, 48]]));
        assert!(!can_reach_corner(15, 15, &[[2, 20, 13], [20, 2, 13]]));
        assert!(can_reach_corner(
            500_000_000,
            500_000_000,
            &[
                [499_980_000, 699_999_999, 200_000_000],
                [500_020_000, 300_000_001, 200_000_000]
            ]
        ));
    }
}
