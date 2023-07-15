/*
--- Day 6: Tuning Trouble ---
The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way
toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that
it has many fancy features, but the most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he
convinced the other Elves that it would be okay to give you their one malfunctioning device -
surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to lock on to their signal. The signal
is a series of seemingly-random characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet
marker in the datastream. In the protocol being used by the Elves, the start of a packet is indicated by
a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to
identify the first position where the four most recently received characters were all different.
Specifically, it needs to report the number of characters from the beginning of the buffer to the
end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb

After the first three characters (mjq) have been received, there haven't been enough characters received
yet to find the marker. The first time a marker could occur is after the fourth character is received,
making the most recent four characters mjqj. Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters
received are jpqm, which are all different. In this case, your subroutine should report the value 7,
because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

How many characters need to be processed before the first start-of-packet marker is detected?
*/
// pub fn part_1(input: &str) -> usize {
//     (3..input.len()) // range of 3 to input length
//         .find(|&i| {
//             let slice = &input[i - 3..=i]; // get slice of 4 characters
//             slice
//                 .chars()
//                 .collect::<Vec<_>>() // collect into vector of chars
//                 .iter()
//                 .all(|&c| slice.matches(c).count() == 1) // check if all chars are unique
//         })
//         .unwrap()
//         + 1 // add 1 to account for 0-indexing
// }

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
}
