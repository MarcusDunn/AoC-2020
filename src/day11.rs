use core::fmt;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Spot {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Debug for Spot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Spot::Floor => write!(f, "."),
            Spot::Empty => write!(f, "L"),
            Spot::Occupied => write!(f, "#"),
        }
    }
}

impl Spot {
    fn outcome(seats: [[Spot; 3]; 3]) -> Spot {
        let focus = seats[1][1];
        if focus == Spot::Floor {
            Spot::Floor
        } else if Spot::Empty == focus && Spot::empty_surroundings(&seats) {
            Spot::Occupied
        } else if Spot::Occupied == focus && Spot::has_four_or_more_surrounding(seats) {
            Spot::Empty
        } else {
            focus
        }
    }

    fn outcome_v2(seats: [[Spot; 3]; 3]) -> Spot {
        let focus = seats[1][1];
        if focus == Spot::Floor {
            Spot::Floor
        } else if Spot::Empty == focus && Spot::empty_surroundings(&seats) {
            Spot::Occupied
        } else if Spot::Occupied == focus && Spot::has_five_or_more_surrounding(seats) {
            Spot::Empty
        } else {
            focus
        }
    }

    fn has_five_or_more_surrounding(seats: [[Spot; 3]; 3]) -> bool {
        seats
            .iter()
            .flatten()
            .enumerate()
            .filter(|(i, _)| i.ne(&4))
            .filter(|(_, s)| Spot::Occupied.eq(s))
            .count()
            >= 5
    }

    fn has_four_or_more_surrounding(seats: [[Spot; 3]; 3]) -> bool {
        seats
            .iter()
            .flatten()
            .enumerate()
            .filter(|(i, _)| i.ne(&4))
            .filter(|(_, s)| Spot::Occupied.eq(s))
            .count()
            >= 4
    }

    fn empty_surroundings(seats: &[[Spot; 3]; 3]) -> bool {
        seats
            .iter()
            .flatten()
            .all(|s| matches!(s, Spot::Empty | Spot::Floor))
    }
}

impl From<char> for Spot {
    fn from(c: char) -> Self {
        match c {
            '#' => Spot::Occupied,
            'L' => Spot::Empty,
            '.' => Spot::Floor,
            _ => unreachable!("all spots should be in the set {'#', 'L', '.'}"),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
pub struct WaitingArea {
    spots: Vec<Vec<Spot>>,
}

impl fmt::Display for WaitingArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for row in &self.spots {
            out.push_str(format!("{:?} \n", row).replace(", ", "").as_str());
        }
        write!(f, "{}", out.as_str().replace("[", "").replace("]", ""))
    }
}

impl From<Vec<Vec<char>>> for WaitingArea {
    fn from(mut spots: Vec<Vec<char>>) -> Self {
        WaitingArea::pad_with_empty(&mut spots);
        Self {
            spots: spots
                .iter()
                .map(|a| a.iter().map(|&c| Spot::from(c)).collect())
                .collect(),
        }
    }
}

impl WaitingArea {
    fn step(&mut self) -> WaitingArea {
        let mut clone = self.clone();
        for x in 1..self.spots.len() - 1 {
            for y in 1..self.spots[0].len() - 1 {
                let surroundings = self.create_3x3(x, y);
                clone.spots[x][y] = Spot::outcome(surroundings);
            }
        }
        clone
    }

    fn find_stable_state_p1(&self) -> usize {
        let mut curr = self.clone();
        let mut next = curr.step();
        while curr != next {
            next = next.step();
            curr = curr.step();
        }
        curr.spots
            .iter()
            .flatten()
            .filter(|s| matches!(s, Spot::Occupied))
            .count()
    }

    fn find_stable_state_p2(&self) -> usize {
        let mut curr = self.clone();
        let mut next = curr.step_rtx();
        while curr != next {
            next = next.step_rtx();
            curr = curr.step_rtx();
        }
        curr.spots
            .iter()
            .flatten()
            .filter(|s| matches!(s, Spot::Occupied))
            .count()
    }

    fn pad_with_empty(spots: &mut Vec<Vec<char>>) {
        spots.push(vec!['.'; spots[0].len()]);
        spots.insert(0, vec!['.'; spots[0].len()]);
        for row in spots {
            row.insert(0, '.');
            row.push('.')
        }
    }

    fn create_3x3_rtx(&self, x: usize, y: usize) -> [[Spot; 3]; 3] {
        let ix = x as i32;
        let iy = y as i32;
        let north = self.find_in_dir((ix, iy), (1, 0));
        let south = self.find_in_dir((ix, iy), (-1, 0));
        let east = self.find_in_dir((ix, iy), (0, -1));
        let west = self.find_in_dir((ix, iy), (0, 1));
        let nw = self.find_in_dir((ix, iy), (1, 1));
        let ne = self.find_in_dir((ix, iy), (1, -1));
        let sw = self.find_in_dir((ix, iy), (-1, 1));
        let se = self.find_in_dir((ix, iy), (-1, -1));
        let focus = self.spots[x][y];
        [[nw, north, ne], [west, focus, east], [sw, south, se]]
    }

    fn create_3x3(&self, x: usize, y: usize) -> [[Spot; 3]; 3] {
        [
            [
                self.spots[x - 1][y - 1],
                self.spots[x - 1][y],
                self.spots[x - 1][y + 1],
            ],
            [self.spots[x][y - 1], self.spots[x][y], self.spots[x][y + 1]],
            [
                self.spots[x + 1][y - 1],
                self.spots[x + 1][y],
                self.spots[x + 1][y + 1],
            ],
        ]
    }

    fn find_in_dir(&self, location: (i32, i32), direction: (i32, i32)) -> Spot {
        let mut i = 1;
        loop {
            let ray: (usize, usize) = (
                (location.0 + (direction.0 * i)) as usize,
                (location.1 + (direction.1 * i)) as usize,
            );
            if let Some(spot) = self.spots.get(ray.0).unwrap_or(&Vec::new()).get(ray.1) {
                if !(*spot == Spot::Floor) {
                    return *spot;
                } else {
                    i += 1
                }
            } else {
                return Spot::Empty;
            }
        }
    }
    fn step_rtx(&self) -> WaitingArea {
        let mut clone = self.clone();
        for x in 1..self.spots.len() - 1 {
            for y in 1..self.spots[0].len() - 1 {
                let surroundings = self.create_3x3_rtx(x, y);
                clone.spots[x][y] = Spot::outcome_v2(surroundings);
            }
        }
        clone
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::WaitingArea;
    use crate::loader::file_to_vec;

    #[test]
    fn test_parse() {
        fromfile("inputs/day11small.txt");
    }

    #[test]
    fn test_step() {
        let mut a = fromfile("inputs/day11small.txt");
        let mut next = a.step();
        let mut curr = a.clone();
        while curr != next {
            next = next.step();
            curr = curr.step();
        }
    }

    #[test]
    fn test_count_steady_state() {
        let a = fromfile("inputs/day11small.txt");
        println!("{}", a.find_stable_state_p1());
    }

    #[test]
    fn test_p1() {
        let a = fromfile("inputs/day11.txt");
        println!("{}", a.find_stable_state_p1());
    }

    #[test]
    fn test_p2() {
        let a = fromfile("inputs/day11.txt");
        println!("{}", a.find_stable_state_p2());
    }

    fn fromfile(path: &str) -> WaitingArea {
        WaitingArea::from(
            file_to_vec::<String>(path)
                .iter()
                .map(|s| s.chars().collect())
                .collect::<Vec<Vec<char>>>(),
        )
    }
}
