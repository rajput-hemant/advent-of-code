use std::collections::HashMap;

use regex::Regex;

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
    let mut readings: HashMap<String, i64> = HashMap::new();

    readings.insert("children".to_string(), 3);
    readings.insert("cats".to_string(), 7);
    readings.insert("samoyeds".to_string(), 2);
    readings.insert("pomeranians".to_string(), 3);
    readings.insert("akitas".to_string(), 0);
    readings.insert("vizslas".to_string(), 0);
    readings.insert("goldfish".to_string(), 5);
    readings.insert("trees".to_string(), 3);
    readings.insert("cars".to_string(), 2);
    readings.insert("perfumes".to_string(), 1);

    find_aunt_with_readings(input, &readings).to_string()
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

fn find_aunt_with_readings(input: &str, readings: &HashMap<String, i64>) -> i64 {
    parse_aunt(input)
        .into_iter()
        .find(|aunt| {
            aunt.properties
                .iter()
                .all(|(k, v)| readings.get(k).unwrap() == v)
        })
        .unwrap()
        .id
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
}
