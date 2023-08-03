pub fn part_1(input: &str) -> usize {
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
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (mut count, mut chars) = (2, line.chars()); // 2 for the surrounding quotes

            while let Some(c) = chars.next() {
                match c {
                    '\\' => count += 2,
                    '"' => count += 2,
                    _ => count += 1,
                }
            }

            count - line.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(r#""""#), 2);
        assert_eq!(part_1(r#""abc""#), 2);
        assert_eq!(part_1(r#""aaa\"aaa""#), 3);
        assert_eq!(part_1(r#""\x27""#), 5);
        assert_eq!(
            part_1(
                r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#
            ),
            12
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(r#""""#), 4);
        assert_eq!(part_2(r#""abc""#), 4);
        assert_eq!(part_2(r#""aaa\"aaa""#), 6);
        assert_eq!(part_2(r#""\x27""#), 5);
        assert_eq!(
            part_2(
                r#"""
        "abc"
        "aaa\"aaa"
        "\x27""#
            ),
            19
        );
    }
}
