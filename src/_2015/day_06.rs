pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .fold(vec![vec![false; 1000]; 1000], |mut lights, line| {
            match line[0] {
                "turn" => match line[1] {
                    "on" => {
                        let (start_x, start_y) = parse_range(line[2]);
                        let (end_x, end_y) = parse_range(line[4]);

                        (start_x..=end_x).for_each(|i| {
                            (start_y..=end_y).for_each(|j| {
                                lights[i][j] = true;
                            });
                        });
                    }
                    "off" => {
                        let (start_x, start_y) = parse_range(line[2]);
                        let (end_x, end_y) = parse_range(line[4]);

                        (start_x..=end_x).for_each(|i| {
                            (start_y..=end_y).for_each(|j| {
                                lights[i][j] = false;
                            });
                        });
                    }
                    _ => unreachable!(),
                },
                "toggle" => {
                    let (start_x, start_y) = parse_range(line[1]);
                    let (end_x, end_y) = parse_range(line[3]);

                    (start_x..=end_x).for_each(|i| {
                        (start_y..=end_y).for_each(|j| {
                            lights[i][j] = !lights[i][j];
                        });
                    });
                }
                _ => unreachable!(),
            }

            lights
        })
        .iter()
        .map(|row| row.iter().filter(|&light| *light).count())
        .sum::<usize>()
        .to_string()
}

fn parse_range(range: &str) -> (usize, usize) {
    let range = range.split(",").collect::<Vec<_>>();
    (range[0].parse().unwrap(), range[1].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("turn on 0,0 through 999,999"), "1000000");
        assert_eq!(part_1("toggle 0,0 through 999,0"), "1000");
        assert_eq!(part_1("turn off 499,499 through 500,500"), "0");
    }
}
