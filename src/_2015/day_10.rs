pub fn part_1(input: &str) -> usize {
    (0..40)
        .fold(String::from(input), |acc, _| look_and_say(&acc))
        .len()
}

pub fn part_2(input: &str) -> usize {
    (0..50)
        .fold(String::from(input), |acc, _| look_and_say(&acc))
        .len()
}

fn look_and_say(input: &str) -> String {
    let (mut result, mut chars) = (String::new(), input.chars());
    let (mut current_char, mut current_count) = (chars.next().unwrap(), 1);

    chars.for_each(|c| {
        if c == current_char {
            current_count += 1;
        } else {
            result.push_str(&format!("{current_count}{current_char}"));
            current_char = c;
            current_count = 1;
        }
    });

    result.push_str(&format!("{current_count}{current_char}"));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("1"), 82350);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("1"), 1166642);
    }
}
