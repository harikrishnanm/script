use crate::file::models::*;
use crate::rbac::models::Identity;

use log::*;
use sqlx::Error;

impl File {
  pub async fn save(
    self: &Self,
    identity: Identity,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
  ) -> Result<File, Error> {
    match sqlx::query_as!(
      File,
      "INSERT INTO file (file_id, name, original_name, folder, mime_type, site_name, created_by) 
      VALUES ($1, $2, $3, $4, $5, $6, $7) 
        RETURNING file_id, name, original_name, folder, mime_type, site_name, created_by",
      self.file_id,
      self.name,
      self.original_name,
      self.folder,
      self.mime_type,
      self.site_name,
      identity.user
    )
    .fetch_one(tx)
    .await
    {
      Ok(new_file) => Ok(new_file),
      Err(e) => {
        error!("Error creating file entry {}", e);
        Err(e)
      }
    }
  }
}
