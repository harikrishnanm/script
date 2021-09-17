pub mod boolean;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtOption {
  key: String,
  value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Field<T> {
  name: String,
  label: String,
  default: Option<T>,
  info: String,
  localize: bool,
  value: T,
  options: Option<Vec<ExtOption>>,
}
