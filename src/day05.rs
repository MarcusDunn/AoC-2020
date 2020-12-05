mod day05 {
    use std::str::FromStr;

    pub struct Ticket {
        row: i32,
        col: i32,
    }

    #[derive(Debug)]
    struct HighLowBounds {
        top: i32,
        bot: i32,
    }

    impl HighLowBounds {
        fn new(low: i32, high: i32) -> HighLowBounds {
            HighLowBounds { top: high, bot: low }
        }

        fn take_lower_half(&mut self) {
            self.top -= (self.top - self.bot) / 2;
        }

        fn take_upper_half(&mut self) {
            self.bot += (self.top - self.bot) / 2;
        }

        fn get_result(&self) -> i32 {
            assert_eq!(self.top - self.bot, 1, "{}", format!("top {}, bot {}", self.top, self.bot));
            self.bot
        }
    }

    impl Ticket {
        fn new(seat: Vec<char>) -> Ticket {
            let (row, col) = Ticket::get_seat_number(&seat);
            Ticket {
                row,
                col,
            }
        }

        pub fn get_seat_id(&self) -> i32 {
            self.row * 8 + self.col
        }

        fn get_seat_number(seat: &Vec<char>) -> (i32, i32) {
            let mut row = HighLowBounds::new(0, 128);
            let mut col = HighLowBounds::new(0, 8);

            for char in seat {
                match char {
                    'F' | 'f' => row.take_lower_half(),
                    'B' | 'b' => row.take_upper_half(),
                    'R' | 'r' => col.take_upper_half(),
                    'L' | 'l' => col.take_lower_half(),
                    c @ _ => unreachable!(format!("all chars should be in set {{F, B, R, L}} , instead got {}", c)),
                }
            }
            (row.get_result(), col.get_result())
        }
    }

    impl FromStr for Ticket {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Ticket::new(s.chars().collect()))
        }
    }
}

#[cfg(test)]
mod day05test {
    use crate::day05::day05::Ticket;
    use crate::loader::loader::file_to_vec;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        Ticket::from_str("FBFBBFFRLR").unwrap();
    }

    #[test]
    fn test_get_seat() {
        let t = Ticket::from_str("FBFBBFFRLR").unwrap();
        assert_eq!(t.get_seat_id(), 357)
    }

    #[test]
    fn test_parse() {
        let input: Vec<Ticket> = file_to_vec("inputs/day05.txt");
    }

    #[test]
    fn test_large() {
        let input: Vec<Ticket> = file_to_vec("inputs/day05.txt");
        let max = input.iter().max_by_key(|t| t.get_seat_id()).unwrap();
        assert_eq!(max.get_seat_id(), 842);
    }

    #[test]
    fn test_find_my_seat() {
        let mut input: Vec<Ticket> = file_to_vec("inputs/day05.txt");
        input.sort_by_key(|t| t.get_seat_id());
        let seat_in_front = input.iter().enumerate().find(|(i, t)| input.get(i+1).unwrap().get_seat_id() == t.get_seat_id() + 2).unwrap().1;
        println!("{}", seat_in_front.get_seat_id() + 1);
    }
}
