pub fn part_1(input: &str) -> String {
    (0..40)
        .fold(String::from(input), |acc, _| look_and_say(&acc))
        .len()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    (0..50)
        .fold(String::from(input), |acc, _| look_and_say(&acc))
        .len()
        .to_string()
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
        assert_eq!(part_1("1"), "82350");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("1"), "1166642");
    }

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
