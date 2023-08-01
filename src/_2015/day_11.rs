pub fn part_1(input: &str) -> String {
    let mut password = input.chars().collect();

    loop {
        increase_password(&mut password);

        if !has_invalid_char(&password)
            && has_increasing_straight(&password)
            && has_two_pairs(&password)
        {
            break;
        }
    }

    password.iter().collect()
}

pub fn part_2(input: &str) -> String {
    part_1(&part_1(input))
}

fn has_increasing_straight(password: &Vec<char>) -> bool {
    (0..24)
        .map(|i| (0..3).map(|j| (i + j + 97) as u8 as char).collect())
        .collect::<Vec<String>>()
        .iter()
        .any(|s| password.iter().collect::<String>().contains(s))
}

fn has_invalid_char(password: &Vec<char>) -> bool {
    vec!['i', 'o', 'l']
        .iter()
        .any(|c| if password.contains(c) { true } else { false })
}

fn has_two_pairs(password: &Vec<char>) -> bool {
    let mut pairs = 0;
    let mut i = 0;

    while i < 7 {
        if password[i] == password[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
    }

    pairs >= 2
}

fn increase_password(password: &mut Vec<char>) {
    let mut i = 7;
    while i != 0 {
        if password[i] == 'z' {
            password[i] = 'a';
            i -= 1;
        } else {
            password[i] = (password[i] as u8 + 1) as char;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("abcdefgh"), "abcdffaa");
        assert_eq!(part_1("ghijklmn"), "ghjaabcc");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("abcdefgh"), "abcdffbb");
    }
}
