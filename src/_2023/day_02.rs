pub fn part_01(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (game, sets) = l.split_once(":").unwrap();

            let (blue_max, red_max, green_max) = sets
                .split(|c| c == ',' || c == ';')
                .map(|s| {
                    let (qty, color) = s.trim().split_once(' ').unwrap();
                    (qty.parse::<usize>().unwrap(), color)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .fold((0, 0, 0), |(b, r, m), (qty, color)| match color {
                    "blue" => (qty.max(b), r, m),
                    "red" => (b, qty.max(r), m),
                    "green" => (b, r, qty.max(m)),
                    _ => unreachable!(),
                });

            if red_max <= 12 && green_max <= 13 && blue_max <= 14 {
                game.split_whitespace().nth(1).unwrap().parse().unwrap()
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(part_01(INPUT), 8);
    }
}
