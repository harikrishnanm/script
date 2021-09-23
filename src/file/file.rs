use actix_multipart::Multipart;
use actix_web::web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct File {
  name: String,
  time: u64,
  err: String,
}

#[derive(Deserialize)]
struct Download {
  name: String,
}

pub async fn upload() -> impl HttpResponse {
  let mut filename = "".to_string();
  while let Ok(Some(mut field)) = payload.try_next().await {
    let content_type = field.content_disposition().unwrap();
    filename = format!(
      "{} - {}",
      SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros(),
      content_type.get_filename().unwrap(),
    );
    let filepath = format!("{}/{}", UPLOAD_PATH, sanitize_filename::sanitize(&filename));
    // File::create is blocking operation, use thread pool
    let mut f = web::block(|| std::fs::File::create(filepath))
      .await
      .unwrap();
    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
      let data = chunk.unwrap();
      // filesystem operations are blocking, we have to use thread pool
      f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
  }
  // Create a unique name for the file
  let res = &File {
    name: filename,
    time: SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs(),
    err: "".to_string(),
  };
  Ok(HttpResponse::Ok().json(res))
}
async fn download(info: web::Path<(String)>) -> HttpResponse {
  // Body of the function goes here!)
}
