use std::collections::{HashMap, HashSet};

type Shape = Vec<(i32, i32)>;

fn parse_input(input: &str) -> (Vec<Shape>, Vec<(usize, usize, Vec<usize>)>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Parse shape definitions
        if line.ends_with(':') && line.chars().next().map_or(false, |c| c.is_digit(10)) {
            let mut shape_lines = Vec::new();
            i += 1;
            
            while i < lines.len() && !lines[i].trim().is_empty() && !lines[i].contains(':') {
                shape_lines.push(lines[i]);
                i += 1;
            }
            
            shapes.push(parse_shape(&shape_lines));
        }
        // Parse region definitions
        else if line.contains("x") && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let dims: Vec<&str> = parts[0].split('x').collect();
                if dims.len() == 2 {
                    let width = dims[0].parse::<usize>().unwrap();
                    let height = dims[1].parse::<usize>().unwrap();
                    let counts: Vec<usize> = parts[1]
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                    regions.push((width, height, counts));
                }
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    
    (shapes, regions)
}

fn parse_shape(lines: &[&str]) -> Shape {
    let mut cells = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                cells.push((r as i32, c as i32));
            }
        }
    }
    normalize_shape(cells)
}

fn normalize_shape(mut cells: Shape) -> Shape {
    if cells.is_empty() {
        return cells;
    }
    
    let min_r = cells.iter().map(|(r, _)| *r).min().unwrap();
    let min_c = cells.iter().map(|(_, c)| *c).min().unwrap();
    
    cells.iter_mut().for_each(|(r, c)| {
        *r -= min_r;
        *c -= min_c;
    });
    
    cells.sort();
    cells
}

fn rotate_shape(shape: &Shape) -> Shape {
    let rotated: Shape = shape.iter().map(|(r, c)| (*c, -*r)).collect();
    normalize_shape(rotated)
}

fn flip_shape(shape: &Shape) -> Shape {
    let flipped: Shape = shape.iter().map(|(r, c)| (*r, -*c)).collect();
    normalize_shape(flipped)
}

fn get_all_transformations(shape: &Shape) -> Vec<Shape> {
    let mut transformations = HashSet::new();
    let mut current = shape.clone();
    
    for _ in 0..4 {
        transformations.insert(current.clone());
        transformations.insert(flip_shape(&current));
        current = rotate_shape(&current);
    }
    
    transformations.into_iter().collect()
}

fn can_place_shape(grid: &[Vec<bool>], shape: &Shape, start_r: i32, start_c: i32) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    
    for (dr, dc) in shape {
        let r = start_r + dr;
        let c = start_c + dc;
        
        if r < 0 || r >= height || c < 0 || c >= width {
            return false;
        }
        
        if grid[r as usize][c as usize] {
            return false;
        }
    }
    
    true
}

fn place_shape(grid: &mut [Vec<bool>], shape: &Shape, start_r: i32, start_c: i32) {
    for (dr, dc) in shape {
        let r = (start_r + dr) as usize;
        let c = (start_c + dc) as usize;
        grid[r][c] = true;
    }
}

fn remove_shape(grid: &mut [Vec<bool>], shape: &Shape, start_r: i32, start_c: i32) {
    for (dr, dc) in shape {
        let r = (start_r + dr) as usize;
        let c = (start_c + dc) as usize;
        grid[r][c] = false;
    }
}

// Find the first empty cell (top-left-most)
fn find_first_empty(grid: &[Vec<bool>]) -> Option<(i32, i32)> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if !cell {
                return Some((r as i32, c as i32));
            }
        }
    }
    None
}

// Count empty cells for early termination
fn count_empty_cells(grid: &[Vec<bool>]) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|&&c| !c).count())
        .sum()
}

fn solve_packing(
    grid: &mut [Vec<bool>],
    presents: &[Vec<Shape>],
    index: usize,
) -> bool {
    if index >= presents.len() {
        return true;
    }
    
    // Early termination: check if we have enough space for remaining presents
    let empty_cells = count_empty_cells(grid);
    let remaining_cells: usize = presents[index..]
        .iter()
        .map(|shapes| shapes[0].len())
        .sum();
    
    if empty_cells < remaining_cells {
        return false;
    }
    
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    
    // Try all transformations at all valid positions
    for shape in &presents[index] {
        for r in 0..height {
            for c in 0..width {
                if can_place_shape(grid, shape, r, c) {
                    place_shape(grid, shape, r, c);
                    
                    if solve_packing(grid, presents, index + 1) {
                        return true;
                    }
                    
                    remove_shape(grid, shape, r, c);
                }
            }
        }
    }
    
    false
}

fn check_region(
    width: usize,
    height: usize,
    counts: &[usize],
    shapes: &[Shape],
) -> bool {
    let mut grid = vec![vec![false; width]; height];
    let mut presents = Vec::new();
    
    for (shape_idx, &count) in counts.iter().enumerate() {
        for _ in 0..count {
            if shape_idx < shapes.len() {
                presents.push(get_all_transformations(&shapes[shape_idx]));
            }
        }
    }
    
    // Sort presents by size (largest first) to reduce search space
    presents.sort_by(|a, b| b[0].len().cmp(&a[0].len()));
    
    solve_packing(&mut grid, &presents, 0)
}

pub fn silver_star(inp: Option<&str>) -> i64 {
    let default_input = include_str!("../input/day12.txt");
    let input = inp.unwrap_or(default_input).replace("\r\n", "\n");
    
    let (shapes, regions) = parse_input(&input);
    
    let mut valid_count = 0;
    for (width, height, counts) in regions {
        if check_region(width, height, &counts, &shapes) {
            valid_count += 1;
        }
    }
    
    valid_count
}

pub fn gold_star(inp: Option<&str>) -> i64 {
    let default_input = include_str!("../input/day12.txt");
    let input = inp.unwrap_or(default_input).replace("\r\n", "\n");
    
    // Part 2 implementation will go here
    5
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    
    const TEST_INPUT: &str = indoc! {"
    0:
    ###
    ##.
    ##.

    1:
    ###
    ##.
    .##

    2:
    .##
    ###
    ##.

    3:
    ##.
    ###
    ##.

    4:
    ###
    #..
    ###

    5:
    ###
    .#.
    ###

    4x4: 0 0 0 0 2 0
    12x5: 1 0 1 0 2 2
    12x5: 1 0 1 0 3 2
    "};
    
    #[test]
    fn test_silver() {
        assert_eq!(silver_star(Some(TEST_INPUT)), 2);
        println!("paths: {}", silver_star(None));
    }

    #[test]
    fn test_gold() {
        // assert_eq!(gold_star(Some(TEST_INPUT)), 2);
        // println!("paths: {}", gold_star(None));
    }
}