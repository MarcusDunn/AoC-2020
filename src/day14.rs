use std::collections::HashMap;
use std::fmt;
use std::ops::Add;

#[derive(Debug)]
struct Program {
    instr: Vec<String>,
    mem: HashMap<usize, Base2Num>,
}
#[derive(Debug, Clone)]
struct Base2Num {
    inner: Vec<bool>,
}

impl fmt::Display for Base2Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|&b| match b {
                    true => '1',
                    false => '0',
                })
                .collect::<String>()
        )
    }
}

impl Add<u128> for Base2Num {
    type Output = u128;

    fn add(self, rhs: u128) -> Self::Output {
        self.inner.iter().rev().enumerate().fold(0, |acc, (i, &b)| {
            acc + match b {
                true => 2_u128.pow(i as u32),
                false => 0,
            }
        }) + rhs
    }
}

impl From<i32> for Base2Num {
    fn from(num: i32) -> Self {
        let mut num_cpy = num.clone() as i128;
        let mut inner = Vec::new();
        for i in (0..36).rev() {
            let pow = 2_i128.pow(i);
            if num_cpy - pow >= 0 {
                inner.push(true);
                num_cpy -= pow;
            } else {
                inner.push(false);
            }
        }
        Base2Num { inner }
    }
}

impl Program {
    fn new(instr: Vec<String>) -> Program {
        Program {
            instr,
            mem: HashMap::new(),
        }
    }

    pub fn run_v2(&mut self) {
        let mut curr_mask = String::new();
        for (r, l) in self
            .instr
            .iter()
            .map(|instr| instr.split_once(" = ").unwrap())
            .collect::<Vec<(&str, &str)>>()
        {
            if r.contains("mask") {
                curr_mask = String::from(l);
            } else {
                let addr = Base2Num::from(
                    r.replace("mem[", "")
                        .replace("]", "")
                        .parse::<i32>()
                        .unwrap(),
                );
                let masked_addr = Self::mask_v2(&curr_mask, addr.to_string());
                let poss = Self::find_poss_from_masked(&masked_addr);
                for addr_str in poss {
                    let addr = usize::from_str_radix(addr_str.as_str(), 2).unwrap();
                    self.mem.insert(addr, Base2Num::from(l.parse::<i32>().unwrap()));
                }
            }
        }
    }
    fn mask_v2(mask: &String, num: String) -> String {
        let mut out: Vec<char> = Vec::new();
        let zipped = mask.chars().zip(num.chars());
        for (mask_c, num_c) in zipped.into_iter() {
            if mask_c == '0' {
                out.push(num_c)
            } else if mask_c == '1' {
                out.push(mask_c)
            } else {
                out.push('X')
            }
        }
        out.iter().collect()
    }

    fn find_poss_from_masked(masked: &String) -> Vec<String> {
        Self::find_poss_from_masked_r(vec![masked.chars().collect()])
            .iter()
            .map(|s| s.iter().collect())
            .collect()
    }

    fn find_poss_from_masked_r(masked_vec: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut ret = Vec::new();
        let mut finished = true;
        for mut masked in masked_vec {
            if let Some((i, _)) = masked.iter().enumerate().find(|(_, c)| c == &&'X') {
                finished = false;
                masked[i] = '1';
                ret.push(masked.clone());
                masked[i] = '0';
                ret.push(masked.clone());
            } else {
                ret.push(masked.clone())
            }
        }
        if finished {
            ret
        } else {
            Self::find_poss_from_masked_r(ret)
        }
    }

    fn mask(mask: &String, num: Base2Num) -> Base2Num {
        let mut i = 0;
        let mut out = num.clone();
        while let Some(c) = mask.chars().nth(i) {
            if c == '1' || c == '0' {
                out.inner[i] = c == '1';
            }
            i += 1
        }
        out
    }

    fn run(&mut self) {
        let mut curr_mask = String::new();
        for (r, l) in self
            .instr
            .iter()
            .map(|instr| instr.split_once(" = ").unwrap())
            .collect::<Vec<(&str, &str)>>()
        {
            if r.contains("mask") {
                curr_mask = String::from(l)
            } else {
                let b2 = Self::mask(&curr_mask, Base2Num::from(l.parse::<i32>().unwrap()));
                let addr = r
                    .replace("mem[", "")
                    .replace("]", "")
                    .parse::<usize>()
                    .unwrap();
                self.mem.insert(addr, b2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::{Base2Num, Program};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_parse() {
        parse("inputs/day14small.txt");
    }

    #[test]
    fn test_poss_masked() {
        assert_eq!(
            Program::find_poss_from_masked(&"000000000000000000000000000000X1101X".to_string())
                .sort(),
            vec![
                "000000000000000000000000000000011010",
                "000000000000000000000000000000011011",
                "000000000000000000000000000000111010",
                "000000000000000000000000000000111011"
            ]
            .sort()
        )
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(
            Base2Num::from(11)
                .inner
                .iter()
                .map(|&b| match b {
                    true => {
                        '1'
                    }
                    false => {
                        '0'
                    }
                })
                .collect::<String>(),
            "000000000000000000000000000000001011"
        );
    }

    #[test]
    fn test_to_u128() {
        assert_eq!(Base2Num::from(11) + 0, 11)
    }

    #[test]
    fn test_p1_small() {
        let mut prog = parse("inputs/day14small.txt");
        prog.run();
        assert_eq!(
            165,
            prog.mem.iter().fold(0, |acc, (_, x)| x.clone() + acc)
        );
    }

    #[test]
    fn test_p1() {
        let mut prog = parse("inputs/day14.txt");
        prog.run();
        assert_eq!(
            15018100062885,
            prog.mem.iter().fold(0, |acc, (_, x)| x.clone() + acc)
        );
    }

    #[test]
    fn test_mask_v2() {
        let res = Program::mask_v2(
            &String::from("000000000000000000000000000000X1001X"),
            String::from("000000000000000000000000000000101010"),
        );
        assert_eq!(res, "000000000000000000000000000000X1101X");
    }

    #[test]
    fn test_p2_small() {

        timed!({
        let mut prog = parse("inputs/day14smallp2.txt");
        prog.run_v2();
        assert_eq!(
            208,
            prog.mem.iter().fold(0, |acc, (_, x)| x.clone() + acc)
        );
        }, "test_p2_small"
        );
    }

    #[test]
    fn test_p2() {
        timed!({
        let mut prog = parse("inputs/day14.txt");
        prog.run_v2();
        assert_eq!(
            5724245857696,
            prog.mem.iter().fold(0, |acc, (_, x)| x.clone() + acc)
        );
        }, "test_p2");

    }


    fn parse(file: &str) -> Program {
        Program::new(
            BufReader::new(File::open(file).unwrap())
                .lines()
                .map(Result::unwrap)
                .collect(),
        )
    }
}
