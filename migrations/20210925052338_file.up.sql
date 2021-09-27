-- Add up migration script here
CREATE TABLE file (
  id SERIAL PRIMARY KEY,
  file_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  name VARCHAR(255) NOT NULL,
  original_name VARCHAR(255) NOT NULL,
  cache_control VARCHAR(200) NOT NULL DEFAULT 'max-age=0, no-store, must-revalidate',
  tags VARCHAR(50) [] NOT NULL,
  size INTEGER NOT NULL, 
  mime_type VARCHAR(50) NOT NULL,
  folder VARCHAR(25) NOT NULL DEFAULT 'root',
  parent_folder VARCHAR(200),
  site_name VARCHAR(50) NOT NULL,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT site_name_fk FOREIGN KEY (site_name) REFERENCES site(name) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX filename_folder_site_uniq_idx ON file(name, folder, site_name);
