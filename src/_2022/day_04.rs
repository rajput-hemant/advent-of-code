pub fn part_1(input: &str) -> String {
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
        // check if the first range is inside the second range or vice versa
        // a-b is in c-d OR c-d is in a-b
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count() // count the number of ranges that are inside or overlaps with another range
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part_1(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            ),
            "2"
        );
    }
}
