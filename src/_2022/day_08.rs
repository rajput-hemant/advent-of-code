pub fn part_1(input: &str) -> String {
    count_visible_trees(&parse_input(input)).to_string()
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

// https://galaxyinferno.com/how-to-solve-advent-of-code-2022-day-8-with-python/
fn count_visible_trees(trees: &[Vec<i32>]) -> i32 {
    // count all trees at the edges as they are always visible
    let mut visible_trees: i32 = ((2 * trees[0].len()) + (2 * (trees.len() - 2))) as i32;

    // exclude the outer boundary
    for i in 1..trees.len() - 1 {
        for j in 1..trees[0].len() - 1 {
            let (mut columns, mut rows) = (Vec::new(), Vec::new());

            // for each tree, calculate the difference between the current tree
            // and all other trees in the same row and column
            for k in 0..trees.len() {
                columns.push(trees[k][j] - trees[i][j]);
            }

            for k in 0..trees[i].len() {
                rows.push(trees[i][k] - trees[i][j]);
            }

            // vector representing potential routes for the current tree
            let routes = vec![
                &rows[..j],        // left side of the current row
                &rows[j + 1..],    // right side of the current row
                &columns[..i],     // above the current column
                &columns[i + 1..], // below the current column
            ];

            // check if any of the potential routes have all negative values (lower trees)
            if routes.iter().any(|route| route.iter().all(|&x| x < 0)) {
                visible_trees += 1;
            }
        }
    }

    // Return the final count of visible trees
    visible_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "21");
    }
}
