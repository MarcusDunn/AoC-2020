use std::collections::HashMap;

fn run_to(past: &mut Vec<i32>, times: usize) {
    for _ in past.len()..times {
        step(past)
    }
}

fn run_v2(starting: &mut HashMap<i32, usize>, steps: usize) -> usize {
    let len = starting.len();
    let mut most_recent = starting
        .into_iter()
        .max_by_key(|(_, v)| **v)
        .unwrap()
        .0
        .clone();

    for i in len..steps {
        most_recent = step_v2(starting, most_recent, i + 1);
    }
    most_recent as usize
}

fn step_v2(map: &mut HashMap<i32, usize>, last_spoken: i32, curr_step: usize) -> i32 {
    let mut word_said = None;
    if let Some(last_time) = map.get(&last_spoken) {
        word_said = Some(curr_step - 1 - last_time);
    } else {
        word_said = Some(0);
    }
    map.insert(last_spoken as i32, curr_step - 1);
    word_said.unwrap() as i32
}

fn step(past: &mut Vec<i32>) {
    let last = past.last().unwrap();
    if let Some((steps_ago, _)) = past
        .iter()
        .rev()
        .enumerate()
        .find(|(i, x)| &last == x && i != &0)
    {
        past.push((steps_ago) as i32);
    } else {
        past.push(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::day15::{run_to, run_v2, step, step_v2};
    use std::collections::HashMap;

    #[test]
    fn test_parse() {
        assert_eq!(vec![0, 3, 6], parse("0,3,6"));
    }

    #[test]
    fn test_step() {
        let mut a = parse("0,3,6");
        step(&mut a);
        assert_eq!(a.last(), Some(&0));
    }

    #[test]
    fn test_run_to() {
        let mut a = parse("0,3,6");
        run_to(&mut a, 10);
        assert_eq!(vec![0, 3, 6, 0, 3, 3, 1, 0, 4, 0], a);
    }

    #[test]
    fn test_run_2020_1() {
        let mut a = parse("1,3,2");
        run_to(&mut a, 2020);
        assert_eq!(Some(&1), a.last());
    }

    #[test]
    fn test_run_2020_2() {
        let mut a = parse("3,1,2");
        run_to(&mut a, 2020);
        assert_eq!(Some(&1836), a.last());
    }

    #[test]
    fn test_p1() {
        let mut a = parse("16,12,1,0,15,7,11");
        run_to(&mut a, 2020);
        assert_eq!(Some(&403), a.last());
    }

    #[test]
    fn test_parse_v2() {
        let a = parse_map("16,12,1,0,15,7,11");
        println!("{:?}", a);
    }

    #[test]
    fn test_step_v2() {
        let mut a = parse_map("0,3");
        let mut next = step_v2(&mut a, 6, 4);
        assert_eq!(0, next);
        next = step_v2(&mut a, next as i32, 5);
        assert_eq!(3, next);
        next = step_v2(&mut a, next as i32, 6);
        assert_eq!(3, next);
        next = step_v2(&mut a, next as i32, 7);
        assert_eq!(1, next);
        next = step_v2(&mut a, next as i32, 8);
        assert_eq!(0, next);
        next = step_v2(&mut a, next as i32, 9);
        assert_eq!(4, next);
        next = step_v2(&mut a, next as i32, 10);
        assert_eq!(0, next);
    }

    #[test]
    fn test_run_v2() {
        let mut a = parse_map("0,3,6");
        run_v2(&mut a, 10);
    }

    #[test]
    fn test_p2_1() {
        let mut a = parse_map("1,3,2");
        let last = run_v2(&mut a, 2020);
        assert_eq!(1, last);
    }

    #[test]
    fn test_p2() {
        timed!(
            {
                let mut a = parse_map("16,12,1,0,15,7,11");
                let last = run_v2(&mut a, 30000000);
                assert_eq!(6823, last);
            },
            "test_p2"
        );
    }

    fn parse_map(input: &str) -> HashMap<i32, usize> {
        let mut ret = HashMap::new();
        for (i, x) in input
            .split(',')
            .map(|num| num.parse::<i32>().unwrap())
            .enumerate()
        {
            ret.insert(x, i + 1);
        }
        ret
    }

    fn parse(input: &str) -> Vec<i32> {
        input.split(',').map(|c| c.parse().unwrap()).collect()
    }
}
