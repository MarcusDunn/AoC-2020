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
        let (a, b) = find_pair(&report, 2020)?;
        Some(a * b)
    }

    pub fn find_and_mult_2020_triple(report: Vec<i32>) -> Option<i32> {
        let (a, b, c) = find_triple(&report, 2020)?;
        Some(a * b * c)
    }

    fn find_triple(report: &Vec<i32>, adds_to: i32) -> Option<(i32, i32, i32)> {
        let mut covered: Vec<i32> = Vec::new();
        for &x in report {
            covered.push(x);
            if let Some((a, b)) = find_pair(&covered, adds_to - x) {
                return Some((a, b, x));
            } else {
                continue;
            }
        }
        None
    }
}