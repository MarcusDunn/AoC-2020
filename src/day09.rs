use crate::day01::expense_report::ComboSums;
use std::cmp::Ordering;

pub struct XMAS {
    preamble_len: usize,
    contents: Vec<i32>,
}

impl XMAS {
    fn new(contents: Vec<i32>) -> XMAS {
        XMAS {
            contents,
            preamble_len: 25,
        }
    }

    fn find_contiguous_summing_to(&self, goal: i32) -> i32 {
        let mut contiguous = Vec::new();
        let mut i = 0;
        let mut sum = 0;
        loop {
            match sum.cmp(&goal) {
                Ordering::Less => {
                    let new_value = self.contents[i];
                    contiguous.push(new_value);
                    sum += new_value;
                    i += 1
                }
                Ordering::Equal => {
                    return contiguous.iter().max().unwrap() + contiguous.iter().min().unwrap();
                }
                Ordering::Greater => {
                    sum -= contiguous.remove(0);
                }
            }
        }
    }

    fn find_rule_breaker(&self) -> i32 {
        for i in self.preamble_len..self.contents.len() {
            if !self.is_pair_summing_to(i as usize) {
                return *self.contents.get(i).unwrap() as i32;
            }
        }
        unreachable!("should be a number here, I expect valid input")
    }

    fn is_pair_summing_to(&self, index: usize) -> bool {
        self.contents
            .split_at(index)
            .0
            .iter()
            .rev()
            .take(self.preamble_len)
            .copied()
            .collect::<Vec<i32>>()
            .find_combo(self.contents[index], 2)
            .is_some()
    }
}

impl std::convert::From<Vec<i32>> for XMAS {
    fn from(v: Vec<i32>) -> Self {
        XMAS::new(v)
    }
}

#[cfg(test)]
mod test {
    use crate::day09::XMAS;
    use crate::loader::file_to_vec;

    #[test]
    fn test_parse() {
        file_to_vec::<i32>("inputs/day09small.txt");
    }

    #[test]
    fn test_xmas_from() {
        XMAS::from(file_to_vec::<i32>("inputs/day09small.txt"));
    }

    #[test]
    #[ignore] // changed preamble len for large input
    fn test_find_anomaly_small() {
        let res = XMAS::from(file_to_vec::<i32>("inputs/day09small.txt")).find_rule_breaker();
        assert_eq!(res, 127)
    }

    #[test]
    fn test_find_anomaly_large() {
        let res = XMAS::from(file_to_vec::<i32>("inputs/day09.txt")).find_rule_breaker();
        assert_eq!(res, 36845998)
    }

    #[test]
    fn test_find_vec_summing_to_anomaly_large() {
        let code = XMAS::from(file_to_vec::<i32>("inputs/day09.txt"));
        let rule_breaker = code.find_rule_breaker();
        let res = code.find_contiguous_summing_to(rule_breaker);
        assert_eq!(4830226, res);
    }
}
