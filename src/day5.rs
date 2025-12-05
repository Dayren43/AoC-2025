fn silver_star(input: Option<&str>) -> u32 {
    let input = input
        .unwrap_or(include_str!("../input/day5.txt"))
        .replace("\r\n", "\n");


    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(u64, u64)> = parts[0]
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let mut count = 0;

    for product in parts[1].lines() {
        let n: u64 = product.parse().unwrap();
        if ranges.iter().any(|&(s, e)| n >= s && n <= e) {
            count += 1;
        }
    }

    count
}

fn gold_star(input: Option<&str>) -> u64 {
    let normalized_input = input
        .unwrap_or(include_str!("../input/day5.txt"))
        .replace("\r\n", "\n");

    let sections: Vec<&str> = normalized_input.split("\n\n").collect();

    // ---- Parse the fresh ID ranges ----
    let mut ranges: Vec<(u64, u64)> = sections[0]
        .lines()
        .map(|line| {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start_id = start_str.parse::<u64>().unwrap();
            let end_id = end_str.parse::<u64>().unwrap();
            (start_id, end_id)
        })
        .collect();

    ranges.sort_unstable_by_key(|range| range.0);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    for (range_start, range_end) in ranges {
        match merged_ranges.last_mut() {
            Some((_s, merged_end)) => {
                let overlaps = range_start <= *merged_end + 1;
                if overlaps {
                    // Extend the merged range to cover the new one
                    *merged_end = (*merged_end).max(range_end);
                } else {
                    merged_ranges.push((range_start, range_end));
                }
            }
            None => {
                merged_ranges.push((range_start, range_end));
            }
        }
    }

    merged_ranges
        .iter()
        .map(|(start_id, end_id)| end_id - start_id + 1)
        .sum()
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day5::*;

    const TEST_INPUT: &str = indoc! {"
    3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    "};

    
    #[test]
    fn test_silver(){
        assert_eq!(silver_star(Some(TEST_INPUT)), 3);
        println!("{}", silver_star(None));
    }   

    
    #[test]
    fn test_gold(){
        assert_eq!(gold_star(Some(TEST_INPUT)), 14);
        println!("{}", gold_star(None));
    }   
}