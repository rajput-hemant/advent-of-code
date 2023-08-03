pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut cals = input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<usize>().unwrap()).sum())
        .collect::<Vec<usize>>();

    cals.sort();

    cals.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 24000);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 45000);
    }
}
