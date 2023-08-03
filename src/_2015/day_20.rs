pub fn part_1(input: &str) -> usize {
    let presents: usize = input.parse().unwrap();

    (1..presents / 10) // iter
        .fold(vec![0; presents / 10], |mut houses, i| {
            (i..presents / 10)
                .step_by(i)
                .for_each(|j| houses[j] += i * 10);
            houses
        })
        .iter()
        .position(|&x| x >= presents)
        .unwrap()
}

// pub fn part_1(input: &str) -> usize {
//     let input = input.parse().unwrap();
//     let mut houses = vec![0; input / 10];

//     for i in 1..input / 10 {
//         for j in (i..input / 10).step_by(i) {
//             houses[j] += i * 10;
//         }
//     }

//     houses.iter().position(|&x| x >= input).unwrap()
// }

pub fn part_2(input: &str) -> usize {
    let presents: usize = input.parse().unwrap();

    (1..presents / 11)
        .fold(vec![0; presents / 11], |mut houses, i| {
            (i..presents / 11)
                .step_by(i)
                .take(50)
                .for_each(|j| houses[j] += i * 11);
            houses
        })
        .iter()
        .position(|&x| x >= presents)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("150"), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("150"), 8);
    }
}
