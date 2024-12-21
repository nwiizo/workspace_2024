use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

fn simulate_santa_movement(
    h: usize,
    w: usize,
    start: Position,
    grid: &Vec<Vec<char>>,
    moves: &Vec<char>,
) -> (Position, usize) {
    let mut current_pos = start;
    let mut visited_houses = HashSet::new();

    // 初期位置が家の場合もカウント
    if grid[current_pos.x][current_pos.y] == '@' {
        visited_houses.insert((current_pos.x, current_pos.y));
    }

    for &movement in moves {
        let (next_x, next_y) = match movement {
            'U' => (current_pos.x.saturating_sub(1), current_pos.y),
            'D' => (current_pos.x + 1, current_pos.y),
            'L' => (current_pos.x, current_pos.y.saturating_sub(1)),
            'R' => (current_pos.x, current_pos.y + 1),
            _ => (current_pos.x, current_pos.y),
        };

        // 移動先が通行可能（'.' または '@'）かチェック
        if next_x < h && next_y < w && (grid[next_x][next_y] == '.' || grid[next_x][next_y] == '@')
        {
            current_pos = Position {
                x: next_x,
                y: next_y,
            };

            // 家に到達した場合、セットに追加
            if grid[next_x][next_y] == '@' {
                visited_houses.insert((next_x, next_y));
            }
        }
    }

    (current_pos, visited_houses.len())
}

fn main() {
    input! {
        h: usize,
        w: usize,
        x: usize,
        y: usize,
        grid: [Chars; h],
        moves: Chars,
    }

    let start_pos = Position {
        x: x - 1, // 0-basedに変換
        y: y - 1,
    };

    let (final_pos, houses_visited) = simulate_santa_movement(h, w, start_pos, &grid, &moves);

    // 1-basedで出力
    println!("{} {} {}", final_pos.x + 1, final_pos.y + 1, houses_visited);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case_1() {
        let h = 5;
        let w = 5;
        let grid = vec![
            vec!['#', '#', '#', '#', '#'],
            vec!['#', '.', '.', '.', '#'],
            vec!['#', '.', '@', '.', '#'],
            vec!['#', '.', '.', '@', '#'],
            vec!['#', '#', '#', '#', '#'],
        ];
        let start_pos = Position { x: 2, y: 3 }; // 3,4 in 1-based
        let moves = "LLLDRUU".chars().collect();

        let (final_pos, houses_visited) = simulate_santa_movement(h, w, start_pos, &grid, &moves);
        assert_eq!(final_pos.x + 1, 2);
        assert_eq!(final_pos.y + 1, 3);
        assert_eq!(houses_visited, 1);
    }

    #[test]
    fn test_sample_case_2() {
        let h = 6;
        let w = 13;
        let grid = vec![
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
            vec![
                '#', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '#',
            ],
            vec![
                '#', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '#',
            ],
            vec![
                '#', '@', '@', '@', '@', '.', '@', '@', '@', '@', '@', '@', '#',
            ],
            vec![
                '#', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@', '#',
            ],
            vec![
                '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#', '#',
            ],
        ];
        let start_pos = Position { x: 3, y: 5 }; // 4,6 in 1-based
        let moves = "UURUURLRLUUDDURDURRR".chars().collect();

        let (final_pos, houses_visited) = simulate_santa_movement(h, w, start_pos, &grid, &moves);
        assert_eq!(final_pos.x + 1, 3);
        assert_eq!(final_pos.y + 1, 11);
        assert_eq!(houses_visited, 11);
    }
}
