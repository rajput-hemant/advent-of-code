pub fn part_1(input: &str) -> usize {
    score(
        &input
            .lines()
            .map(|l| {
                (
                    l.as_bytes()[0] as char, // my move
                    l.as_bytes()[2] as char, // opponent's move
                )
            })
            .collect::<Vec<_>>(),
    )
}

pub fn part_2(input: &str) -> usize {
    score(
        &input
            .lines()
            .map(|l| {
                (
                    l.as_bytes()[0] as char, // my move
                    l.as_bytes()[2] as char, // opponent's move
                )
            })
            .collect::<Vec<_>>()
            .iter()
            .map(|(a, b)| match (a, b) {
                // win
                ('A', 'Z') => ('A', 'Y'),
                ('B', 'Z') => ('B', 'Z'),
                ('C', 'Z') => ('C', 'X'),
                // draw
                ('A', 'Y') => ('A', 'X'),
                ('B', 'Y') => ('B', 'Y'),
                ('C', 'Y') => ('C', 'Z'),
                // lose
                ('A', 'X') => ('A', 'Z'),
                ('B', 'X') => ('B', 'X'),
                ('C', 'X') => ('C', 'Y'),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>(),
    )
}

pub fn score(rounds: &[(char, char)]) -> usize {
    rounds
        .iter()
        .map(|(a, b)| match (a, b) {
            ('A', 'Y') => 6 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 6 + 1,
            ('A', 'X') => 3 + 1,
            ('B', 'Y') => 3 + 2,
            ('C', 'Z') => 3 + 3,
            ('A', 'Z') => 0 + 3,
            ('B', 'X') => 0 + 1,
            ('C', 'Y') => 0 + 2,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 8 + 1 + 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 4 + 1 + 7);
    }
}
