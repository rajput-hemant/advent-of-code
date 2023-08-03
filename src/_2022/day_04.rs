pub fn part_1(input: &str) -> usize {
    input
        .lines() // split into lines
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap(); // split at comma (a-b,c-d)
            let ((a, b), (c, d)) = (
                left.split_once("-").unwrap(),  // split at dash (a-b)
                right.split_once("-").unwrap(), // split at dash (c-d)
            );

            // convert from &str to u8
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        // filter out the ranges that are not inside or overlaps with another range
        // a-b is in c-d OR c-d is in a-b
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count() // count the number of ranges that are inside or overlaps with another range
}

/*
--- Part Two ---
It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like
to know the number of pairs that overlap at all.

In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining
four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

5-7,7-9 overlaps in a single section, 7.
2-8,3-7 overlaps all of the sections 3 through 7.
6-6,4-6 overlaps in a single section, 6.
2-6,4-8 overlaps in sections 4, 5, and 6.
So, in this example, the number of overlapping assignment pairs is 4.

In how many assignment pairs do the ranges overlap?
*/
pub fn part_2(input: &str) -> usize {
    input
        .lines() // split into lines
        .map(|line| {
            let (left, right) = line.split_once(",").unwrap(); // split at comma (a-b,c-d)
            let ((a, b), (c, d)) = (
                left.split_once("-").unwrap(),  // split at dash (a-b)
                right.split_once("-").unwrap(), // split at dash (c-d)
            );

            // convert from &str to u8
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        // filter out the ranges that are not overlapping
        .filter(|(a, b, c, d)| (a <= d && c <= b))
        .count() // count the number of ranges that are inside or overlaps with another range
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(INPUT), 2);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(INPUT), 4);
    }
}
