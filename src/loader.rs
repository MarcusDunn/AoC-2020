pub mod loader {
    use std::str::FromStr;
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    use std::option::NoneError;

    pub fn file_to_vec<T: FromStr>(path: &str) -> Vec<T> {
        BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(|line| line.ok()?.trim().parse().ok())
            .collect()
    }

}