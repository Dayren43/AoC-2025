fn silver_star(input: Option<&str>) -> u64 {
    let input = input.unwrap_or(include_str!("../input/day2.txt"));
    let mut total: u64 = 0;

    for section in input.trim().split(',') {
        let parts: Vec<&str> = section.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        for n in start..=end {
            let s = n.to_string();
            let len = s.len();

            // Invalid IDs must have even number of digits
            if len % 2 != 0 {
                continue;
            }

            let (a, b) = s.split_at(len / 2);
            if a == b {
                total += n;
            }
        }
    }

    println!("Silver: {}", total);
    total
}



fn gold_star(input: Option<&str>) -> u64 {
 let input = input.unwrap_or(include_str!("../input/day2.txt"));
    let mut total: u64 = 0;

    for section in input.trim().split(',') {
        let parts: Vec<&str> = section.split('-').collect();
        if parts.len() != 2 {
            continue;
        }

        let start: u64 = parts[0].parse().unwrap();
        let end: u64 = parts[1].parse().unwrap();

        'nloop: for n in start..=end {
            let s = n.to_string();
            let len = s.len();

            // Try all possible chunk sizes
            for chunk_size in 1..=len/2 {
                // Must divide evenly
                if len % chunk_size != 0 {
                    continue;
                }

                let chunk = &s[0..chunk_size];
                let repeats = len / chunk_size;

                // Check if s == chunk repeated repeats times
                if chunk.repeat(repeats) == s {
                    total += n;  // append full ID
                    continue 'nloop;   // number is invalid, skip rest of chunk sizes
                }
            }
        }
    }

            

    println!("Gold: {:?}", total);
    total
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day2::*;

    const TEST_INPUT: &str = indoc! {"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"};

    
    #[test]
    fn test_silver(){
        assert_eq!(silver_star(Some(TEST_INPUT)), 1227775554);
        silver_star(None);
    }   

    
    #[test]
    fn test_gold(){
        assert_eq!(gold_star(Some(TEST_INPUT)), 4174379265);
        gold_star(None);
    }   
}