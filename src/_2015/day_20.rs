pub fn part_1(input: String) -> String {
    let presents: usize = input.parse().unwrap();

    (1..presents)
        .fold(vec![0; presents], |mut houses, i| {
            (i..presents).step_by(i).for_each(|j| houses[j] += i * 10);
            houses
        })
        .iter()
        .position(|&x| x >= presents)
        .unwrap()
        .to_string()
}

pub fn part_2(input: String) -> String {
    let presents: usize = input.parse().unwrap();

    (1..presents)
        .fold(vec![0; presents], |mut houses, i| {
            (i..presents)
                .step_by(i)
                .take(50)
                .for_each(|j| houses[j] += i * 11);
            houses
        })
        .iter()
        .position(|&x| x >= presents)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("150".to_string()), "8");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("150".to_string()), "8");
    }
}
