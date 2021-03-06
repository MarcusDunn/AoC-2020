pub mod expense_report {
    use std::collections::HashMap;

    fn find_pair(report: &[i32], adds_to: i32) -> Option<(i32, i32)> {
        let mut partner_lookup: HashMap<i32, i32> = HashMap::new();
        for &x in report {
            if let Some(&v) = partner_lookup.get(&x) {
                return Some((x, v));
            } else {
                partner_lookup.insert(adds_to - x, x);
            }
        }
        None
    }

    impl ComboSums<i32> for [i32] {
        fn find_combo(&self, goal: i32, vec_len: i32) -> Option<Vec<i32>> {
            if vec_len == 2 {
                if let Some(pair) = find_pair(self, goal) {
                    return Some(vec![pair.0, pair.1]);
                }
            } else {
                let mut covered: Vec<i32> = Vec::new();
                for &x in self {
                    if let Some(mut v) = covered.find_combo(goal - x, vec_len - 1) {
                        v.push(x);
                        return Some(v);
                    } else {
                        covered.push(x)
                    }
                }
            }
            None
        }

        fn find_combo_product(&self, goal: i32, vec_len: i32) -> Option<i32> {
            Some(self.find_combo(goal, vec_len)?.iter().product())
        }
    }

    pub trait ComboSums<T> {
        fn find_combo(&self, sum_goal: T, vec_len: i32) -> Option<Vec<T>>;
        fn find_combo_product(&self, sum_goal: T, vec_len: i32) -> Option<T>;
    }
}

#[cfg(test)]
mod test {
    use crate::day01::expense_report::ComboSums;
    use crate::loader::file_to_vec;

    #[test]
    fn test_short_pairs() {
        let report = get_short_report();
        let result = timed!(
            report.find_combo_product(2020, 2).unwrap(),
            "test_short_pairs"
        );
        assert_eq!(result, 514579);
    }

    #[test]
    fn test_long_pairs() {
        let report = get_long_report();
        let result = timed!(
            report.find_combo_product(2020, 2).unwrap(),
            "test_long_pairs"
        );
        assert_eq!(result, 357504);
    }

    #[test]
    fn test_short_triples() {
        let report = get_short_report();
        let result = timed!(
            report.find_combo_product(2020, 3).unwrap(),
            "test_short_triples"
        );
        assert_eq!(result, 241861950);
    }

    #[test]
    fn test_long_triples() {
        let report = get_long_report();
        let result = timed!(
            report.find_combo_product(2020, 3).unwrap(),
            "test_long_triples"
        );
        assert_eq!(result, 12747392);
    }

    #[test]
    fn test_long_quadruplet() {
        let report = get_long_report();
        let result = timed!(report.find_combo(2020, 4).unwrap(), "test_long_quadruplet");
        assert_eq!(result.iter().fold(0, |a, b| a + b), 2020)
    }

    #[test]
    fn test_medium_sextuplet() {
        let report = get_medium_report(); // worst case as the correct numbers are all at the end. Time gets insanely long adding much more to the front than this (adding to the back has a lesser effect)
        let result = timed!(report.find_combo(2020, 6).unwrap(), "test_medium_sextuplet");
        assert_eq!(result.iter().fold(0, |a, b| a + b), 2020);
    }

    fn get_long_report() -> Vec<i32> {
        file_to_vec("inputs/day01.txt")
    }

    fn get_medium_report() -> Vec<i32> {
        vec![
            1383, 1276, 1613, 1190, 1856, 1528, 1091, 1540, 1720, 1824, 1734, 1919, 1681, 1686,
            1344, 1644, 1670, 1710, 1708, 1458, 1728, 1972, 1630, 1995, 1763, 1935, 451, 1392,
            1990, 14, 1893, 1437, 1632, 1933, 1887, 1975, 1453, 1897, 2005, 2008, 1959, 1716, 1635,
            1619, 543, 231, 123, 11, 1, 4, 55, 24, 342, 1234, 3221, 900, 595, 420, 70, 30, 5, 40,
            565, 3241, 1123, 2234,
        ]
    }

    fn get_short_report() -> Vec<i32> {
        vec![1721, 979, 366, 299, 675, 1456]
    }
}
