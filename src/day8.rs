#[derive(Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn dist2(&self, other: &Pos) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Clone)]
struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, a: usize) -> usize {
        if self.parent[a] != a {
            self.parent[a] = self.find(self.parent[a]);
        }
        self.parent[a]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let pa = self.find(a);
        let pb = self.find(b);

        if pa == pb {
            return false;
        }

        if self.size[pa] < self.size[pb] {
            self.parent[pa] = pb;
            self.size[pb] += self.size[pa];
        } else {
            self.parent[pb] = pa;
            self.size[pa] += self.size[pb];
        }

        true
    }
    
    fn component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut freq = vec![0; n];
        for i in 0..n {
            let root = self.find(i);
            freq[root] += 1;
        }
        let mut sizes: Vec<usize> = freq.into_iter().filter(|&x| x > 0).collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes
    }
}

fn parse_positions(inp: &str) -> Vec<Pos> {
    inp.lines()
        .map(|line| {
            let v: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Pos { x: v[0], y: v[1], z: v[2] }
        })
        .collect()
}

pub fn silver_star(inp: Option<&str>) -> i64 {
    let input = inp.unwrap_or(include_str!("../input/day8.txt")).replace("\r\n", "\n");

    let positions: Vec<Pos> = parse_positions(&input);

    let n = positions.len();
    let mut dsu = DSU::new(n);

    let mut pairs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push((positions[i].dist2(&positions[j]), i, j));
        }
    }

    pairs.sort_by_key(|p| p.0);

    let merge_limit = if inp.is_some() { 10 } else { 1000 };
    let mut merges = 0;

    for (_dist, a, b) in pairs {
        dsu.union(a, b);
        merges += 1;
        if merges == merge_limit {
            break;
        }
    }

    let sizes = dsu.component_sizes();
    sizes[0] as i64 * sizes[1] as i64 * sizes[2] as i64
}

pub fn gold_star(inp: Option<&str>) -> i64 {
    let input = inp.unwrap_or(include_str!("../input/day8.txt")).replace("\r\n", "\n");

    let positions: Vec<Pos> = parse_positions(&input);

    let n = positions.len();
    let mut dsu = DSU::new(n);

    let mut pairs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push((positions[i].dist2(&positions[j]), i, j));
        }
    }
    pairs.sort_by_key(|p| p.0);

    let mut last_pair = (0, 0);

    for (_dist, a, b) in pairs {
        if dsu.union(a, b) {
            last_pair = (a, b);
            if dsu.component_sizes().len() == 1 {
                break;
            }
        }
    }

    let (a, b) = last_pair;
    positions[a].x as i64 * positions[b].x as i64
}


#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::day8::*;

    const TEST_INPUT: &str = indoc! {"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
    "};

    #[test]
    fn test_silver() {
        assert_eq!(silver_star(Some(TEST_INPUT)), 40);
        println!("{}", silver_star(None));
    }

    #[test]
    fn test_gold() {
        assert_eq!(gold_star(Some(TEST_INPUT)), 25272);
        println!("{}", gold_star(None));
    }

}