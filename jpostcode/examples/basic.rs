use jpostcode::{lookup_address, lookup_addresses};

fn main() {
    // 単一の住所検索
    match lookup_address("1008939") {
        Ok(addr) => println!(
            "〒{}\n{}{}{}\n{}{}{}",
            addr.postcode,
            addr.prefecture,
            addr.city,
            addr.town,
            addr.prefecture_kana,
            addr.city_kana,
            addr.town_kana
        ),
        Err(e) => eprintln!("エラー: {}", e),
    }

    // 前方一致による複数住所検索
    match lookup_addresses("100") {
        Ok(addresses) => {
            println!("\n100から始まる郵便番号の住所一覧:");
            for addr in addresses {
                println!(
                    "〒{}: {}{}{}",
                    addr.postcode, addr.prefecture, addr.city, addr.town
                );
            }
        }
        Err(e) => eprintln!("エラー: {}", e),
    }
}
