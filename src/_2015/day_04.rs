pub fn part_1(input: &str) -> String {
    let mut i = 0;

    loop {
        let hash = md5::compute(format!("{input}{i}",));

        // The trick is to notice that an MD5 hash is actually just 16 bytes.
        // But since this is hard to print, most people just use the hex
        // representation of the hash digest. In the hex representation,
        // each byte is shown with 2 characters. For example; the bytes [0, 0] become "00 00".

        // If the challenge were to find the hash with 4 leading zeros,
        // we could just check the first two bytes of the result and be done.
        // But since it asks for 5 leading zeros, we need to use a little trick.

        // https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7

        if hash[0] == 0 && hash[1] == 0 && hash[2] >> 4 == 0 {
            return i.to_string();
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1("abcdef"), "609043");
        assert_eq!(part_1("pqrstuv"), "1048970");
    }
}
