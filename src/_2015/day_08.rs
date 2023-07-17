pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (mut count, mut chars) = (0, line.chars());

            while let Some(c) = chars.next() {
                match c {
                    '\\' => match chars.next() {
                        Some('x') => {
                            chars.next();
                            chars.next();
                            count += 1;
                        }
                        _ => count += 1,
                    },
                    '"' => {}
                    _ => count += 1,
                }
            }

            line.len() - count
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(r#""""#), "2");
        assert_eq!(part_1(r#""abc""#), "2");
        assert_eq!(part_1(r#""aaa\"aaa""#), "3");
        assert_eq!(part_1(r#""\x27""#), "5");
        assert_eq!(
            part_1(
                r#"""
        "abc"
        "aaa\"aaa"
        "\x27"
        "#
            ),
            "12"
        );
    }
}
