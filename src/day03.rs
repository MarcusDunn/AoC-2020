#[derive(Copy, Clone, Eq, PartialEq)]
enum Contents {
    Tree,
    Empty,
}

impl Contents {
    fn is_tree(&self) -> bool {
        *self == Contents::Tree
    }
}

impl From<char> for Contents {
    fn from(c: char) -> Self {
        match c {
            '#' => Contents::Tree,
            _ => Contents::Empty,
        }
    }
}

pub struct Forest {
    contents: Vec<Contents>,
    width: usize,
}

impl Forest {
    pub fn new(contents: Vec<String>) -> Forest {
        Forest {
            contents: contents
                .iter()
                .map(|str| {
                    str.chars()
                        .map(Contents::from)
                        .collect::<Vec<Contents>>()
                })
                .flatten()
                .collect(),
            width: contents.first().unwrap().len(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&Contents> {
        self.contents.get(y * self.width + (x % self.width))
    }

    pub fn trees_hit(&self, dx: usize, dy: usize) -> i32 {
        let mut hit_count = 0;
        let mut curr_h = 0;
        let mut curr_w = 0;

        while let Some(square) = self.get(curr_w, curr_h) {
            if square.is_tree() {
                hit_count += 1;
            }
            curr_h += dy;
            curr_w += dx;
        }
        hit_count
    }
}

#[cfg(test)]
mod test {
    use crate::day03::Forest;
    use crate::loader::file_to_vec;

    #[test]
    fn test_small() {
        let input = Forest::new(file_to_vec("inputs/day03small.txt"));
        let result = timed!(input.trees_hit(3, 1), "test_small");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_large() {
        let input = Forest::new(load_large());
        let result = timed!(input.trees_hit(3, 1), "test_large");
        assert_eq!(result, 276);
    }

    #[test]
    fn test_large_part_2() {
        let input = Forest::new(load_large());
        let result = timed!(
            load_pairs().iter().fold(1, |acc: u64, (dx, dy)| {
                acc * input.trees_hit(*dx, *dy) as u64
            }),
            "test_large_part_2"
        );
        assert_eq!(result, 7812180000);
    }

    fn load_large() -> Vec<String> {
        file_to_vec::<String>("inputs/day03.txt")
    }

    fn load_pairs() -> Vec<(usize, usize)> {
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    }
}
