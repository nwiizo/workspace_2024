use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
    is_active: bool,
}

fn main() {
    let json_str = r#"
    {
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com",
        "is_active": true
    }
    "#;

    let user: User = serde_json::from_str(json_str).unwrap();
    println!("Deserialized user: {:?}", user);
}
