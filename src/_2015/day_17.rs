use itertools::Itertools;

pub fn part_1(input: &str, litres: i32) -> String {
    find_combinations(&parse_input(input), litres)
        .len()
        .to_string()
}

pub fn part_2(input: &str, litres: i32) -> String {
    let combinations = find_combinations(&parse_input(input), litres);
    combinations
        .iter()
        // only keep the combinations with the minimum number of containers
        .filter(|c| c.len() == combinations.iter().map(|c| c.len()).min().unwrap())
        .count()
        .to_string()
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|c| c.parse().unwrap()).collect()
}

fn find_combinations(containers: &Vec<i32>, total_liters: i32) -> Vec<Vec<i32>> {
    (1..=containers.len())
        .flat_map(|i| {
            containers
                .iter()
                // .map(|&x| x)
                .copied() // same as above
                .combinations(i)
        })
        .filter(|c| c.iter().sum::<i32>() == total_liters)
        .collect()
}

// pub fn part_1(input: &str, litres: i32) -> String {
//     count_combinations(
//         &input.lines().map(|c| c.parse::<i32>().unwrap()).collect(),
//         litres,
//         0,
//     )
//     .to_string()
// }

// fn count_combinations(containers: &Vec<i32>, total_liters: i32, index: usize) -> i32 {
//     // if we have no more liters to fill, we have found a combination
//     if total_liters == 0 {
//         1
//     } else if total_liters < 0 || index >= containers.len() {
//         // if we have no more liters to fill or we have no more containers to use, we have no combination
//         0
//     } else {
//         // include the current container + exclude the current container
//         count_combinations(containers, total_liters - containers[index], index + 1)
//             + count_combinations(containers, total_liters, index + 1)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("25\n15\n10\n5\n5", 25), "3");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("25\n15\n10\n5\n5", 25), "1");
    }
}
