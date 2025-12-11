use std::collections::{HashMap, HashSet};

pub fn silver_star(inp: Option<&str>) -> i64 {
    let default_input = include_str!("../input/day11.txt");
    let input = inp.unwrap_or(default_input).replace("\r\n", "\n");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let id = parts[0].trim().to_string();
        let conns: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        graph.insert(id, conns);
    }

    let mut memo = HashMap::new();
    let mut visiting = HashSet::new();
    count_paths_dp(&graph, "you", &mut memo, &mut visiting)
}

fn count_paths_dp(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    memo: &mut HashMap<String, i64>,
    visiting: &mut HashSet<String>,
) -> i64 {
    if node == "out" {
        return 1;
    }

    if let Some(&v) = memo.get(node) {
        return v;
    }

    if visiting.contains(node) {
        return 0; // cycle detected
    }

    visiting.insert(node.to_string());

    let mut sum = 0;
    if let Some(neigh) = graph.get(node) {
        for n in neigh {
            sum += count_paths_dp(graph, n, memo, visiting);
        }
    }

    visiting.remove(node);
    memo.insert(node.to_string(), sum);
    sum
}

pub fn gold_star(inp: Option<&str>) -> i64 {
    let default_input = include_str!("../input/day11.txt");
    let input = inp.unwrap_or(default_input).replace("\r\n", "\n");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let id = parts[0].trim().to_string();
        let conns = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        graph.insert(id, conns);
    }

    let mut memo = HashMap::new();
    let mut visiting = HashSet::new();

    count_paths_require_dp(
        &graph,
        "svr",
        false,
        false,
        &mut memo,
        &mut visiting,
    )
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct State {
    node: String,
    seen_fft: bool,
    seen_dac: bool,
}

fn count_paths_require_dp(
    graph: &HashMap<String, Vec<String>>,
    node: &str,
    seen_fft: bool,
    seen_dac: bool,
    memo: &mut HashMap<State, i64>,
    visiting: &mut HashSet<State>,
) -> i64 {
    let mut st = State {
        node: node.to_string(),
        seen_fft,
        seen_dac,
    };

    if node == "out" {
        return if seen_fft && seen_dac { 1 } else { 0 };
    }

    if let Some(&v) = memo.get(&st) {
        return v;
    }

    if visiting.contains(&st) {
        return 0; // cycle
    }

    visiting.insert(st.clone());

    let mut sum = 0;
    if let Some(neigh) = graph.get(node) {
        for n in neigh {
            let next_fft = seen_fft || n == "fft";
            let next_dac = seen_dac || n == "dac";

            sum += count_paths_require_dp(
                graph,
                n,
                next_fft,
                next_dac,
                memo,
                visiting,
            );
        }
    }

    visiting.remove(&st);
    memo.insert(st, sum);
    sum
}

#[cfg(test)]
mod tests{
    use super::*;
    use indoc::indoc;
    
    const TEST_INPUT: &str = indoc! {"
    aaa: you hhh 
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg:out
    hhh: ccc fff iii
    iii: out "};
    
     #[test]
     fn test_silver() {
        assert_eq!(silver_star(Some(TEST_INPUT)), 5);
        println!("paths: {}", silver_star(None));
    }
    
    const TEST_INPUT_GOLD: &str = indoc! {"
    svr: aaa bbb
    aaa: fft
    fft: ccc
    bbb: tty
    tty: ccc
    ccc: ddd eee
    ddd: hub
    hub: fff
    eee: dac
    dac: fff
    fff: ggg hhh
    ggg: out
    hhh: out
    "};
    #[test]
    fn test_gold(){
        assert_eq!(gold_star(Some(TEST_INPUT_GOLD)), 2);
        println!("paths: {}", gold_star(None));
    }
}