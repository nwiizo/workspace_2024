use proconio::input;

fn can_divide_into_equal_groups(a: i64, b: i64, c: i64) -> bool {
    // 全ての可能な分け方をチェック

    // ケース1: [a], [b,c]
    if a == b + c {
        return true;
    }

    // ケース2: [b], [a,c]
    if b == a + c {
        return true;
    }

    // ケース3: [c], [a,b]
    if c == a + b {
        return true;
    }

    // ケース4: [a,b], [c]
    if a + b == c {
        return true;
    }

    // ケース5: [a,c], [b]
    if a + c == b {
        return true;
    }

    // ケース6: [b,c], [a]
    if b + c == a {
        return true;
    }

    // ケース7: [a], [b], [c] (全て同じ値の場合)
    if a == b && b == c {
        return true;
    }

    false
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }

    if can_divide_into_equal_groups(a, b, c) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(can_divide_into_equal_groups(3, 3, 3), true); // 全て同じ値
        assert_eq!(can_divide_into_equal_groups(2, 2, 4), true); // 2+2=4
        assert_eq!(can_divide_into_equal_groups(1, 2, 3), false); // 分割不可能
        assert_eq!(can_divide_into_equal_groups(4, 2, 2), true); // 4=2+2
        assert_eq!(can_divide_into_equal_groups(2, 4, 2), true); // 4=2+2
    }
}
