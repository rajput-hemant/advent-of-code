pub fn part_1(input: &str) -> String {
    input
        .chars()
        .fold((0, String::new()), |(mut sum, mut num), c| {
            // check if the character is a digit or a minus sign.
            if c.is_digit(10) || c == '-' {
                // if it's a digit or a minus sign, add it to the current number
                num.push(c);
            } else {
                // if it's not a digit or a minus sign, then we've reached the end of a number,
                // parse the number and add it to the sum.
                if !num.is_empty() {
                    sum += num.parse::<i32>().unwrap();
                    // clear the num string to start forming a new number.
                    num.clear();
                }
            }
            (sum, num)
        })
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("[1,2,3]"), "6");
        assert_eq!(part_1(r#"{"a":2,"b":4}"#), "6");
        assert_eq!(part_1("[[[3]]]"), "3");
        assert_eq!(part_1(r#"{"a":{"b":4},"c":-1}"#), "3");
        assert_eq!(part_1(r#"{"a":[-1,1]}"#), "0");
        assert_eq!(part_1(r#"[-1,{"a":1}]"#), "0");
        assert_eq!(part_1("[]"), "0");
        assert_eq!(part_1("{}"), "0");
    }
}
