#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn area(&self, other: &Pos) -> i64 {
        let width = (self.x - other.x).abs();
        let height = (self.y - other.y).abs();
        (width + 1) * (height + 1)
    }
}

#[derive(Clone, Copy)]
struct Edge {
    min: Pos,
    max: Pos,
}

impl Edge {
    fn new(a: Pos, b: Pos) -> Self {
        if a < b {
            Self { min: a, max: b }
        } else {
            Self { min: b, max: a }
        }
    }

    fn is_horizontal(&self) -> bool {
        self.min.y == self.max.y
    }

    fn contains_x(&self, x: i64) -> bool {
        x >= self.min.x && x <= self.max.x
    }

    fn contains_y(&self, y: i64) -> bool {
        y >= self.min.y && y <= self.max.y
    }
}

fn parse_positions(inp: &str) -> Vec<Pos> {
    inp.lines()
        .map(|line| {
            let v: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Pos { x: v[0], y: v[1] }
        })
        .collect()
}

pub fn silver_star(inp: Option<&str>) -> i64 {
    let input = inp.unwrap_or(include_str!("../input/day9.txt")).replace("\r\n", "\n");
    let positions = parse_positions(&input);

    positions
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| {
            positions.iter().skip(i + 1).map(move |&b| a.area(&b))
        })
        .max()
        .unwrap()
}

fn is_inside(point: Pos, horizontal_edges: &[Edge], vertical_edges: &[Edge]) -> bool {
    // Check if point lies on any edge
    for edge in horizontal_edges {
        if edge.min.y == point.y && edge.contains_x(point.x) {
            return true;
        }
    }
    for edge in vertical_edges {
        if edge.min.x == point.x && edge.contains_y(point.y) {
            return true;
        }
    }

    // Ray casting algorithm: count horizontal edges below the point
    let mut crossings = 0;
    let mut on_vertex_right = 0;
    let mut on_vertex_left = 0;

    for edge in horizontal_edges {
        if edge.min.y <= point.y {
            continue;
        }

        if edge.contains_x(point.x) {
            if point.x == edge.min.x {
                if edge.max.x > point.x {
                    on_vertex_right += 1;
                } else {
                    on_vertex_left += 1;
                }
                if on_vertex_right == on_vertex_left {
                    crossings += 1;
                }
            } else if point.x == edge.max.x {
                if edge.min.x < point.x {
                    on_vertex_left += 1;
                } else {
                    on_vertex_right += 1;
                }
                if on_vertex_right == on_vertex_left {
                    crossings += 1;
                }
            } else {
                crossings += 1;
            }
        }
    }

    crossings % 2 == 1
}

fn edges_intersect(vert: Edge, horiz: Edge) -> bool {
    // Check if vertical edge crosses horizontal edge
    vert.min.x > horiz.min.x
        && vert.min.x < horiz.max.x
        && vert.min.y < horiz.min.y
        && vert.max.y > horiz.min.y
}

pub fn gold_star(inp: Option<&str>) -> i64 {
    let input = inp.unwrap_or(include_str!("../input/day9.txt")).replace("\r\n", "\n");
    let positions = parse_positions(&input);
    let n = positions.len();

    // Build sorted edge lists
    let mut horizontal_edges = Vec::new();
    let mut vertical_edges = Vec::new();

    for i in 0..n {
        let p1 = positions[i];
        let p2 = positions[(i + 1) % n];
        let edge = Edge::new(p1, p2);
        
        if edge.is_horizontal() {
            horizontal_edges.push(edge);
        } else {
            vertical_edges.push(edge);
        }
    }

    horizontal_edges.sort_unstable_by_key(|e| e.min.y);
    vertical_edges.sort_unstable_by_key(|e| e.min.x);

    let mut best_area = 0;

    for (i, &corner_a) in positions.iter().enumerate() {
        for &corner_b in positions.iter().skip(i + 1) {
            let area = corner_a.area(&corner_b);

            if area <= best_area {
                continue;
            }

            // Define rectangle bounds
            let (left, right) = (corner_a.x.min(corner_b.x), corner_a.x.max(corner_b.x));
            let (top, bottom) = (corner_a.y.min(corner_b.y), corner_a.y.max(corner_b.y));

            // Check if any polygon point is strictly inside the rectangle
            if positions.iter().any(|p| {
                p.x > left && p.x < right && p.y > top && p.y < bottom
            }) {
                continue;
            }

            // Check if all rectangle corners are inside the polygon
            let corners = [
                Pos { x: left, y: top },
                Pos { x: left, y: bottom },
                Pos { x: right, y: top },
                Pos { x: right, y: bottom },
            ];

            if !corners.iter().all(|&c| is_inside(c, &horizontal_edges, &vertical_edges)) {
                continue;
            }

            // Check if rectangle edges cross polygon edges
            let rect_top = Edge::new(corners[0], corners[2]);
            let rect_bottom = Edge::new(corners[1], corners[3]);
            let rect_left = Edge::new(corners[0], corners[1]);
            let rect_right = Edge::new(corners[2], corners[3]);

            let has_vertical_crossing = left < right
                && vertical_edges
                    .iter()
                    .take_while(|e| e.min.x < right)
                    .any(|&v| edges_intersect(v, rect_top) || edges_intersect(v, rect_bottom));

            let has_horizontal_crossing = top < bottom
                && horizontal_edges
                    .iter()
                    .take_while(|e| e.min.y < bottom)
                    .any(|&h| edges_intersect(rect_left, h) || edges_intersect(rect_right, h));

            if has_vertical_crossing || has_horizontal_crossing {
                continue;
            }

            best_area = area;
        }
    }

    best_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn test_silver() {
        assert_eq!(silver_star(Some(TEST_INPUT)), 50);
        println!("{}", silver_star(None));
    }

    #[test]
    fn test_gold() {
        assert_eq!(gold_star(Some(TEST_INPUT)), 24);
        println!("{}", gold_star(None));
    }
}