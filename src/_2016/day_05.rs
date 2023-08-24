pub fn part_1(input: &str) -> String {
    (0..)
        .map(|i| format!("{:x}", md5::compute(format!("{}{}", input, i))))
        .filter(|hash| hash.starts_with("00000"))
        .take(8)
        .map(|hash| hash.chars().nth(5).unwrap())
        .collect()
}

pub fn part_2(input: &str) -> String {
    let mut password = vec![' '; 8];
    let mut count = 0;

    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{}{}", input, i)));
        if hash.starts_with("00000") {
            let pos = hash.chars().nth(5).unwrap();
            if let Some(pos) = pos.to_digit(10).map(|pos| pos as usize) {
                if pos < 8 && password[pos] == ' ' {
                    password[pos] = hash.chars().nth(6).unwrap();
                    count += 1;
                    if count == 8 {
                        break;
                    }
                }
            }
        }
    }

    password.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("abc"), "18f47a30");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("abc"), "05ace8e3");
    }
}
