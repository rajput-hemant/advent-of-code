struct Reindeer {
    speed: u32,
    flight_time: u32,
    rest_time: u32,
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            let speed = words[3].parse().unwrap();
            let flight_time = words[6].parse().unwrap();
            let rest_time = words[13].parse().unwrap();

            Reindeer {
                speed,
                flight_time,
                rest_time,
            }
        })
        .collect()
}

pub fn part_1(input: &str, time: u32) -> String {
    parse_input(input)
        .iter()
        .map(|r| {
            let trip_time = r.flight_time + r.rest_time; // total time for one trip
            let rounds = time / trip_time; // total complete rounds/trips in given time
            let remaining_time = time % trip_time; // remaining time after complete rounds

            let distance_covered = (rounds * r.speed * r.flight_time)
                + if remaining_time > r.flight_time {
                    r.flight_time * r.speed
                } else {
                    remaining_time * r.speed
                };

            distance_covered
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn part_2(input: &str, time: u32) -> String {
    let reindeers = parse_input(input);
    let mut scores = vec![0; reindeers.len()]; // scores of each reindeers

    // iterate from 1 to the given `time`
    (1..=time).for_each(|t| {
        // variables to track the maximum distance and reindeers achieving that distance.
        let (mut max_distance, mut max_distance_reindeers) = (0, Vec::new());

        reindeers.iter().enumerate().for_each(|(i, r)| {
            let trip_time = r.flight_time + r.rest_time; // total time for one trip
            let rounds = t / trip_time; // total complete rounds/trips in given time
            let remaining_time = t % trip_time; //

            // distance covered in each round/trip + distance covered in remaining time
            let distance_covered = (rounds * r.speed * r.flight_time)
                + if remaining_time > r.flight_time {
                    r.flight_time * r.speed
                } else {
                    remaining_time * r.speed
                };

            // if the current distance is greater than the maximum distance,
            // update the maximum distance and the reindeers achieving that distance
            if distance_covered > max_distance {
                max_distance = distance_covered;
                max_distance_reindeers = vec![i];
            } else if distance_covered == max_distance {
                //  if the current distance is equal to the maximum distance,
                //  add the current reindeer to the list of reindeers achieving the maximum distance
                max_distance_reindeers.push(i);
            }
        });

        // update the scores of the reindeers that achieved the maximum distance at time `t`.
        max_distance_reindeers.iter().for_each(|&i| scores[i] += 1);
    });

    scores.iter().max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

    #[test]
    fn test_part_1() {
        // 1120km at 1000th second
        assert_eq!(part_1(INPUT, 1000), "1120");

        // 2503km at 2503rd second
        assert_eq!(part_1(INPUT, 2503), "2660");
    }

    #[test]
    fn test_part_2() {
        // 689km at 1000th second
        assert_eq!(part_2(INPUT, 1000), "689");

        // 1564km at 2503rd second
        assert_eq!(part_2(INPUT, 2503), "1564");
    }
}
