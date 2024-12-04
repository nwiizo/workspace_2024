use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AddressResponse {
    pub postal_code: String,
    pub prefecture: String,
    pub prefecture_kana: String,
    pub prefecture_code: i32,
    pub city: String,
    pub city_kana: String,
    pub town: String,
    pub town_kana: String,
    pub street: Option<String>,
    pub office_name: Option<String>,
    pub office_name_kana: Option<String>,
}

impl From<jpostcode_rs::Address> for AddressResponse {
    fn from(addr: jpostcode_rs::Address) -> Self {
        AddressResponse {
            postal_code: addr.postcode,
            prefecture: addr.prefecture,
            prefecture_kana: addr.prefecture_kana,
            prefecture_code: addr.prefecture_code,
            city: addr.city,
            city_kana: addr.city_kana,
            town: addr.town,
            town_kana: addr.town_kana,
            street: addr.street,
            office_name: addr.office_name,
            office_name_kana: addr.office_name_kana,
        }
    }
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct AddressQuery {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}
