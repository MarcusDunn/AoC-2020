mod day04 {
    use crate::day04::day04::RequiredField::{
        BirthYear, CountryID, ExpirationYear, EyeColour, HairColour, Hght, IssueYear, PassportID,
    };
    use crate::day04::day04::Unit::{Cm, In};
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
            match s.to_ascii_lowercase().as_str() {
                "amb" => Ok(Self::Amb),
                "blu" => Ok(Self::Blu),
                "brn" => Ok(Self::Brn),
                "gry" => Ok(Self::Gry),
                "grn" => Ok(Self::Grn),
                "hzl" => Ok(Self::Hzl),
                "oth" => Ok(Self::Oth),
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
                "in" => Ok(In),
                "cm" => Ok(Cm),
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
            for x in s.chars() {
                if x.is_numeric() {
                    value.push(x)
                } else {
                    unit.push(x)
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
        BirthYear(String),
        IssueYear(String),
        ExpirationYear(String),
        Hght(String),
        HairColour(String),
        EyeColour(String),
        PassportID(String),
        CountryID(String),
    }

    impl RequiredField {
        fn meets_requirements(&self) -> bool {
            match self {
                BirthYear(value) => {
                    let by: Option<i32> = value.parse().ok();
                    by.is_some() && value.len() == 4 && by.unwrap() <= 2002 && by.unwrap() >= 1920
                }
                IssueYear(value) => {
                    let iy: Option<i32> = value.parse().ok();
                    iy.is_some() && value.len() == 4 && iy.unwrap() >= 2010 && iy.unwrap() <= 2020
                }
                ExpirationYear(value) => {
                    let ey: Option<i32> = value.parse().ok();
                    ey.is_some() && value.len() == 4 && ey.unwrap() >= 2020 && ey.unwrap() <= 2030
                }
                Hght(value) => {
                    if let Ok(h) = Height::from_str(value) {
                        match h.unit {
                            In => h.value >= 59 && h.value <= 76,
                            Cm => h.value >= 150 && h.value <= 193,
                        }
                    } else {
                        false
                    }
                }
                HairColour(value) => {
                    let a = value.chars().into_iter().enumerate().all(|(i, c)| {
                        if i == 0 {
                            c == '#'
                        } else {
                            c.is_numeric() || match c {
                                'a'..='f' => true,
                                _ => false,
                            }
                        }
                    });
                    a && value.len() == 7
                },
                EyeColour(value) => value.parse::<EyeColor>().is_ok(),
                PassportID(value) => value.len() == 9 && value.chars().all(|c| c.is_numeric()),
                CountryID(_) => true,
            }
        }
    }

    impl RequiredField {
        fn is_needed(&self) -> bool {
            match self {
                CountryID(_) => false,
                _ => true,
            }
        }
    }

    impl FromStr for RequiredField {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (field, rest) = s.split_once(":").unwrap();
            let value = String::from(rest);
            match field {
                "byr" => Ok(BirthYear(value)),
                "iyr" => Ok(IssueYear(value)),
                "eyr" => Ok(ExpirationYear(value)),
                "hgt" => Ok(Hght(value)),
                "hcl" => Ok(HairColour(value)),
                "ecl" => Ok(EyeColour(value)),
                "pid" => Ok(PassportID(value)),
                "cid" => Ok(CountryID(value)),
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
                .split(" ")
                .map(|x| RequiredField::from_str(x).unwrap())
                .collect();
            Ok(Passport::new(fields))
        }
    }

    impl Passport {
        fn new(fields: Vec<RequiredField>) -> Passport {
            Passport { fields }
        }

        pub fn is_valid(&self) -> bool {
            let cond1 = self.fields.iter().filter(|f| f.is_needed()).count() == 7;
            let cond2 = self.fields.iter().all(|f| f.meets_requirements());
            cond1 && cond2
        }
    }
}

#[cfg(test)]
mod day04test {
    use crate::day04::day04::Passport;
    use crate::loader::loader::file_to_vec_by_blank_lines;

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
        println!("{}", result)
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
