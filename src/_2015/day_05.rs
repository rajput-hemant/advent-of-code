pub fn part_1(input: &str) -> String {
    input
        .lines()
        .filter(|line| {
            // contains at least three vowels
            line.chars().filter(|c| matches!(*c, 'a' | 'e' | 'i' | 'o' | 'u')).count() >= 3
                // contains at least one letter that appears twice in a row
                && line.as_bytes().windows(2).any(|w| w[0] == w[1])
                // does not contain the strings ab, cd, pq, or xy
                && !["ab", "cd", "pq", "xy"].iter().any(|s| line.contains(s))
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("ugknbfddgicrmopn"), "1");
        assert_eq!(part_1("aaa"), "1");
        assert_eq!(part_1("jchzalrnumimnmhp"), "0");
        assert_eq!(part_1("haegwjzuvuyypxyu"), "0");
        assert_eq!(part_1("dvszwmarrgswjxmb"), "0");
    }
}
