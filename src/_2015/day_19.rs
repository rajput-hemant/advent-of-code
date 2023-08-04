use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let (replacements, molecule) = parse_input(input);

    // let mut molecules = HashSet::new();

    // for (from, to) in replacements {
    //     for (i, _) in molecule.match_indices(from) {
    //         let new_molecule = molecule[..i].to_string() + to + &molecule[i + from.len()..];
    //         molecules.insert(new_molecule);
    //     }
    // }

    // molecules.len().to_string()

    replacements
        .into_iter()
        .flat_map(|(from, to)| {
            molecule
                .match_indices(from)
                .map(|(i, _)| molecule[..i].to_string() + to + &molecule[i + from.len()..])
        })
        .collect::<HashSet<_>>()
        .len()
}

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (replacements, molecule) = input.split_once("\n\n").unwrap();

    (
        replacements
            .lines()
            .map(|line| line.split_once(" => ").unwrap())
            .collect(),
        molecule,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"H => HO
H => OH
O => HH

HOH"#;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 4);
    }
}
