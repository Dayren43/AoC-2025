use itertools::Itertools;

fn silver_star(input: Option<&str>) -> u64 {
    let input = input.unwrap_or(include_str!("../input/day3.txt"));

    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> =
                line.chars().map(|c| c.to_digit(10).unwrap()).collect();

            let digit1 = *digits[0..digits.len() - 1]
                .iter()
                .max()
                .unwrap();

            let pos = digits.iter().position(|&x| x == digit1).unwrap();

            let digit2 = *digits[pos+1..]
                .iter()
                .max()
                .unwrap();

            // build two-digit number
            format!("{}{}", digit1, digit2).parse::<u64>().unwrap()
        })
        .sum()
}

fn gold_star(input: Option<&str>) -> u64 {

    let input = input.unwrap_or(include_str!("../input/day3.txt"));

    input.lines().map(|line| {
        let digits: Vec<u32> = line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let need = 12;
        let mut drop = digits.len() - need;
        let mut stack: Vec<u32> = Vec::new();

        for &d in &digits {
            while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
                stack.pop();
                drop -= 1;
            }
            stack.push(d);
        }
        (&stack[..need]).iter()
            .map(u32::to_string)
            .join("")
            .parse::<u64>()
            .unwrap()
    }).sum()
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day3::*;

    const TEST_INPUT: &str = indoc! {"
    987654321111111
    811111111111119
    234234234234278
    818181911112111
    "};

    
    #[test]
    fn test_silver(){
        assert_eq!(silver_star(Some(TEST_INPUT)), 357);
        println!("{}", silver_star(None));
    }   

    
    #[test]
    fn test_gold(){
        assert_eq!(gold_star(Some(TEST_INPUT)), 3121910778619);
        println!("{}", gold_star(None));
    }   
}