use jpostcode::{lookup_address, lookup_addresses, ToJson};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = lookup_address("0280052")?;
    let addresses = lookup_addresses("113")?;

    let address_json = address.to_json()?;
    println!("Single address:\n{}", address_json);

    let addresses_json = addresses.to_json()?;
    println!("\nMultiple addresses:\n{}", addresses_json);

    Ok(())
}
