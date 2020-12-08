use std::ops::RangeInclusive;
use std::str::FromStr;

enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl FromStr for EyeColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" | "Amb" => Ok(Self::Amb),
            "blu" | "Blu" => Ok(Self::Blu),
            "brn" | "Brn" => Ok(Self::Brn),
            "gry" | "Gry" => Ok(Self::Gry),
            "grn" | "Grn" => Ok(Self::Grn),
            "hzl" | "Hzl" => Ok(Self::Hzl),
            "oth" | "Oth" => Ok(Self::Oth),
            _ => Err(()),
        }
    }
}

enum Unit {
    In,
    Cm,
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "in" | "In" => Ok(Unit::In),
            "cm" | "Cm" => Ok(Unit::Cm),
            _ => Err(()),
        }
    }
}

struct Height {
    value: i32,
    unit: Unit,
}

impl FromStr for Height {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value: Vec<char> = Vec::new();
        let mut unit: Vec<char> = Vec::new();
        for c in s.chars() {
            if c.is_numeric() {
                value.push(c)
            } else {
                unit.push(c)
            }
        }
        Ok(Height {
            value: value.iter().collect::<String>().parse().unwrap(),
            unit: Unit::from_str(unit.iter().collect::<String>().as_str())?,
        })
    }
}

#[derive(Debug)]
enum RequiredField {
    Byr(String),
    Iyr(String),
    Eyr(String),
    Hgt(String),
    Hcl(String),
    Ecl(String),
    Pid(String),
    Cid(String),
}

impl RequiredField {
    fn meets_requirements(&self) -> bool {
        match self {
            RequiredField::Byr(birth_year) => {
                RequiredField::has_len_and_within_range(birth_year, 1920..=2002, 4)
            }

            RequiredField::Iyr(issue_year) => {
                RequiredField::has_len_and_within_range(issue_year, 2010..=2020, 4)
            }

            RequiredField::Eyr(expiration_year) => {
                RequiredField::has_len_and_within_range(expiration_year, 2020..=2030, 4)
            }

            RequiredField::Hgt(height) => match Height::from_str(height) {
                Ok(Height {
                    value,
                    unit: Unit::In,
                }) => (59..=76).contains(&value),
                Ok(Height {
                    value,
                    unit: Unit::Cm,
                }) => (150..=193).contains(&value),
                Err(_) => false,
            },
            RequiredField::Hcl(hair_color_hex) => {
                hair_color_hex.len() == 7 && RequiredField::is_valid_color_hex(hair_color_hex)
            }
            RequiredField::Ecl(eye_color) => eye_color.parse::<EyeColor>().is_ok(),
            RequiredField::Pid(pid) => pid.len() == 9 && pid.chars().all(char::is_numeric),
            RequiredField::Cid(_) => true,
        }
    }

    fn is_valid_color_hex(value: &str) -> bool {
        value
            .chars()
            .into_iter()
            .enumerate()
            .all(|(i, c)| (c.is_numeric() || matches!(c, 'a'..='f')) || (i == 0 && c == '#'))
    }

    fn has_len_and_within_range(
        value: &str,
        acceptable_range: RangeInclusive<i32>,
        acceptable_length: usize,
    ) -> bool {
        value.len() == acceptable_length
            && matches!(value.parse::<i32>().ok(), Some(birth_year) if acceptable_range.contains(&birth_year))
    }
}

impl RequiredField {
    fn is_needed(&self) -> bool {
        !matches!(self, RequiredField::Cid(_))
    }
}

impl FromStr for RequiredField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, rest) = s.split_once(":").unwrap();
        let value = String::from(rest);
        match field {
            "byr" | "Byr" => Ok(RequiredField::Byr(value)),
            "iyr" | "Iyr" => Ok(RequiredField::Iyr(value)),
            "eyr" | "Eyr" => Ok(RequiredField::Eyr(value)),
            "hgt" | "Hgt" => Ok(RequiredField::Hgt(value)),
            "hcl" | "Hcl" => Ok(RequiredField::Hcl(value)),
            "ecl" | "Ecl" => Ok(RequiredField::Ecl(value)),
            "pid" | "Pid" => Ok(RequiredField::Pid(value)),
            "cid" | "Cid" => Ok(RequiredField::Cid(value)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    fields: Vec<RequiredField>,
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s
            .split(' ')
            .map(|x| {
                RequiredField::from_str(x)
                    .unwrap_or_else(|_| panic!("failed to map {}, to RequiredField", x))
            })
            .collect();
        Ok(Passport::new(fields))
    }
}

impl Passport {
    fn new(fields: Vec<RequiredField>) -> Passport {
        Passport { fields }
    }

    pub fn is_valid(&self) -> bool {
        self.fields.iter().filter(|f| f.is_needed()).count() == 7
            && self.fields.iter().all(|f| f.meets_requirements())
    }
}

#[cfg(test)]
mod test {
    use crate::day04::Passport;
    use crate::loader::file_to_vec_by_blank_lines;

    #[test]
    fn test_parse() {
        let _input: Vec<Passport> = file_to_vec_by_blank_lines("inputs/day04small.txt");
    }

    #[test]
    fn test_small() {
        let input: Vec<Passport> = file_to_vec_by_blank_lines("inputs/day04small.txt");
        let result = input.iter().filter(|pass| pass.is_valid()).count();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_large() {
        let input: Vec<Passport> = file_to_vec_by_blank_lines("inputs/day04.txt");
        let result = input.iter().filter(|pass| pass.is_valid()).count();
        assert_eq!(167, result)
    }

    #[test]
    fn test_p2_small_invalid() {
        let input: Vec<Passport> = file_to_vec_by_blank_lines("inputs/day04small1.txt");
        assert!(!input.iter().any(|p| p.is_valid()))
    }

    #[test]
    fn test_p2_small_valid() {
        let input: Vec<Passport> = file_to_vec_by_blank_lines("inputs/day04small2.txt");
        assert!(input.iter().all(|p| p.is_valid()))
    }
}
