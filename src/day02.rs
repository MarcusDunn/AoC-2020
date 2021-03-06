use std::option::NoneError;
use std::str::FromStr;

#[derive(Debug)]
pub struct PolicyPasswordPair {
    letter: char,
    first_char: i32,
    second_char: i32,
    password: String,
}

impl PolicyPasswordPair {
    pub fn new(
        letter: char,
        first_char: i32,
        second_char: i32,
        password: String,
    ) -> PolicyPasswordPair {
        PolicyPasswordPair {
            letter,
            first_char,
            second_char,
            password,
        }
    }

    pub fn is_following_policy(&self) -> bool {
        if let Some(first) = self.password.chars().nth((self.first_char - 1) as usize) {
            if let Some(second) = self.password.chars().nth((self.second_char - 1) as usize) {
                let is_letter_at_first = first == self.letter;
                let is_letter_at_second = second == self.letter;
                return (is_letter_at_first || is_letter_at_second)
                    && !(is_letter_at_first && is_letter_at_second);
            }
        }
        false
    }
}

impl FromStr for PolicyPasswordPair {
    type Err = NoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, rest) = s.rsplit_once("-")?;
        let (second, rest) = rest.split_once(" ")?;
        let (letter, password) = rest.split_once(":")?;
        Ok(PolicyPasswordPair::new(
            letter.parse().ok()?,
            first.parse().ok()?,
            second.parse().ok()?,
            password.trim().parse().ok()?,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::day02::PolicyPasswordPair;
    use crate::loader::file_to_vec;

    #[test]
    fn test_small_input() {
        let num_following = timed!(
            get_small_input()
                .iter()
                .filter(|ppp| ppp.is_following_policy())
                .count(),
            "test_small_input"
        ) as i32;
        assert_eq!(num_following, 1)
    }

    #[test]
    fn test_large_input() {
        let num_following = timed!(
            get_large_input()
                .iter()
                .filter(|ppp| ppp.is_following_policy())
                .count(),
            "test_large_input"
        ) as i32;
        assert_eq!(num_following, 562);
    }

    fn get_large_input() -> Vec<PolicyPasswordPair> {
        file_to_vec::<PolicyPasswordPair>("inputs/day02.txt")
    }

    fn get_small_input() -> Vec<PolicyPasswordPair> {
        vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}
