use std::io::Write;

use actix_multipart::Multipart;
use actix_web::{post, web, App, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use log::*;

#[post("/file")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  while let Ok(Some(mut field)) = payload.try_next().await {
    debug!("Field {:?}", field);
    let content_type = field.content_disposition().unwrap();
    let filename = content_type.get_filename().unwrap();

    let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

    // File::create is blocking operation, use threadpool
    let mut f = web::block(|| std::fs::File::create(filepath))
      .await
      .unwrap();

    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
      let data = chunk.unwrap();
      // filesystem operations are blocking, we have to use threadpool
      f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
  }
  Ok(HttpResponse::Ok().into())
}
