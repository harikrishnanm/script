use crate::error::ScriptError;
use crate::rbac::models::Identity;

use actix_web::{get, post, web, web::Path, HttpResponse};

use log::*;

use serde_json::{json, Map, Value};
use std::ops::Deref;

/*#[derive(Debug, Deserialize, Clone)]
pub struct SContent {
  ,
}*/

#[post("/site/{site_name}/collection/{collection_name}/scontent")]
pub async fn save(
    _identity: web::ReqData<Identity>,
    sc_data: web::Json<Vec<Map<String, Value>>>,
    Path((_site_name, _collection_name)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
    debug!("{:?}", sc_data);
    let data_vals = sc_data.deref().into_iter();

    for data_val in data_vals {
        let keys = data_val.keys();
        for key in keys {
            debug!("Found key {}", key);
            match data_val.get(key).unwrap() {
                Value::Number(v) => debug!("Number {}", v),
                Value::String(v) => debug!("String {}", v),
                _ => debug!("Unknown"),
            }
        }
    }

    /*let keys = sc_data.get(0).keys();

    for key in keys {
      debug!("Found key {}", key);
    }*/

    /*match &sc_data.value {
      Value::Number(v) => debug!("Number {}", v),
      Value::String(v) => debug!("String {}", v),
      _ => debug!("Unknown"),
    };*/
    Ok(HttpResponse::Ok().finish())
}

#[get("/site/{site_name}/collection/{collection_name}/scontent/{scontent_name}")]
pub async fn get() -> Result<HttpResponse, ScriptError> {
    let mut m1 = Map::new();

    let _number = m1.insert("Hello".to_string(), json!("World"));
    m1.insert("990".to_string(), json!(233));
    Ok(HttpResponse::Ok().json(m1))
}
