pub mod file;
pub mod models;

use crate::common::utils;
use crate::file::models::File;
use crate::rbac::models::Identity;
use crate::AppData;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use std::fs;
use std::io::Write;
//use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{get, post, web, web::Path, Error, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use log::*;

use crate::error::ScriptError;
use uuid::Uuid;

use crate::constants::*;

#[get("/site/{site_name}/file/{file:.*}")]
async fn get_file(
    _data: web::Data<AppData>,
    Path((site_name, file)): Path<(String, String)>,
) -> Result<NamedFile, ScriptError> {
    debug!("Request file {}", file);
    let root_path = utils::get_root_path();
    let full_file_name = format!("{}/{}/{}", root_path, site_name, file);
    debug!("Opening file {}", full_file_name);
    match NamedFile::open(full_file_name) {
        Ok(file) => {
            if file.metadata().unwrap().is_dir() {
                return Err(ScriptError::BadRequest(
                    "Requested resource is not a file".to_string(),
                ));
            }
            debug!("File found {:?}", file);
            Ok(file.use_last_modified(true).use_etag(true))
        }
        Err(e) => {
            error!("Error getting file {}", e);
            Err(ScriptError::FileNotFound)
        }
    }

    //TODO Content dispositon
}
#[get("/site/{site_name}/folder/{folder:.*}")]
async fn list(
    data: web::Data<AppData>,
    Path((site_name, folder)): Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    match sqlx::query_as!(
    File,
    "SELECT file_id, name, original_name, cache_control, size, tags, path, mime_type, site_name, created_by 
      FROM file WHERE site_name = $1 and path = $2",
    site_name,
    folder
  )
  .fetch_all(&data.db_pool)
  .await
  {
    Ok(files) => Ok(HttpResponse::Ok().json(files)),
    Err(e) => {
      error!("Error getting file list {}", e);
      Ok(HttpResponse::InternalServerError().finish())
    }
  }
}

#[post("/site/{site_name}/file/{folder:.*}")]
async fn upload(
    mut payload: Multipart,
    identity: web::ReqData<Identity>,
    data: web::Data<AppData>,
    Path((site_name, folder)): Path<(String, String)>,
) -> Result<HttpResponse, ScriptError> {
    let mut files: Vec<File> = Vec::new();

    let root_path = utils::get_root_path();
    let folder_name: &str = &format!("{}/{}/{}", root_path, site_name, folder);
    debug!("Path {}", folder_name);
    //check if folder exixts

    if let Ok(metadata) = fs::metadata(folder_name) {
        if !metadata.is_dir() {
            error!("Path is not a directory");
            return Err(ScriptError::FileNotFound);
        }
    } else {
        error!("Path does not exist");
        return Err(ScriptError::FileNotFound);
    }

    while let Ok(Some(mut field)) = payload.try_next().await {
        let mut tx = match data.db_pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                error!("Could not start transaction {}", e);
                return Ok(HttpResponse::InternalServerError().body(e.to_string()));
            }
        };
        debug!("File upload {:?}", field);
        let content_type = field.content_disposition().unwrap();
        debug!("Headers {:?}", field.headers());
        let filename = content_type.get_filename().unwrap();
        let sanitized_filename = sanitize_filename::sanitize(&filename);
        //let filepath = format!("{}/{}", folder_name, sanitized_filename);

        let mime_type = match field.headers().get("content-type") {
            Some(mime_type) => mime_type.to_str().unwrap(),
            None => "application/octet-stream",
        };
        debug!("Mime type {}", mime_type);

        let tags = vec![sanitized_filename.clone(), site_name.clone()];

        let mut new_file = File {
            file_id: Uuid::new_v4(),
            name: sanitized_filename.to_string(),
            original_name: filename.to_string(),
            cache_control: CACHE_CONTROL_DEFAULT.to_string(),
            path: folder.clone(),
            size: 0,
            tags: tags,
            mime_type: mime_type.to_string(),
            site_name: site_name.clone(),
            created_by: identity.clone().into_inner().user,
        };

        match new_file.save(identity.clone().into_inner(), &mut tx).await {
            Ok(saved_file) => {
                // File::create is blocking operation, use threadpool
                let full_path = format!("{}/{}", folder_name, sanitized_filename);
                let mut f = web::block(|| std::fs::File::create(full_path))
                    .await
                    .unwrap();

                // Field in turn is stream of *Bytes* object
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    // filesystem operations are blocking, we have to use threadpool
                    f = match web::block(move || f.write_all(&data).map(|_| f)).await {
                        Ok(f) => f,
                        Err(e) => {
                            error!("Error saving file {}", e);
                            return Err(ScriptError::FileCreationError);
                        }
                    }
                }
                let size = f.metadata().unwrap().len();
                debug!("Filesize {}", size);
                new_file.size = size as i32;
                match new_file
                    .update_size(identity.clone().into_inner(), &mut tx)
                    .await
                {
                    Ok(updated_file) => files.push(updated_file.to_owned()),
                    Err(e) => {
                        error!("Unable to compute file size {}", e);
                        files.push(saved_file.to_owned());
                    }
                };
                let _ = tx.commit().await;
            }
            Err(e) => {
                error!("Error saving file {:?}  Error: {}", new_file, e);
                let _ = tx.rollback().await;
                return Err(ScriptError::FileCreationError);
            }
        };
    }
    Ok(HttpResponse::Ok().json(files))
}
