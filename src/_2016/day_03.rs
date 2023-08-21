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

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .chunks(3)
        .fold(0, |mut count, lines| {
            (0..3).for_each(|i| {
                let mut dims = vec![lines[0][i], lines[1][i], lines[2][i]];

                dims.sort();

                if dims[0] + dims[1] > dims[2] {
                    count += 1;
                }
            });

            count
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("5 10 25"), 0);
        assert_eq!(part_1("5 10 10"), 1);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603"
            ),
            6
        );
    }
}
