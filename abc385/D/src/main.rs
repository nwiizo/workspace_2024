use proconio::input;
use std::io::{self, BufWriter, Write};

fn main() {
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    input! {
        (n, m, sx_0, sy_0): (usize, usize, i64, i64),
        xy: [(i64, i64); n],
        dc: [(char, i64); m],
    }
    let mut xy: std::collections::BTreeSet<_> = xy.iter().collect();
    let mut col = std::collections::BTreeMap::new();
    for &(x, y) in &xy {
        let e = col.entry(x).or_insert(std::collections::BTreeSet::new());
        e.insert(y);
    }
    let mut ans = 0;

    let mut sx = sx_0;
    let mut sy = sy_0;

    for &(d, c) in &dc {
        match d {
            'U' => {
                if col.contains_key(&sx) {
                    let v = col.get(&sx).unwrap();
                    let mut r = vec![];
                    for s in v.range(sy..=sy + c) {
                        ans += 1;
                        r.push(*s);
                        xy.remove(&(sx, **s));
                    }
                    let e = col.get_mut(&sx).unwrap();
                    for s in &r {
                        e.remove(*s);
                    }
                }
                sy += c;
            }
            'D' => {
                if col.contains_key(&sx) {
                    let v = col.get(&sx).unwrap();
                    let mut r = vec![];
                    for s in v.range(sy - c..=sy) {
                        ans += 1;
                        r.push(*s);
                        xy.remove(&(sx, **s));
                    }
                    let e = col.get_mut(&sx).unwrap();
                    for s in &r {
                        e.remove(*s);
                    }
                }
                sy -= c;
            }
            'R' => {
                sx += c;
            }
            'L' => {
                sx -= c;
            }
            _ => unreachable!(),
        }
    }

    let mut row = std::collections::BTreeMap::new();
    for &(x, y) in &xy {
        let e = row.entry(y).or_insert(std::collections::BTreeSet::new());
        e.insert(x);
    }
    let mut sx = sx_0;
    let mut sy = sy_0;
    for &(d, c) in &dc {
        match d {
            'U' => {
                sy += c;
            }
            'D' => {
                sy -= c;
            }
            'R' => {
                if row.contains_key(&sy) {
                    let v = row.get(&sy).unwrap();
                    let mut r = vec![];
                    for s in v.range(sx..=sx + c) {
                        ans += 1;
                        r.push(*s);
                    }
                    let e = row.get_mut(&sy).unwrap();
                    for s in &r {
                        e.remove(*s);
                    }
                }
                sx += c;
            }
            'L' => {
                if row.contains_key(&sy) {
                    let v = row.get(&sy).unwrap();
                    let mut r = vec![];
                    for s in v.range(sx - c..=sx) {
                        ans += 1;
                        r.push(*s);
                    }
                    let e = row.get_mut(&sy).unwrap();
                    for s in &r {
                        e.remove(*s);
                    }
                }
                sx -= c;
            }
            _ => unreachable!(),
        }
    }
    writeln!(writer, "{} {} {}", sx, sy, ans).unwrap();
}
