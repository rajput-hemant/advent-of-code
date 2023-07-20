pub fn part(input: &str, time: u32) -> String {
    input
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let distance_per_sec: u32 = words[3].parse().unwrap();
            let flight_time: u32 = words[6].parse().unwrap();
            let resting_time: u32 = words[13].parse().unwrap();

            let trip_time = flight_time + resting_time; // total time for one trip
            let rounds = time / trip_time; // total complete rounds/trips in given time
            let remaining_time = time % trip_time; // remaining time after complete rounds

            let distance_covered = (rounds * distance_per_sec * flight_time)
                + if remaining_time > flight_time {
                    flight_time * distance_per_sec
                } else {
                    remaining_time * distance_per_sec
                };

            distance_covered
        })
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_part_1() {
        // 1120km at 1000th second
        assert_eq!(part(INPUT, 1000), "1120");

        // 2503km at 2503rd second
        assert_eq!(part(INPUT, 2503), "2660");
    }
}
