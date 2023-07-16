pub fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut dims = line
                .split('x')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            dims.sort_unstable();

            let l = dims[0];
            let w = dims[1];
            let h = dims[2];

            ((2 * l * w) + (2 * w * h) + (2 * h * l)) // surface area
             + (l * w) // the area of the smallest side
        })
        .sum::<i32>()
        .to_string()
}
pub fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut dims = line
                .split('x')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            dims.sort_unstable();

            let l = dims[0];
            let w = dims[1];
            let h = dims[2];

            (2 * l) + (2 * w) // perimeter of the smallest side
                + (l * w * h) // volume
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1("2x3x4"), "58");
        assert_eq!(part_1("1x1x10"), "43");
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2("2x3x4"), "34");
        assert_eq!(part_2("1x1x10"), "14");
    }
}
