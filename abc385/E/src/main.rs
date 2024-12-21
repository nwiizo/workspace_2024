use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut graph = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            u: Usize1,
            v: Usize1,
        }

        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = 0;
    for v in 0..n {
        let ord = (0..graph[v].len())
            .sorted_by_key(|&i| Reverse(graph[graph[v][i]].len()))
            .collect_vec();

        for x in 1..=graph[v].len() {
            let y = graph[graph[v][ord[x - 1]]].len() - 1;
            ans = ans.max(x * y + x + 1);
        }
    }

    println!("{}", n - ans);
}
