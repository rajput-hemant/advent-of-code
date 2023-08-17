const COMPASS: [(i32, i32); 4] = [
    (0, 1),  // North
    (1, 0),  // East
    (0, -1), // South
    (-1, 0), // West
];

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .split(", ")
        .map(|i| (i.chars().next().unwrap(), i[1..].parse().unwrap()))
        .collect()
}

pub fn part_1(input: &str) -> usize {
    let (x, y, _) = parse_input(input)
        .into_iter()
        .fold((0, 0, 0), |(x, y, dir), (turn, steps)| {
            // update the direction based on the turn ('R' for right, 'L' for left).
            // dir ranges from 0 to 3 (representing North, East, South, West respectively).
            let dir = (dir + if turn == 'R' { 1 } else { 3 }) % 4;

            // Update the position based on the direction and number of steps.
            (x + COMPASS[dir].0 * steps, y + COMPASS[dir].1 * steps, dir)
        });

    // calculate the Manhattan distance by taking the absolute sum of x and y coordinates
    (x.abs() + y.abs()) as usize
}

pub fn part_2(input: &str) -> usize {
    let (mut x, mut y, mut dir) = (0, 0, 0);
    let mut visited = std::collections::HashSet::new();

    for (turn, steps) in parse_input(input) {
        dir = (dir + if turn == 'R' { 1 } else { 3 }) % 4;

        for _ in 0..steps {
            x += COMPASS[dir].0;
            y += COMPASS[dir].1;

            if !visited.insert((x, y)) {
                return (x.abs() + y.abs()) as usize;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("R2, L3"), 5);
        assert_eq!(part_1("R2, R2, R2"), 2);
        assert_eq!(part_1("R5, L5, R5, R3"), 12);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("R8, R4, R4, R8"), 4);
    }
}
