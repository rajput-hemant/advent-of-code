// https://eddmann.com/posts/advent-of-code-2015-day-16-aunt-sue/

use std::collections::HashMap;

use regex::Regex;

type Comparator = Box<dyn Fn(i64) -> bool>;

struct Aunt {
    id: i64,
    properties: HashMap<String, i64>,
}

impl Aunt {
    fn new(id: i64) -> Aunt {
        Aunt {
            id,
            properties: HashMap::new(),
        }
    }
}

pub fn part_1(input: &str) -> String {
    let mut readings: HashMap<String, Comparator> = HashMap::new();

    readings.insert("children".to_string(), equal_to(3));
    readings.insert("cats".to_string(), equal_to(7));
    readings.insert("samoyeds".to_string(), equal_to(2));
    readings.insert("pomeranians".to_string(), equal_to(3));
    readings.insert("akitas".to_string(), equal_to(0));
    readings.insert("vizslas".to_string(), equal_to(0));
    readings.insert("goldfish".to_string(), equal_to(5));
    readings.insert("trees".to_string(), equal_to(3));
    readings.insert("cars".to_string(), equal_to(2));
    readings.insert("perfumes".to_string(), equal_to(1));

    find_aunt_with_readings(parse_aunt(input), &readings).to_string()
}

pub fn part_2(input: &str) -> String {
    let mut readings: HashMap<String, Comparator> = HashMap::new();

    readings.insert("children".to_string(), equal_to(3));
    readings.insert("cats".to_string(), greater_than(7));
    readings.insert("samoyeds".to_string(), equal_to(2));
    readings.insert("pomeranians".to_string(), less_than(3));
    readings.insert("akitas".to_string(), equal_to(0));
    readings.insert("vizslas".to_string(), equal_to(0));
    readings.insert("goldfish".to_string(), less_than(5));
    readings.insert("trees".to_string(), greater_than(3));
    readings.insert("cars".to_string(), equal_to(2));
    readings.insert("perfumes".to_string(), equal_to(1));

    find_aunt_with_readings(parse_aunt(input), &readings).to_string()
}

fn parse_aunt(input: &str) -> Vec<Aunt> {
    let re = Regex::new(r"(\w+): (\d+)").unwrap();
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let mut aunt = Aunt::new(i as i64 + 1);
            re.captures_iter(line).for_each(|cap| {
                aunt.properties
                    .insert(cap[1].to_string(), cap[2].parse().unwrap());
            });
            aunt
        })
        .collect()
}

fn find_aunt_with_readings(aunts: Vec<Aunt>, readings: &HashMap<String, Comparator>) -> i64 {
    aunts
        .into_iter()
        .find(|aunt| {
            aunt.properties.iter().all(|(k, v)| {
                // if let Some(comparator) = readings.get(k) {
                //     comparator(*v)
                // } else {
                //     false
                // }
                readings.get(k).unwrap()(*v)
            })
        })
        .unwrap()
        .id
}

fn equal_to(x: i64) -> Comparator {
    Box::new(move |y| x == y)
}

fn less_than(x: i64) -> Comparator {
    Box::new(move |y| y < x)
}

fn greater_than(x: i64) -> Comparator {
    Box::new(move |y| y > x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sue 1: goldfish: 6, trees: 9, akitas: 0
Sue 2: goldfish: 7, trees: 1, akitas: 0
Sue 3: cars: 2, akitas: 0, perfumes: 1";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "3");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), "3");
    }
}
