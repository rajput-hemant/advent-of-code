pub fn part_1(input: &str) -> usize {
    let keypad = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];

    input
        .lines()
        .fold(
            (1_usize, 1_usize, String::new()),
            |(mut x, mut y, mut code), line| {
                line.chars().for_each(|c| match c {
                    'U' => y = y.saturating_sub(1),
                    'D' => y = (y + 1).min(2),
                    'L' => x = x.saturating_sub(1),
                    'R' => x = (x + 1).min(2),
                    _ => unreachable!(),
                });

                code.push(keypad[y][x]);

                (x, y, code)
            },
        )
        .2
        .parse()
        .unwrap()
}

pub fn part_2(input: &str) -> String {
    let keypad = vec![
        vec![' ', ' ', '1', ' ', ' '],
        vec![' ', '2', '3', '4', ' '],
        vec!['5', '6', '7', '8', '9'],
        vec![' ', 'A', 'B', 'C', ' '],
        vec![' ', ' ', 'D', ' ', ' '],
    ];

    let (mut x, mut y) = (0, 2);

    input
        .lines()
        .map(|line| {
            for c in line.chars() {
                match c {
                    'U' => {
                        if y > 0 && keypad[y - 1][x] != ' ' {
                            y -= 1;
                        }
                    }
                    'D' => {
                        if y < 4 && keypad[y + 1][x] != ' ' {
                            y += 1;
                        }
                    }
                    'L' => {
                        if x > 0 && keypad[y][x - 1] != ' ' {
                            x -= 1;
                        }
                    }
                    'R' => {
                        if x < 4 && keypad[y][x + 1] != ' ' {
                            x += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }

            keypad[y][x]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"ULL
RRDDD
LURDL
UUUUD"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1985);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("INPUT"), "5DB3");
    }
}
