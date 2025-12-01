fn silver_star(input: Option<&str>){
    let input = input.unwrap_or(include_str!("../input/day2.txt"));
    let res: ();

    println!("Silver: {:?}", res);
    res
}



fn gold_star(input: Option<&str>) {
    let input = input.unwrap_or(include_str!("../input/day1.txt"));
    
    let res: ();
    println!("Gold: {:?}", res);
    res
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day1::*;

    const TEST_INPUT: &str = indoc! {""};

    
    #[test]
    fn test_silver(){
        assert_eq!(silver_star(Some(TEST_INPUT)), ());
        silver_star(None);
    }   

    
    #[test]
    fn test_gold(){
        assert_eq!(gold_star(Some(TEST_INPUT)), ());
        gold_star(None);
    }   
}