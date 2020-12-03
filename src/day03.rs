pub mod day03 {
    use std::option::NoneError;
    use std::str::FromStr;

    pub struct Row {
        contents: Vec<Option<()>>,
    }

    impl Row {
        pub fn get(&self, index: usize) -> Option<()> {
            self.contents[index % self.contents.len()]
        }
    }

    pub struct Forest {
        pub contents: Vec<Row>,
    }

    impl Forest {
        fn get(&self, index: usize) -> Option<&Row> {
            self.contents.get(index)
        }

        pub fn trees_hit(&self, dx: usize, dy: usize) -> i32 {
            let mut ret = 0;
            let mut counter: usize = 0;

            while let Some(row) = self.get(counter * dy) {
                if row.get(counter * dx).is_some() {
                    ret = ret + 1;
                }
                counter = counter + 1;
            }
            ret
        }
    }

    impl FromStr for Row {
        type Err = NoneError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Row {
                contents: s
                    .chars()
                    .map(|c| match c {
                        '#' => Some(()),
                        _ => None,
                    })
                    .collect(),
            })
        }
    }
}

#[cfg(test)]
mod day03test {
    use crate::day03::day03::{Forest, Row};
    use crate::loader::loader::file_to_vec;
    use std::option::NoneError;
    use std::str::FromStr;

    #[test]
    fn test_small() {
        let input: Forest = Forest {
            contents: file_to_vec("inputs/day03small.txt"),
        };
        let result = timed!(input.trees_hit(3, 1), "test_small");
        assert_eq!(result, 7);
    }
    #[test]
    fn test_large() {
        let input = Forest {
            contents: load_large(),
        };
        let result = timed!(input.trees_hit(3, 1), "test_large");
        assert_eq!(result, 276);
    }

    #[test]
    fn test_large_part_2() {
        let input = Forest {
            contents: load_large(),
        };
        let result = timed!(
            load_pairs().iter().fold(1, |acc: u64, (dx, dy)| acc
                * input.trees_hit(*dx, *dy) as u64),
            "test_large_part_2"
        );
        assert_eq!(result, 7812180000);
    }

    fn load_large() -> Vec<Row> {
        file_to_vec::<Row>("inputs/day03.txt")
    }

    fn load_pairs() -> Vec<(usize, usize)> {
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    }
}
