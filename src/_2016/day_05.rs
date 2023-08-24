pub fn part_1(input: &str) -> String {
    (0..)
        .map(|i| format!("{:x}", md5::compute(format!("{}{}", input, i))))
        .filter(|hash| hash.starts_with("00000"))
        .take(8)
        .map(|hash| hash.chars().nth(5).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("abc"), "18f47a30");
    }
}
