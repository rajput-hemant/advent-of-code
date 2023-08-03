use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    input
        .as_bytes() // convert to bytes
        .windows(4) // get windows of 4 bytes
        // check if all bytes are unique
        // .position(|b| {
        //     b[0] != b[1]
        //         && b[0] != b[2]
        //         && b[0] != b[3]
        //         && b[1] != b[2]
        //         && b[1] != b[3]
        //         && b[2] != b[3]
        // })
        // collect into hashset and if length is 4, all bytes are unique
        .position(|b| b.iter().collect::<HashSet<_>>().len() == 4)
        .unwrap()
        + 4 // add 4 as it will give the index of the last byte of the 4 bytes
}

pub fn part_2(input: &str) -> usize {
    input
        .as_bytes() // convert to bytes
        .windows(14) // get windows of 14 bytes
        // collect into hashset and if length is 14, all bytes are unique
        .position(|b| b.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        + 14 // add 14 as it will give the index of the last byte of the 14 bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUFFER1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const BUFFER2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const BUFFER3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const BUFFER4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const BUFFER5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(BUFFER1), 7);
        assert_eq!(part_1(BUFFER2), 5);
        assert_eq!(part_1(BUFFER3), 6);
        assert_eq!(part_1(BUFFER4), 10);
        assert_eq!(part_1(BUFFER5), 11);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(BUFFER1), 19);
        assert_eq!(part_2(BUFFER2), 23);
        assert_eq!(part_2(BUFFER3), 23);
        assert_eq!(part_2(BUFFER4), 29);
        assert_eq!(part_2(BUFFER5), 26);
    }
}
