use proconio::input;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

// 移動を安全に計算する関数
fn safe_move(curr: Point, dir: char, dist: i64) -> Point {
    match dir {
        'U' => {
            if dist > 0 && curr.y > i64::MAX - dist {
                Point {
                    x: curr.x,
                    y: i64::MAX,
                }
            } else if dist < 0 && curr.y < i64::MIN - dist {
                Point {
                    x: curr.x,
                    y: i64::MIN,
                }
            } else {
                Point {
                    x: curr.x,
                    y: curr.y + dist,
                }
            }
        }
        'D' => {
            if dist > 0 && curr.y < i64::MIN + dist {
                Point {
                    x: curr.x,
                    y: i64::MIN,
                }
            } else if dist < 0 && curr.y > i64::MAX + dist {
                Point {
                    x: curr.x,
                    y: i64::MAX,
                }
            } else {
                Point {
                    x: curr.x,
                    y: curr.y - dist,
                }
            }
        }
        'L' => {
            if dist > 0 && curr.x < i64::MIN + dist {
                Point {
                    x: i64::MIN,
                    y: curr.y,
                }
            } else if dist < 0 && curr.x > i64::MAX + dist {
                Point {
                    x: i64::MAX,
                    y: curr.y,
                }
            } else {
                Point {
                    x: curr.x - dist,
                    y: curr.y,
                }
            }
        }
        'R' => {
            if dist > 0 && curr.x > i64::MAX - dist {
                Point {
                    x: i64::MAX,
                    y: curr.y,
                }
            } else if dist < 0 && curr.x < i64::MIN - dist {
                Point {
                    x: i64::MIN,
                    y: curr.y,
                }
            } else {
                Point {
                    x: curr.x + dist,
                    y: curr.y,
                }
            }
        }
        _ => curr,
    }
}

// 点が線分上にあるかどうかを判定する関数
fn is_point_on_line(p: Point, start: Point, end: Point) -> bool {
    if start.x == end.x {
        // 垂直線
        if p.x != start.x {
            return false;
        }
        let min_y = start.y.min(end.y);
        let max_y = start.y.max(end.y);
        return p.y >= min_y && p.y <= max_y;
    } else {
        // 水平線
        if p.y != start.y {
            return false;
        }
        let min_x = start.x.min(end.x);
        let max_x = start.x.max(end.x);
        return p.x >= min_x && p.x <= max_x;
    }
}

fn solve(
    _n: usize,
    _m: usize,
    sx: i64,
    sy: i64,
    houses: &[(i64, i64)],
    moves: &[(char, i64)],
) -> (i64, i64, usize) {
    let mut curr = Point { x: sx, y: sy };
    let mut visited = HashSet::new();

    for &(dir, dist) in moves {
        let next = safe_move(curr, dir, dist);

        // 各家について、現在の移動線分上にあるかチェック
        for (i, &(hx, hy)) in houses.iter().enumerate() {
            let house = Point { x: hx, y: hy };
            if is_point_on_line(house, curr, next) {
                visited.insert(i);
            }
        }

        curr = next;
    }

    (curr.x, curr.y, visited.len())
}

fn main() {
    input! {
        n: usize,
        m: usize,
        sx: i64,
        sy: i64,
        houses: [(i64, i64); n],
        moves: [(char, i64); m],
    }

    let (final_x, final_y, visited_count) = solve(n, m, sx, sy, &houses, &moves);
    println!("{} {} {}", final_x, final_y, visited_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample1() {
        let houses = vec![(2, 2), (3, 3), (2, 1)];
        let moves = vec![('L', 2), ('D', 1), ('R', 1), ('U', 2)];
        assert_eq!(solve(3, 4, 3, 2, &houses, &moves), (2, 3, 2));
    }

    #[test]
    fn test_sample2() {
        let houses = vec![(1, 1)];
        let moves = vec![('R', 1000000000), ('R', 1000000000), ('R', 1000000000)];
        assert_eq!(solve(1, 3, 0, 0, &houses, &moves), (3000000000, 0, 0));
    }

    #[test]
    fn test_overflow() {
        let houses = vec![(i64::MAX - 1, 0)];
        let moves = vec![('R', 2)];
        let (x, _, _) = solve(1, 1, i64::MAX - 2, 0, &houses, &moves);
        assert_eq!(x, i64::MAX);
    }
}
