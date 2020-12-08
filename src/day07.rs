use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq)]
pub struct BagType {
    adj: String,
    color: String,
}

impl FromStr for BagType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (adj, color) = s.split_once(" ").unwrap_or_else(|| {
            panic!(
                "attempted to split_once with a \" \" and failed on \"{}\"",
                s
            )
        });
        let adj = String::from(adj);
        let color = String::from(color.split(' ').next().unwrap_or_else(|| {
            panic!(
                "expected at least one value when splitting \"{}\" by \" \" but found none",
                color
            )
        }));
        Ok(BagType { adj, color })
    }
}

pub struct Bag {
    bag_type: BagType,
    inner: Vec<(BagType, i32)>,
}

impl FromStr for Bag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (bag_desc, contents) = s.split_once("bags contain").unwrap_or_else(|| {
            panic!(
                "all lines should contain \"bags contain\", instead got \"{}\"",
                s
            )
        });
        let bag_type = BagType::from_str(bag_desc)
            .unwrap_or_else(|_| panic!("expected to be able to turn {} into a BagType", bag_desc));
        let inner = contents
            .split(|c| c == ',')
            .filter(|s| s.trim() != "no other bags.")
            .map(|s| {
                let (num, bag_type) = s
                    .trim_start()
                    .split_once(" ")
                    .unwrap_or_else(|| panic!("splitting at \" \" failed on string: \"{}\"", s));
                (
                    BagType::from_str(bag_type).unwrap_or_else(|_| {
                        panic!(
                            "attempted (and failed!) to parse BagType from \"{}\"",
                            bag_type
                        )
                    }),
                    num.parse().unwrap_or_else(|_| {
                        panic!("attempted (and failed!) to parse a i32 from \"{}\"", num)
                    }),
                )
            })
            .collect();
        Ok(Bag { bag_type, inner })
    }
}

pub struct HashMatrix<'a> {
    matrix: Vec<(&'a BagType, &'a BagType, i32)>,
}

impl<'a> HashMatrix<'a> {
    pub fn new() -> HashMatrix<'a> {
        HashMatrix { matrix: Vec::new() }
    }

    pub fn add(&mut self, bag: &'a Bag) {
        let Bag { bag_type, inner } = bag;
        for (inner_bag, num) in inner {
            self.matrix.push((&bag_type, &inner_bag, *num));
        }
    }

    pub fn bags_that_contain(&self, bag_type: &BagType) -> HashSet<&BagType> {
        self.matrix
            .iter()
            .filter(|(_, contents, _)| bag_type.eq(contents))
            .fold(HashSet::new(), |mut acc, (bag, ..)| {
                acc.insert(bag);
                acc.union(&self.bags_that_contain(bag)).copied().collect()
            })
    }

    pub fn contents(&self, bag_type: &BagType) -> i32 {
        self.matrix
            .iter()
            .filter(|(bag, ..)| bag_type.eq(bag))
            .fold(0, |acc, (_, contents, num)| {
                acc + num * (self.contents(contents) + 1)
            })
    }
}

#[cfg(test)]
mod test {
    use crate::day07::{Bag, HashMatrix};
    use crate::loader::file_to_vec;

    #[test]
    fn test_parse() {
        file_to_vec::<Bag>("inputs/day07small.txt");
    }

    #[test]
    fn test_small() {
        let input = file_to_vec::<Bag>("inputs/day07small.txt");
        let mut hash_matrix = HashMatrix::new();
        let a = timed!(
            {
                for bag in input.iter() {
                    hash_matrix.add(&bag)
                }
                hash_matrix
                    .bags_that_contain(&"shiny gold".parse().unwrap())
                    .len()
            },
            "test_small"
        );
        assert_eq!(4, a);
    }

    #[test]
    fn test_large() {
        let input = file_to_vec::<Bag>("inputs/day07.txt");
        let mut hash_matrix = HashMatrix::new();
        let a = timed!(
            {
                for bag in input.iter() {
                    hash_matrix.add(&bag)
                }
                hash_matrix
                    .bags_that_contain(&"shiny gold".parse().unwrap())
                    .len()
            },
            "test_large"
        );
        assert_eq!(124, a);
    }

    #[test]
    fn test_small_p2() {
        let input = file_to_vec::<Bag>("inputs/day07small2.txt");
        let mut hash_matrix = HashMatrix::new();
        let a = timed!(
            {
                for bag in input.iter() {
                    hash_matrix.add(&bag)
                }
                hash_matrix.contents(&"shiny gold".parse().unwrap())
            },
            "test_small_p2"
        );

        assert_eq!(126, a);
    }

    #[test]
    fn test_large_p2() {
        let input = file_to_vec::<Bag>("inputs/day07.txt");
        let mut hash_matrix = HashMatrix::new();
        let a = timed!(
            {
                for bag in input.iter() {
                    hash_matrix.add(&bag)
                }
                hash_matrix.contents(&"shiny gold".parse().unwrap())
            },
            "test_large_p2"
        );
        assert_eq!(34862, a);
    }
}
