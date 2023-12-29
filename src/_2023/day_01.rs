pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let nums: Vec<_> = l.chars().filter(|c| c.is_numeric()).collect();
            format!("{}{}", nums[0], nums[nums.len() - 1])
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let digits_map = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    input
        .lines()
        .map(|l| {
            let (mut first, mut last) = (String::default(), String::default());

            let mut str = String::default();

            l.chars().for_each(|c| {
                let mut digit = String::default();

                if c.is_numeric() {
                    digit.push(c);
                } else {
                    str.push(c);

                    digits_map.iter().for_each(|(k, v)| {
                        if str.ends_with(k) {
                            digit = v.to_string();
                        }
                    });
                }

                if !digit.is_empty() {
                    if first.is_empty() {
                        first = digit;
                    } else {
                        last = digit;
                    }
                }
            });

            if last.is_empty() {
                last = first.clone();
            }

            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_1() {
        const INPUT: &str = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
        assert_eq!(part_1(INPUT), 142);
    }

    const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 281);
    }
}
