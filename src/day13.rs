use std::convert::TryFrom;

struct Notes {
    earliest: i32,
    bus_ids: Vec<i32>,
}

impl Notes {
    fn find_fastest(&self) -> i32 {
        *self
            .bus_ids
            .iter()
            .filter(|x| x != &&0)
            .min_by_key(|&&x| (self.earliest / x) * x + x - self.earliest)
            .unwrap()
    }

    fn p1(&self) -> i32 {
        let fastest_bus = self.find_fastest();
        let time_waited = (self.earliest / fastest_bus) * fastest_bus + fastest_bus - self.earliest;
        fastest_bus * time_waited
    }

    fn p2(&self) -> usize {
        let mut timestamp: usize = 1;
        let mut wait_time: usize = 1;
        for (bus_num, bus_minutes) in self.bus_ids.iter().map(|a| usize::try_from(*a).unwrap()).enumerate() {
            if bus_minutes == 0 { // Skip this as it's the 'x'
                continue
            }
            loop {
                if (timestamp + bus_num) % bus_minutes == 0 {
                    wait_time *= bus_minutes;
                    break;
                }
                timestamp += wait_time;
            }
        }
        timestamp
    }
}

impl From<Vec<String>> for Notes {
    fn from(a: Vec<String>) -> Self {
        Notes {
            earliest: a.first().unwrap().parse().unwrap_or(0),
            bus_ids: a.last().unwrap().split(',').map(|a| a.parse().unwrap_or(0)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day13::Notes;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_parse() {
        file_to_notes("inputs/day13small.txt");
    }

    #[test]
    fn test_find_fastest_small() {
        let a = file_to_notes("inputs/day13small.txt").find_fastest();
        assert_eq!(59, a);
    }

    #[test]
    fn test_p1_small() {
        let a = file_to_notes("inputs/day13small.txt").p1();
        assert_eq!(295, a);
    }

    #[test]
    fn test_p1() {
        let a = file_to_notes("inputs/day13.txt").p1();
        assert_eq!(6559, a);
    }

    #[test]
    fn test_p2() {
        let a = file_to_notes("inputs/day13.txt").p2();
        assert_eq!(626670513163231, a);
    }


    fn file_to_notes(path: &str) -> Notes {
        Notes::from(
            BufReader::new(File::open(path).unwrap())
                .lines()
                .map(Result::unwrap)
                .collect::<Vec<String>>(),
        )
    }
}
