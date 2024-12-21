use proconio::input;

fn find_max_buildings(n: usize, heights: &[i32]) -> usize {
    let mut max_count = 1;

    // 開始位置を全探索
    for start in 0..n {
        let height = heights[start];

        // 開始位置から等差数列の公差を全探索
        let mut diff_start = start + 1;
        while diff_start < n {
            // 最初の等差数列の項を見つける
            while diff_start < n && heights[diff_start] != height {
                diff_start += 1;
            }
            if diff_start >= n {
                break;
            }

            // 公差を計算
            let diff = diff_start - start;
            let mut count = 2; // 開始位置とdiff_startの2つ
            let mut current = diff_start + diff;

            // この公差で等差数列を伸ばせるだけ伸ばす
            while current < n && heights[current] == height {
                count += 1;
                current += diff;
            }

            max_count = max_count.max(count);
            diff_start += 1;
        }
    }

    max_count
}

fn main() {
    input! {
        n: usize,
        heights: [i32; n],
    }

    let result = find_max_buildings(n, &heights);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_1() {
        let heights = vec![5, 7, 5, 7, 7, 5, 7, 7];
        assert_eq!(find_max_buildings(8, &heights), 3);
    }

    #[test]
    fn test_sample_2() {
        let heights = vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000];
        assert_eq!(find_max_buildings(10, &heights), 1);
    }

    #[test]
    fn test_sample_3() {
        let heights = vec![
            3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4, 6, 2, 6, 4, 3, 3, 8, 3, 2,
            7, 9, 5,
        ];
        assert_eq!(find_max_buildings(32, &heights), 3);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(find_max_buildings(1, &[1]), 1);
        assert_eq!(find_max_buildings(5, &[3, 3, 3, 3, 3]), 5);
        assert_eq!(find_max_buildings(3, &[1, 2, 1]), 2);
    }
}
