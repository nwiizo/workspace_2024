#![allow(
    dead_code,
    non_snake_case,
    unused_imports,
    unused_mut,
    unused_variables,
    while_true,
    unused_assignments,
    clippy::needless_range_loop,
    clippy::ptr_arg,
    clippy::type_complexity,
    clippy::unnecessary_cast
)]
use proconio::{input, marker::Usize1 as usize1, marker::Usize1 as us1};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
//998244353
use ac_library::convolution;
use ac_library::ModInt998244353 as Mint;

//use rec_macro::rec;
//use accum::{accum, accum_by, accum_by_add};
//use segment_tree::segment_tree_new_by;
//use segment_tree_by_min::segment_tree_new_by_min;
//use segment_tree_by_max::segment_tree_new_by_max;
//use segment_tree_by_add::segment_tree_new_by_add;

fn main() {
    input! {
      N: usize,
      K: i32,
    };
    if N == 1 {
        if K == 0 {
            // 1
            println!("1");
        } else {
            // 0
            println!("0");
        }
        return;
    }
    let B = N + 1;
    let mut vs = vec![];
    for t in 2..=N {
        let t = Mint::from(t);
        vs.push(vec![Mint::from(1) / t, (t - 2) / t, Mint::from(1) / t]);
    }
    while vs.len() > 1 {
        let mut vs2 = vec![];
        let mut i = 0;
        while i < vs.len() {
            if i + 1 < vs.len() {
                let c = convolution(&vs[i], &vs[i + 1]);
                vs2.push(c);
                i += 2;
            } else {
                vs2.push(vs[i].clone());
                i += 1;
            }
        }
        vs = vs2;
    }
    let mut pp = Mint::from(1);
    for i in 2..=N {
        pp *= Mint::from(i);
    }
    let v = &vs[0];
    let ans = v[(N as i32 - 1 + K) as usize];
    println!("{}", ans * pp);
}
