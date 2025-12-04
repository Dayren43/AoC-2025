use itertools::Itertools;

fn neighbor_count(matrix: &[Vec<i32>], x: usize, y: usize) -> usize {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let mut neighbor_count = 0usize;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < rows && ny >= 0 && ny < cols {
                if matrix[nx as usize][ny as usize] == 1 {
                    neighbor_count += 1;
                }
            }
        }
    }
    neighbor_count
}

fn silver_star(input: Option<&str>) -> u32 {
    let input = input.unwrap_or(include_str!("../input/day4.txt"));

    let matrix: Vec<Vec<i32>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect())
        .collect();

    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut count = 0u32;

    for (x, row) in matrix.iter().enumerate() {
        for (y, &val) in row.iter().enumerate() {
            // only consider cells that are '@'
            if val != 1 {
                continue;
            }
            let n = neighbor_count(&matrix, x, y);
            if n < 4 {
                count += 1;
            }
        }
    }

    count
}

fn gold_star(input: Option<&str>) -> u32 {
 let input = input.unwrap_or(include_str!("../input/day4.txt"));

    let mut matrix: Vec<Vec<i32>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().map(|c| if c == '@' { 1 } else { 0 }).collect())
        .collect();

    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }

    let mut count = 0u32;

loop {
    let mut to_remove = Vec::new();

    // 1. Identify all removable @'s in this round
    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            if matrix[x][y] == 1 && neighbor_count(&matrix, x, y) < 4 {
                to_remove.push((x, y));
            }
        }
    }

    // 2. If none, stop
    if to_remove.is_empty() {
        break;
    }

    // 3. Remove them simultaneously
    for (x, y) in &to_remove {
        matrix[*x][*y] = 0;
    }

    count += to_remove.len() as u32;
}
    count
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day4::*;

    const TEST_INPUT: &str = indoc! {"
    ..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.
    "};

    
    #[test]
    fn test_silver(){
        assert_eq!(silver_star(Some(TEST_INPUT)), 13);
        println!("{}", silver_star(None));
    }   

    
    #[test]
    fn test_gold(){
        assert_eq!(gold_star(Some(TEST_INPUT)), 43);
        println!("{}", gold_star(None));
    }   
}