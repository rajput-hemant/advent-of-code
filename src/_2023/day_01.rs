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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 142);
    }
}
