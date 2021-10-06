CREATE TABLE content (
  id SERIAL PRIMARY KEY,
  content_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  name VARCHAR(50) NOT NULL,
  tags VARCHAR(20)[] NOT NULL,
  site_id UUID NOT NULL,
  site_name VARCHAR(50) NOT NULL,
  collection_id UUID NOT NULL,
  collection_name VARCHAR(50) NOT NULL,
  content_item_id UUID NOT NULL,
  raw BOOLEAN NOT NULL DEFAULT true,
  taxonomy_id UUID,
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
  content UUID NOT NULL,
  raw BOOLEAN NOT NULL DEFAULT true,
  taxonomy_id UUID,
  content_length INTEGER NOT NULL,
  cache_control VARCHAR(200) NOT NULL DEFAULT 'max-age=0, no-store, must-revalidate',
  version INTEGER NOT NULL DEFAULT 0,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT collection_id_fk FOREIGN KEY (collection_id) REFERENCES collection(collection_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT site_id_fk FOREIGN KEY (site_id) REFERENCES site(site_id) ON DELETE CASCADE ON UPDATE CASCADE
);


CREATE TABLE content_item_raw (
  id SERIAL PRIMARY KEY,
  content_item_raw_id UUID NOT NULL,
  content TEXT NOT NULL
);

CREATE TABLE content_set (
  id SERIAL PRIMARY KEY,
  content_set_id UUID NOT NULL,
  taxonomy_id UUID NOT NULL,
  content_id UUID NOT NULL,
  ordinal INTEGER NOT NULL DEFAULT 0,
  version INTEGER NOT NULL DEFAULT 0,
  parent UUID,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT taxonomy_id_fk FOREIGN KEY (taxonomy_id) REFERENCES taxonomy(taxonomy_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT content_id_fk FOREIGN KEY (content_id) REFERENCES content(content_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE content_item_text (
  id SERIAL PRIMARY KEY,
  content_item_text_id UUID NOT NULL,
  content_set_id UUID NOT NULL,
  content TEXT NOT NULL,
  ordinal INTEGER NOT NULL
);

CREATE TABLE content_item_number (
  id SERIAL PRIMARY KEY,
  content_item_number_id UUID NOT NULL,
  content_set_id UUID NOT NULL,
  content NUMERIC NOT NULL,
  ordinal INTEGER NOT NULL
);
CREATE TABLE content_item_bool (
  id SERIAL PRIMARY KEY,
  content_item_raw_id UUID NOT NULL,
  content_set_id UUID NOT NULL,
  content BOOLEAN NOT NULL,
  ordinal INTEGER NOT NULL
);
