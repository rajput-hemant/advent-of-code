use std::collections::HashSet;

pub fn part_1(input: &str) -> String {
    let (mut x, mut y) = (0, 0); // coordinates

    (input
        .chars() // split into chars
        .fold(HashSet::new(), |mut acc, c| {
            acc.insert((x, y));

            match c {
                '^' => y += 1, // move up
                'v' => y -= 1, // move down
                '>' => x += 1, // move right
                '<' => x -= 1, // move left
                _ => unreachable!(),
            }

            acc.insert((x, y));

            acc
        })
        .len())
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(">"), "2");
        assert_eq!(part_1("^>v<"), "4");
        assert_eq!(part_1("^v^v^v^v^v"), "2");
    }
}
