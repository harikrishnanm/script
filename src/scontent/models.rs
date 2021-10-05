use serde::{Serialize, Deserialize}

#[derive(Serialize, Deserialize)]
pub struct SContent {
  pub name: String,
  pub taxonomy: String,
}

