#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }

    #[inline]
    fn delta(self, amount: i32) -> i32 {
        match self {
            Direction::Right => amount,
            Direction::Left  => -amount,
        }
    }
}

/// Move the dial and count how many times it hits 0 *during* the movement.
/// Returns number of zero crossings. Updates `state` to final wrapped position.
fn count_and_move(state: &mut i32, direction: Direction, amount: i32) -> i32 {
    let old = *state;
    let delta = direction.delta(amount);
    let raw  = old + delta;

    // Zero-crossing calculation
    let zeroes = if delta >= 0 {
        // Moving right/up
        raw.div_euclid(100) - old.div_euclid(100)
    } else {
        // Moving left/down (include landing-on-0 cases)
        (old - 1).div_euclid(100) - (raw - 1).div_euclid(100)
    };

    // Wrap to 0–99
    *state = raw.rem_euclid(100);

    zeroes
}

fn silver_star(input: Option<&str>) -> i32{
    let input = input.unwrap_or(include_str!("../input/day1.txt"));

    let mut state = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let (dir, n) = line.split_at(1);
        let direction = Direction::from_char(dir.chars().next().unwrap());
        let amount: i32 = n.parse().unwrap();

        // Part 1: only count if final state == 0
        let _ = count_and_move(&mut state, direction, amount);
        if state == 0 {
            zeroes += 1;
        }
    }

    println!("Silver: {}", zeroes);
    zeroes
}



fn gold_star(input: Option<&str>) -> i32 {
    let input = input.unwrap_or(include_str!("../input/day1.txt"));

    let mut state = 50;
    let mut zeroes = 0;
    let mut state  = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let (dir, n) = line.split_at(1);
        let direction = Direction::from_char(dir.chars().next().unwrap());
        let amount: i32 = n.parse().unwrap();

        // Part 2: count ALL intermediate zeroes
        zeroes += count_and_move(&mut state, direction, amount);
    }

    println!("Gold: {}", zeroes);
    zeroes
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day1::*;

    const TEST_INPUT: &str = indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82"
    };

    
    #[test]
    fn test_silver(){
        assert_eq!(silver_star(Some(TEST_INPUT)), 3);
        silver_star(None);
    }   

    
    #[test]
    fn test_gold(){
        assert_eq!(gold_star(Some(TEST_INPUT)), 6);
        gold_star(None);
    }   
}