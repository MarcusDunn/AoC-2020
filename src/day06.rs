mod day06 {
    use std::collections::{HashMap};
    use std::str::FromStr;

    #[derive(Debug)]
    pub struct Group {
        yeses: HashMap<char, usize>,
        size: usize,
    }

    impl Group {
        pub fn amount_all_yeses(&self) -> usize {
            self.yeses.iter().filter(|(_, i)| self.size.eq(i)).count()
        }
    }

    impl FromStr for Group {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let size = s.chars().filter(|c| c == &' ').count() + 1;
            let filtered = s.chars().filter(|c| c != &' ').collect::<Vec<char>>();
            let mut yeses = HashMap::new();
            for char in filtered {
                if let Some(&amount) = yeses.get(&char) {
                    yeses.insert(char, amount + 1);
                } else {
                    yeses.insert(char, 1);
                }
            }
            Ok(Group { yeses, size })
        }
    }
}

#[cfg(test)]
mod day06test {
    use crate::day06::day06::Group;
    use crate::loader::loader::file_to_vec_by_blank_lines;

    #[test]
    fn test_small() {
        let result = timed!(
            file_to_vec_by_blank_lines::<Group>("inputs/day06small.txt")
                .iter()
                .fold(0, |acc, x| acc + x.amount_all_yeses()),
            "test_small"
        );
        assert_eq!(result, 6)
    }

    #[test]
    fn test_large() {
        let result = timed!(
            file_to_vec_by_blank_lines::<Group>("inputs/day06.txt")
                .iter()
                .fold(0, |acc, x| acc + x.amount_all_yeses()),
            "test_large"
        );
        assert_eq!(result, 3290)
    }
}
