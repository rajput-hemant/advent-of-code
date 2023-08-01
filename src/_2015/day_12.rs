use serde_json::Value;

pub fn part_1(input: &str) -> String {
    input
        .chars()
        .fold((0, String::new()), |(mut sum, mut num), c| {
            // check if the character is a digit or a minus sign.
            if c.is_digit(10) || c == '-' {
                // if it's a digit or a minus sign, add it to the current number
                num.push(c);
            } else {
                // if it's not a digit or a minus sign, then we've reached the end of a number,
                // parse the number and add it to the sum.
                if !num.is_empty() {
                    sum += num.parse::<i32>().unwrap();
                    // clear the num string to start forming a new number.
                    num.clear();
                }
            }
            (sum, num)
        })
        .0
        .to_string()
}

pub fn part_1_reg(input: &str) -> String {
    regex::Regex::new(r"-?\d+")
        .unwrap()
        .find_iter(input)
        .map(|m| m.as_str().parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    sum_json(&serde_json::from_str(input).unwrap()).to_string()
}

fn sum_json(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| sum_json(v)).sum(),
        Value::Object(o) => {
            if o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(|v| sum_json(v)).sum()
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("[1,2,3]"), "6");
        assert_eq!(part_1(r#"{"a":2,"b":4}"#), "6");
        assert_eq!(part_1("[[[3]]]"), "3");
        assert_eq!(part_1(r#"{"a":{"b":4},"c":-1}"#), "3");
        assert_eq!(part_1(r#"{"a":[-1,1]}"#), "0");
        assert_eq!(part_1(r#"[-1,{"a":1}]"#), "0");
        assert_eq!(part_1("[]"), "0");
        assert_eq!(part_1("{}"), "0");

        assert_eq!(part_1_reg("[1,2,3]"), "6");
        assert_eq!(part_1_reg(r#"{"a":2,"b":4}"#), "6");
        assert_eq!(part_1_reg("[[[3]]]"), "3");
        assert_eq!(part_1_reg(r#"{"a":{"b":4},"c":-1}"#), "3");
        assert_eq!(part_1_reg(r#"{"a":[-1,1]}"#), "0");
        assert_eq!(part_1_reg(r#"[-1,{"a":1}]"#), "0");
        assert_eq!(part_1_reg("[]"), "0");
        assert_eq!(part_1_reg("{}"), "0");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("[1,2,3]"), "6");
        assert_eq!(part_2(r#"[1,{"c":"red","b":2},3]"#), "4");
        assert_eq!(part_2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), "0");
        assert_eq!(part_2(r#"[1,"red",5]"#), "6");
    }
}
