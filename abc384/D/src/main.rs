use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        a: [i64; n],
    }

    // 2周期分の配列を作成
    let mut v = vec![0i64; 2 * n];
    for i in 0..n {
        v[i] = a[i];
        v[i + n] = a[i];
    }

    let mut found = false;
    let mut sum = 0;
    let mut right = 0;

    for left in 0..n {
        // leftを進める時は前の値を引く
        if left > 0 {
            sum -= v[left - 1];
            // sumが小さすぎる場合はrightを進める
            while sum < s && right < 2 * n && right >= left {
                sum += v[right];
                right += 1;
            }
        }

        // rightを進めながら合計を更新
        while sum < s && right < 2 * n {
            sum += v[right];
            right += 1;
        }

        // 目標値と一致したら終了
        if sum == s {
            found = true;
            break;
        }

        // 目標値を超えた場はrightを戻す
        while sum > s && right > left {
            right -= 1;
            sum -= v[right];
            if sum == s {
                found = true;
                break;
            }
        }
    }

    println!("{}", if found { "Yes" } else { "No" });
}
