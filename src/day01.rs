pub mod expense_report {
    use std::collections::HashMap;

    fn find_pair(report: &Vec<i32>, adds_to: i32) -> Option<(i32, i32)> {
        let mut partner_lookup: HashMap<i32, i32> = HashMap::new();
        for x in report {
            if let Some((&k, &v)) = partner_lookup.get_key_value(&x) {
                return Some((k, v));
            } else {
                partner_lookup.insert(adds_to - x, *x);
            }
        }
        None
    }

    pub fn find_and_mult_2020_pair(report: Vec<i32>) -> Option<i32> {
        let v = find_n(2, &report, 2020)?;
        return Some(product(v));
    }

    pub fn find_and_mult_2020_triple(report: Vec<i32>) -> Option<i32> {
        let v = find_n(3, &report, 2020)?;
        Some(product(v))
    }

    fn product(vec: Vec<i32>) -> i32 {
        vec.iter().fold(1, |a, b| { a * b })
    }

    pub fn find_n(n: i32, report: &Vec<i32>, adds_to: i32) -> Option<Vec<i32>> {
        if n == 2 {
            if let Some((a, b)) = find_pair(&report, adds_to) {
                return Some(vec![a, b]);
            }
        } else {
            let mut covered: Vec<i32> = Vec::new();
            for x in report {
                if let Some(mut v) = find_n(n - 1, &covered, adds_to - x) {
                    v.push(*x);
                    return Some(v);
                } else {
                    covered.push(*x)
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::day01::expense_report::{find_and_mult_2020_pair, find_and_mult_2020_triple, find_n};
    use std::time::SystemTime;

    #[test]
    fn test_short_pairs() {
        let report = get_short_report();
        let start = SystemTime::now();
        let result = find_and_mult_2020_pair(report).unwrap();
        println!("test_short_pairs took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
        assert_eq!(result, 514579);
    }

    #[test]
    fn test_long_pairs() {
        let report = get_long_report();
        let start = SystemTime::now();
        let result = find_and_mult_2020_pair(report).unwrap();
        println!("test_long_pairs took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
        assert_eq!(result, 357504);
    }

    #[test]
    fn test_short_triples() {
        let report = get_short_report();
        let start = SystemTime::now();
        let result = find_and_mult_2020_triple(report).unwrap();
        println!("test_short_triples took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
        assert_eq!(result, 241861950);
    }

    #[test]
    fn test_long_triples() {
        let report = get_long_report();
        let start = SystemTime::now();
        let result = find_and_mult_2020_triple(report).unwrap();
        println!("test_long_triples took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
        assert_eq!(result, 12747392);
    }

    #[test]
    fn test_long_quadruplet() {
        let report = get_long_report();
        let start = SystemTime::now();
        let result = find_n(4, &report, 2020).unwrap();
        println!("test_long_quadruplet took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
        assert_eq!(result.iter().fold(0, |a, b| { a + b }), 2020)
    }

    #[test]
    fn test_medium_sextuplet() {
        let report = vec![
            1383, 1276, 1613, 1190, 1856, 1528, 1091, 1540, 1720, 1824, 1734, 1919, 1681, 1686,
            1344, 1644, 1670, 1710, 1708, 1458, 1728, 1972, 1630, 1995, 1763, 1935, 451, 1392, 1990,
            14, 1893, 1437, 1632, 1933, 1887, 1975, 1453, 1897, 2005, 2008, 1959, 1716, 1635, 1619,
            543, 231, 123, 11, 1, 4, 55, 24, 342, 1234, 3221, 900, 595, 420, 70, 30, 5, 40, 565,
            3241, 1123, 2234
        ]; // worst case as the correct numbers are all at the end gets insanely long adding much more than this
        let start = SystemTime::now();
        let result = find_n(6, &report, 2020).unwrap();
        println!("test_long_sextuplet took {}ms", SystemTime::now().duration_since(start).unwrap().as_millis());
        assert_eq!(result.iter().fold(0, |a, b| { a + b }), 2020);
    }

    fn get_long_report() -> Vec<i32> {
        vec![
            1974, 1902, 1356, 1724, 1550, 1870, 1436, 1945, 1640, 1766, 1508, 1802, 1495, 1837, 131,
            1754, 1296, 1627, 1768, 1451, 1252, 1566, 1611, 1531, 1868, 1745, 1894, 1799, 1948,
            1930, 1400, 2003, 1777, 1279, 472, 1474, 1787, 1406, 1522, 1646, 1865, 1581, 1609, 1705,
            1383, 1276, 1613, 1190, 1856, 1528, 1091, 1540, 1720, 1824, 1734, 1919, 1681, 1686,
            1344, 1644, 1670, 1710, 1708, 1458, 1728, 1972, 1630, 1995, 1763, 1935, 451, 1392, 1990,
            14, 1893, 1437, 1632, 1933, 1887, 1975, 1453, 1897, 2005, 2008, 1959, 1716, 1635, 1619,
            1994, 1674, 1942, 1817, 1825, 196, 769, 1065, 1662, 1079, 1574, 1554, 1621, 1857, 1312,
            1544, 2001, 1991, 1602, 1669, 1982, 1309, 1556, 1855, 1284, 1641, 1786, 735, 1921, 1661,
            1934, 1552, 1012, 1748, 1782, 1631, 1607, 1659, 1997, 1600, 1594, 1798, 1405, 1790,
            1993, 1960, 1717, 999, 1687, 1771, 1977, 1809, 1884, 1795, 1639, 1565, 1299, 1643, 1700,
            2002, 1823, 1369, 1572, 1657, 1683, 1966, 1606, 1792, 1756, 1936, 1718, 2009, 1711,
            1461, 1638, 1645, 1914, 1963, 1546, 1846, 1737, 1788, 1589, 1860, 1830, 1905, 1571,
            1989, 1780, 1878, 1767, 1776, 1727, 1582, 1769, 1040, 694, 1327, 1623, 1688, 1694, 1932,
            2000, 1969, 1590, 1425, 1917, 1324, 1852, 1753, 1743, 1551,
        ]
    }

    fn get_short_report() -> Vec<i32> {
        vec![1721, 979, 366, 299, 675, 1456]
    }
}
