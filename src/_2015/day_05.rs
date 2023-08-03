pub fn part_1(input: &str) -> usize {
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
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let line_bytes = line.as_bytes();

            // contains a pair of any two letters that appears at least twice in the string without overlapping
            line_bytes.windows(2).any(|w| {
                let s = std::str::from_utf8(w).unwrap();
                line.matches(&s).count() > 1
            }) 
            // contains at least one letter which repeats with exactly one letter between them
            && line_bytes.windows(3).any(|w| w[0] == w[2])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("ugknbfddgicrmopn"), 1);
        assert_eq!(part_1("aaa"), 1);
        assert_eq!(part_1("jchzalrnumimnmhp"), 0);
        assert_eq!(part_1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part_1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part_2("xxyxx"), 1);
        assert_eq!(part_2("uurcxstgmygtbstg"), 0);
        assert_eq!(part_2("ieodomkazucvgmuy"), 0);
    }
}
