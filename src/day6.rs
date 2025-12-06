fn silver_star(input: Option<&str>) -> u64 {
    let normalized_input = input
        .unwrap_or(include_str!("../input/day6.txt"))
        .replace("\r\n", "\n");

    let lines: Vec<&str> = normalized_input.lines().collect();

    // Parse the grid of numbers
    let numbers: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    lines[lines.len() - 1]
        .split_whitespace()
        .enumerate()
        .map(|(i, op)| match op {
            "*" => numbers.iter().map(|row| row[i]).product::<u64>(),
            "+" => numbers.iter().map(|row| row[i]).sum::<u64>(),
            _ => panic!("Unknown operation: {}", op),
        })
        .sum()
}

pub fn gold_star(input: Option<&str>) -> u64 {
    let normalized_input = input
        .unwrap_or(include_str!("../input/day6.txt"))
        .replace("\r\n", "\n");

    let lines: Vec<&str> = normalized_input.lines().collect();

    let n_rows = lines.len() - 1; // last line = operators
    let n_cols = lines[0].len();
    let ops_line: Vec<char> = lines[n_rows].chars().filter(|c| !c.is_whitespace()).collect();

    let mut num_cols: Vec<Vec<u64>> = Vec::new();
    let mut current_col: Vec<u64> = Vec::new();

    for i in 0..n_cols {
        // Skip column if all spaces
        if lines[..n_rows].iter().all(|line| line.chars().nth(i) == Some(' ')) {
            if !current_col.is_empty() {
                num_cols.push(current_col);
                current_col = Vec::new();
            }
            continue;
        }

        // Build number from vertical digits
        let mut num_str = String::new();
        for row in 0..n_rows {
            num_str.push_str(lines[row][i..=i].trim());
        }

        current_col.push(num_str.parse().unwrap());
    }
    if !current_col.is_empty() {
        num_cols.push(current_col);
    }

    ops_line
        .iter()
        .enumerate()
        .map(|(i, op)| match op {
            '*' => num_cols[i].iter().product::<u64>(),
            '+' => num_cols[i].iter().sum::<u64>(),
            _ => panic!("Unknown operator: {}", op),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::day6::*;

    const TEST_INPUT: &str = indoc! {"
    123 328  51 64 
     45 64  387 23 
      6 98  215 314
    *   +   *   +  
    "};

    #[test]
    fn test_silver() {
        assert_eq!(silver_star(Some(TEST_INPUT)), 4277556);
        println!("{}", silver_star(None));
    }

    #[test]
    fn test_gold() {
        assert_eq!(gold_star(Some(TEST_INPUT)), 3263827);
        println!("{}", gold_star(None));
    }
}
