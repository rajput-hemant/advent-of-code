pub fn part_1(input: &str) -> String {
    input
        .chars()
        .fold(0, |acc, c| match c {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => unreachable!(),
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("(())"), "0");
        assert_eq!(part_1("((("), "3");
        assert_eq!(part_1("(()(()("), "3");
        assert_eq!(part_1("))((((("), "3");
        assert_eq!(part_1("())"), "-1");
        assert_eq!(part_1("))("), "-1");
        assert_eq!(part_1(")))"), "-3");
        assert_eq!(part_1(")())())"), "-3");
    }
}
