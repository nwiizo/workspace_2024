use crate::models::Target;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub targets: Vec<Target>,
}
