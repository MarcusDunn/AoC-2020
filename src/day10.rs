use crate::loader::file_to_vec;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

struct DaisyChain {
    adaptors: Vec<usize>,
}

impl DaisyChain {
    fn contains(&self, index: &usize) -> bool {
        self.adaptors.contains(index)
    }
}

impl DaisyChain {
    fn new(mut adaptors: Vec<usize>) -> DaisyChain {
        adaptors.insert(0, 0);
        adaptors.push(adaptors.iter().max().unwrap() + 3);
        adaptors.sort_unstable();
        DaisyChain { adaptors }
    }

    pub fn part_one(&self) -> usize {
        let mut count = (0, 0, 0);

        let mut curr_adapter = 0;
        for adaptor in self.adaptors.iter() {
            let diff = adaptor - curr_adapter;
            if diff == 1 {
                count.0 += 1;
            } else if diff == 3 {
                count.2 += 1;
            }
            curr_adapter = *adaptor;
        }

        count.0 * count.2
    }

    pub fn part_two(&self) -> usize {
        let &final_number = self.adaptors.last().unwrap();
        let mut hits: HashMap<usize, usize> = HashMap::new();

        hits.insert(self[0], 1);

        for i in self.adaptors.iter() {
            let curr_hits = *hits.get(&i).unwrap();

            for step in 1..4 {
                let i_next = i + step;
                if self.contains(&i_next) {
                    *hits.entry(i_next).or_insert(0) += curr_hits;
                }
            }
        }

        return *hits.get(&final_number).unwrap();
    }
}

impl Index<usize> for DaisyChain {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.adaptors[index]
    }
}

impl IndexMut<usize> for DaisyChain {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.adaptors[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        DaisyChain::new(file_to_vec("inputs/day10.txt"));
    }

    #[test]
    fn test_small() {
        let chain = DaisyChain::new(file_to_vec("inputs/day10small.txt"));
        assert_eq!(35, chain.part_one());
    }

    #[test]
    fn test_medium() {
        let chain = DaisyChain::new(file_to_vec("inputs/day10medium.txt"));
        assert_eq!(220, chain.part_one());
    }

    #[test]
    fn test_p1() {
        let chain = DaisyChain::new(file_to_vec("inputs/day10.txt"));
        assert_eq!(2482, chain.part_one());
    }

    #[test]
    fn test_p2() {
        let chain = DaisyChain::new(file_to_vec("inputs/day10.txt"));
        assert_eq!(96717311574016, chain.part_two());
    }
}
