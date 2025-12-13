use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use regex::Regex;

struct Button {
    flips: HashSet<u64>
}

pub fn silver_star(inp: Option<&str>) -> i64 {
    let input = inp.unwrap_or(include_str!("../input/day10.txt")).replace("\r\n", "\n");

    input.lines().map(|line| {
        let re = Regex::new(r"[\[\{\(]([^\]\}\)]+)[\]\}\)]").unwrap();

        let result = re.captures_iter(line).collect_vec();
        let target_str = result.get(0).map(|c| c.get(1).unwrap().as_str()).unwrap();
        
        // Parse target: which positions should be on (#)
        let target: HashSet<u64> = target_str.chars().enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as u64)
            .collect();
        
        // Parse buttons (skip first capture which is target, and last which is jolt)
        let buttons: Vec<Button> = result.iter()
            .skip(1)
            .take(result.len().saturating_sub(2))
            .map(|capture| {
                let content = capture.get(1).unwrap().as_str();
                let flips: HashSet<u64> = content
                    .split(',')
                    .map(|s| s.trim().parse::<u64>().unwrap())
                    .collect();
                Button { flips }
            })
            .collect();
        
        // Try all possible combinations of button presses (each button 0 or 1 times)
        let n = buttons.len();
        
        // Generate all possible combinations with their resulting XOR
        let mut all_possible_ops: Vec<(usize, HashSet<u64>)> = (0..(1 << n))
            .map(|mask: i32| {
                let presses = mask.count_ones() as usize;
                let mut state = HashSet::new();
                
                for i in 0..n {
                    if (mask & (1 << i)) != 0 {
                        // XOR: symmetric difference
                        state = state.symmetric_difference(&buttons[i].flips).copied().collect();
                    }
                }
                
                (presses, state)
            })
            .collect();
        
        // Sort by number of presses
        all_possible_ops.sort_by_key(|(presses, _)| *presses);
        
        // Find first match
        all_possible_ops.iter()
            .find(|(_, state)| state == &target)
            .map(|(presses, _)| *presses as i64)
            .unwrap_or(i64::MAX)
    }).sum()
}


pub fn gold_star(inp: Option<&str>) -> i64 {
    let input = inp.unwrap_or(include_str!("../input/day10.txt")).replace("\r\n", "\n");

    input.lines().enumerate().map(|(line_num, line)| {
        let re = Regex::new(r"[\[\{\(]([^\]\}\)]+)[\]\}\)]").unwrap();
        let result = re.captures_iter(line).collect_vec();
        
        let buttons: Vec<Button> = result.iter()
            .skip(1)
            .take(result.len().saturating_sub(2))
            .map(|capture| {
                let content = capture.get(1).unwrap().as_str();
                let flips: HashSet<u64> = content
                    .split(',')
                    .map(|s| s.trim().parse::<u64>().unwrap())
                    .collect();
                Button { flips }
            })
            .collect();
        
        let jolt_str = result.last().map(|c| c.get(1).unwrap().as_str()).unwrap();
        let target: Vec<i32> = jolt_str
            .split(',')
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        
        let solution = solve_with_z3(&buttons, &target);
        solution
    }).sum()
}

fn solve_with_z3(buttons: &[Button], target: &[i32]) -> i64 {
    use z3::*;
    
    let cfg = Config::new();
    let num_buttons = buttons.len();
    let num_counters = target.len();
    
    // Use with_z3_config to set up the context with our config
    with_z3_config(&cfg, || {
        let optimizer = Optimize::new();
        
        // Create integer variables for each button (no context needed)
        let button_vars: Vec<ast::Int> = (0..num_buttons)
            .map(|i| ast::Int::new_const(format!("button_{}", i)))
            .collect();
        
        // Add bounds constraints (0 <= button_i <= 1000)
        for var in &button_vars {
            optimizer.assert(&var.ge(&ast::Int::from_i64(0)));
            optimizer.assert(&var.le(&ast::Int::from_i64(1000)));
        }
        
        // Add constraints for each counter
        for counter_idx in 0..num_counters {
            let relevant_buttons: Vec<ast::Int> = button_vars.iter()
                .enumerate()
                .filter(|(btn_idx, _)| buttons[*btn_idx].flips.contains(&(counter_idx as u64)))
                .map(|(_, var)| var.clone())
                .collect();
            
            if !relevant_buttons.is_empty() {
                let sum = if relevant_buttons.len() == 1 {
                    relevant_buttons[0].clone()
                } else {
                    // Build sum iteratively
                    let mut sum = relevant_buttons[0].clone();
                    for var in &relevant_buttons[1..] {
                        sum = sum + var;
                    }
                    sum
                };
                
                let target_val = ast::Int::from_i64(target[counter_idx] as i64);
                optimizer.assert(&sum._eq(&target_val));
            } else {
                // If no buttons affect this counter, target must be 0
                if target[counter_idx] != 0 {
                    return i64::MAX; // Unsolvable
                }
            }
        }
        
        // Minimize the sum of all button presses
        let total = if button_vars.len() == 1 {
            button_vars[0].clone()
        } else {
            let mut total = button_vars[0].clone();
            for var in &button_vars[1..] {
                total = total + var;
            }
            total
        };
        
        optimizer.minimize(&total);
        
        // Solve
        match optimizer.check(&[]) {
            SatResult::Sat => {
                let model = optimizer.get_model().unwrap();
                button_vars.iter()
                    .map(|v| model.eval(v, true).unwrap().as_i64().unwrap())
                    .sum()
            }
            _ => {
                eprintln!("Solver could not find a solution");
                i64::MAX
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "};

    #[test]
    fn test_silver() {
        assert_eq!(silver_star(Some(TEST_INPUT)), 7);
        println!("{}", silver_star(None));
    }

    #[test]
    fn test_gold() {
        assert_eq!(gold_star(Some(TEST_INPUT)), 33);
        println!("{}", gold_star(None));
    }
}