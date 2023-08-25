pub fn part_1(input: &str) -> String {
    let mut result = String::new();

    for i in 0..input.lines().next().unwrap().len() {
        let mut counts = [0; 26];

        for line in input.lines() {
            let c = line.chars().nth(i).unwrap() as usize - 97;
            counts[c] += 1;
        }

        let max = counts.into_iter().max().unwrap();
        let index = counts.into_iter().position(|x| x == max).unwrap();

        result.push((index + 97) as u8 as char);
    }

    result
}

pub fn part_2(input: &str) -> String {
    let mut result = String::new();

    for i in 0..input.lines().next().unwrap().len() {
        let mut counts = [0; 26];

        for line in input.lines() {
            let c = line.chars().nth(i).unwrap() as usize - 97;
            counts[c] += 1;
        }

        let min = counts.into_iter().filter(|x| *x != 0).min().unwrap();
        let index = counts.into_iter().position(|x| x == min).unwrap();

        result.push((index + 97) as u8 as char);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "easter");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), "advent");
    }
}
