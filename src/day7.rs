use std::collections::HashSet;
use std::collections::HashMap;

fn silver_star(input: Option<&str>) -> i32 {
    let normalized_input = input
        .unwrap_or(include_str!("../input/day7.txt"))
        .replace("\r\n", "\n");

    let mut splits = 0;

    // ---- Initialize beams correctly ----
    let start_index = normalized_input
        .lines()
        .next()
        .unwrap()
        .char_indices()
        .find(|&(_, c)| c == 'S')
        .unwrap()
        .0;

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_index);

    // ---- Iterate through remaining lines ----
    for line in normalized_input.lines().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        let mut beams_next: HashSet<usize> = HashSet::new();

        for &beam in &beams {
            match chars[beam] {
                '^' => {
                    println!("Beam at {} split!", beam);
                    beams_next.insert(beam - 1);
                    beams_next.insert(beam + 1);
                    splits += 1;
                }
                _ => {
                    beams_next.insert(beam);
                }
            }
        }

        beams = beams_next;
    }

    splits
}

fn gold_star(input: Option<&str>) -> u64 {
    let normalized_input = input
        .unwrap_or(include_str!("../input/day7.txt"))
        .replace("\r\n", "\n");

    // ---- Initialize beams correctly ----
    let start_index = normalized_input
        .lines()
        .next()
        .unwrap()
        .char_indices()
        .find(|&(_, c)| c == 'S')
        .unwrap()
        .0;

    // map: column -> number of timelines reaching that column at current row
    let mut counts: HashMap<usize, u64> = HashMap::new();
    counts.insert(start_index, 1); // single particle starts -> 1 timeline

    // iterate remaining lines
    for line in normalized_input.lines().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        let width = chars.len();

        let mut next_counts: HashMap<usize, u64> = HashMap::new();

        for (&col, &ways) in &counts {

            match chars[col] {
                '^' => {
                    //add the current timeline to the splits in the next row
                    if col > 0 {
                        *next_counts.entry(col - 1).or_insert(0) += ways;
                    }
                    if col + 1 < width {
                        *next_counts.entry(col + 1).or_insert(0) += ways;
                    }
                }
                _ => {
                    // continue straight down (same column)
                    *next_counts.entry(col).or_insert(0) += ways;
                }
            }
        }

        counts = next_counts;
    }

    // total timelines after finishing all rows:
    counts.values().copied().sum()
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::day7::*;

    const TEST_INPUT: &str = indoc! {"
    .......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............
    "};

    #[test]
    fn test_silver() {
        assert_eq!(silver_star(Some(TEST_INPUT)), 21);
        println!("{}", silver_star(None));
    }

    #[test]
    fn test_gold() {
        assert_eq!(gold_star(Some(TEST_INPUT)), 40);
        println!("{}", gold_star(None));
    }

}
