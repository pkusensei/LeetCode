mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

#[derive(Debug, Clone, Default)]
struct UndergroundSystem {
    ins: HashMap<i32, (String, i32)>,
    times: HashMap<[String; 2], Vec<i32>>,
}

impl UndergroundSystem {
    fn new() -> Self {
        Default::default()
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.ins.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let Some((prev, t1)) = self.ins.remove(&id) else {
            return;
        };
        self.times
            .entry([prev, station_name])
            .or_default()
            .push(t - t1);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let v = &self.times[&[start_station, end_station]];
        let sum = v.iter().sum::<i32>();
        f64::from(sum) / (v.len()) as f64
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        let mut un = UndergroundSystem::new();
        un.check_in(45, "Leyton".into(), 3);
        un.check_in(32, "Paradise".into(), 8);
        un.check_in(27, "Leyton".into(), 10);
        un.check_out(45, "Waterloo".into(), 15); // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
        un.check_out(27, "Waterloo".into(), 20); // Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
        un.check_out(32, "Cambridge".into(), 22); // Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
        float_eq(
            un.get_average_time("Paradise".into(), "Cambridge".into()),
            14.0,
        ); // return 14.00000. One trip "Paradise" -> "Cambridge".into(), (14) / 1 = 14
        float_eq(
            un.get_average_time("Leyton".into(), "Waterloo".into()),
            11.0,
        ); // return 11.00000. Two trips "Leyton" -> "Waterloo".into(), (10 + 12) / 2 = 11
        un.check_in(10, "Leyton".into(), 24);
        float_eq(
            un.get_average_time("Leyton".into(), "Waterloo".into()),
            11.0,
        ); // return 11.00000
        un.check_out(10, "Waterloo".into(), 38); // Customer 10 "Leyton" -> "Waterloo" in 38-24 = 14
        float_eq(
            un.get_average_time("Leyton".into(), "Waterloo".into()),
            12.0,
        ); // return 12.00000. Three trips "Leyton" -> "Waterloo".into(), (10 + 12 + 14) / 3 = 12

        let mut un = UndergroundSystem::new();
        un.check_in(10, "Leyton".into(), 3);
        un.check_out(10, "Paradise".into(), 8); // Customer 10 "Leyton" -> "Paradise" in 8-3 = 5
        float_eq(un.get_average_time("Leyton".into(), "Paradise".into()), 5.0); // return 5.00000, (5) / 1 = 5
        un.check_in(5, "Leyton".into(), 10);
        un.check_out(5, "Paradise".into(), 16); // Customer 5 "Leyton" -> "Paradise" in 16-10 = 6
        float_eq(un.get_average_time("Leyton".into(), "Paradise".into()), 5.5); // return 5.50000, (5 + 6) / 2 = 5.5
        un.check_in(2, "Leyton".into(), 21);
        un.check_out(2, "Paradise".into(), 30); // Customer 2 "Leyton" -> "Paradise" in 30-21 = 9
        float_eq(
            un.get_average_time("Leyton".into(), "Paradise".into()),
            6.66667,
        ); // return 6.66667, (5 + 6 + 9) / 3 = 6.66667
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
