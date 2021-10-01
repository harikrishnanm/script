CREATE TABLE content (
  id SERIAL PRIMARY KEY,
  content_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  name VARCHAR(50) NOT NULL,
  mime_type VARCHAR(50),
  tags VARCHAR(20)[] NOT NULL,
  site_id UUID NOT NULL,
  site_name VARCHAR(50) NOT NULL,
  collection_id UUID NOT NULL,
  collection_name VARCHAR(50) NOT NULL,
  content TEXT NOT NULL,
  content_length INTEGER NOT NULL,
  cache_control VARCHAR(200) NOT NULL DEFAULT 'max-age=0, no-store, must-revalidate',
  version INTEGER NOT NULL DEFAULT 0,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT collection_id_fk FOREIGN KEY (collection_id) REFERENCES collection(collection_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT site_id_fk FOREIGN KEY (site_id) REFERENCES site(site_id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE UNIQUE INDEX name_collection_site_uniq_idx ON content(name, site_id, collection_id);

CREATE TABLE content_archive (
  id SERIAL PRIMARY KEY,
  content_id UUID NOT NULL DEFAULT uuid_generate_v4 (),
  name VARCHAR(50) NOT NULL,
  mime_type VARCHAR(50),
  tags VARCHAR(20)[] NOT NULL,
  site_id UUID NOT NULL,
  site_name VARCHAR(50) NOT NULL,
  collection_id UUID NOT NULL,
  collection_name VARCHAR(50) NOT NULL,
  content TEXT NOT NULL,
  content_length INTEGER NOT NULL,
  cache_control VARCHAR(200) NOT NULL DEFAULT 'max-age=0, no-store, must-revalidate',
  version INTEGER NOT NULL DEFAULT 0,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT collection_id_fk FOREIGN KEY (collection_id) REFERENCES collection(collection_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT site_id_fk FOREIGN KEY (site_id) REFERENCES site(site_id) ON DELETE CASCADE ON UPDATE CASCADE
);


