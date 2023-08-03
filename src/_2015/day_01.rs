pub fn part_1(input: &str) -> isize {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => unreachable!(),
    })
}

pub fn part_2(input: &str) -> usize {
    let mut pos = 0;

    input
        .chars()
        .enumerate()
        .find_map(|(i, c)| {
            pos += match c {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            };

            if pos == -1 {
                Some(i + 1)
            } else {
                None
            }
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")))"), -3);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
    }
}
