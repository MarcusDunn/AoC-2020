pub mod loader {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};
    use std::ops::Add;
    use std::str::FromStr;

    pub fn file_to_vec<T: FromStr>(path: &str) -> Vec<T> {
        BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(|line| line.ok()?.trim().parse().ok())
            .collect()
    }

    pub fn file_to_vec_by_blank_lines<T: FromStr>(path: &str) -> Vec<T> {
        let lines: Vec<String> = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|s| s.unwrap())
            .collect();

        let mut ret: Vec<String> = Vec::with_capacity(lines.len());
        let mut counter = 0;


        for line in lines {
            if line.is_empty() {
                counter += 1
            } else if let Some(x) = ret.get(counter) {
                ret[counter] = x.clone().add(" ").add(line.as_str());
            } else {
                ret.push(line);
            }
        }
        ret.iter()
            .map(|s| s.as_str().parse().ok().unwrap())
            .collect()
    }
}
