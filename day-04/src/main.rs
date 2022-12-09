use std::{fs, ops::RangeInclusive};

#[derive(Debug, Clone)]
struct Elf(RangeInclusive<i32>);

impl Elf {
    fn new(s: &str) -> Self {
        let start_stop: Vec<i32> = s.split('-').map(|s| s.parse::<i32>().unwrap()).collect();

        Self(start_stop[0]..=start_stop[1])
    }

    fn fully_contains_another(&mut self, other: &mut Self) -> bool {
        self.0.contains(&other.0.start()) && self.0.contains(&other.0.end())
            || other.0.contains(&self.0.start()) && other.0.contains(&self.0.end())
    }

    fn partial_contains_another(&mut self, other: &mut Self) -> bool {
        self.0.contains(&other.0.start())
            || self.0.contains(&other.0.end())
            || other.0.contains(&self.0.start())
            || other.0.contains(&self.0.end())
    }
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    println!("Part 1: {}", solve_part_1(&data));
    println!("Part 2: {}", solve_part_2(&data));
}

fn solve_part_1(data: &str) -> String {
    let intersections = data
        .lines()
        .filter(|line| {
            let parts = line.split(',').collect::<Vec<_>>();
            let mut first = Elf::new(&parts[0]);
            let mut second = Elf::new(&parts[1]);

            let intersects = first.fully_contains_another(&mut second);

            intersects
        })
        .count();

    intersections.to_string()
}

fn solve_part_2(data: &str) -> String {
    let intersections = data
        .lines()
        .filter(|line| {
            let parts = line.split(',').collect::<Vec<_>>();
            let mut first = Elf::new(&parts[0]);
            let mut second = Elf::new(&parts[1]);

            let intersects = first.partial_contains_another(&mut second);

            intersects
        })
        .count();

    intersections.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(solve_part_1(data), "2");
    }

    #[test]
    fn test_part_2() {
        let data = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(solve_part_2(data), "4");
    }
}
