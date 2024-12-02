use jpostcode::{lookup_address, lookup_addresses, search_by_address};

fn main() {
    // 郵便番号から検索
    if let Ok(addr) = lookup_address("0280052") {
        println!("基本フォーマット: {}", addr.formatted());
        println!("\nカナ付きフォーマット:\n{}", addr.formatted_with_kana());
    }

    // 住所から検索
    println!("\n渋谷を含む住所:");
    for addr in search_by_address("渋谷") {
        println!("{}", addr.formatted());
    }

    // 地域検索
    if let Ok(addresses) = lookup_addresses("150") {
        println!("\n150で始まる地域:");
        for addr in addresses {
            println!("{}", addr.formatted());
        }
    }
}
