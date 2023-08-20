pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut sides = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();

            sides.sort();

            sides[0] + sides[1] > sides[2]
        })
        .filter(|&x| x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("5 10 25"), 0);
        assert_eq!(part_1("5 10 10"), 1);
    }
}
