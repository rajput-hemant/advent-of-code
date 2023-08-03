pub fn part_1(input: &str) -> usize {
    input
        .lines() // split into lines
        .map(|l| l.split_at(l.len() / 2)) // split each line in half
        .map(|(first_half, second_half)| {
            // calculate the score of the first character that appears in both halves
            calculate_score(
                first_half
                    .chars() // split into characters
                    .find(|c| second_half.contains(*c)) // find the first character that appears in both halves
                    .unwrap(),
            )
        })
        .sum() // sum the scores
}

pub fn part_2(input: &str) -> usize {
    input
        .lines() // split into lines
        .collect::<Vec<_>>() // collect into a vector
        .chunks(3) // split lines into groups of 3
        .map(|group| {
            // calculate the score of the first character that appears in all three lines
            calculate_score(
                group[0]
                    .chars() // split into characters
                    .find(|c| group[1].contains(*c) && group[2].contains(*c)) // find the first character that appears in all three lines
                    .unwrap(),
            )
        })
        .sum() // sum the scores
}

fn calculate_score(char: char) -> usize {
    ('a'..='z')
        .chain('A'..='Z') // create a range from a to z and from A to Z
        .enumerate() // enumerate to get the index as well as the character
        // find the index of the character and add 1 to get the score
        .find_map(|(index, c)| if c == char { Some(index + 1) } else { None })
        .unwrap_or(0) // unwrap the score or return 0 if the character was not found
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 157);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 70);
    }
}
