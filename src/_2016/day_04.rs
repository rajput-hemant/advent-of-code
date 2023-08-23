use lazy_regex::regex_captures;

pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, name, sector_id, checksum) =
                regex_captures!(r"([a-z-]+)-(\d+)\[([a-z]+)\]", line).unwrap();

            // remove dashes
            let mut chars: Vec<_> = name.chars().filter(|&c| c != '-').collect();

            // sort chars in alphabetical order
            chars.sort_unstable();

            // count consecutive chars
            let mut counts: Vec<_> = chars.into_iter().fold(vec![], |mut counts, c| {
                if let Some((prev, count)) = counts.last_mut() {
                    if *prev == c {
                        *count += 1;
                    } else {
                        counts.push((c, 1));
                    }
                } else {
                    counts.push((c, 1));
                }
                counts
            });

            // sort by count, then by char
            counts.sort_unstable_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

            // take the first 5 chars
            let expected_checksum: String = counts.into_iter().take(5).map(|(c, _)| c).collect();
            // compare checksums
            if expected_checksum == checksum {
                sector_id.parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, name, sector_id) = regex_captures!(r"([a-z-]+)-(\d+)\[[a-z]+\]", line).unwrap();
            let sector_id = sector_id.parse::<usize>().unwrap();

            let decrypted_name: String = name
                .chars()
                .map(|c| {
                    if c == '-' {
                        ' '
                    } else {
                        ((c as u8 - b'a' + (sector_id % 26) as u8) % 26 + b'a') as char
                    }
                })
                .collect();

            if decrypted_name.contains("northpole") {
                sector_id
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                "aaaaa-bbb-z-y-x-123[abxyz]\n\
                a-b-c-d-e-f-g-h-987[abcde]\n\
                not-a-real-room-404[oarel]\n\
                totally-real-room-200[decoy]"
            ),
            151
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("qzmt-zixmtkozy-ivhz-343[abcde]"), 0);
    }
}
