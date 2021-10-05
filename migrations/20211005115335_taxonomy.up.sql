CREATE TABLE taxonomy(
  id SERIAL PRIMARY KEY,
  taxonomy_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  name VARCHAR(50) NOT NULL,
  site_id UUID NOT NULL, 
  site_name VARCHAR(50) NOT NULL
);

CREATE UNIQUE INDEX name_site_uniq_idx ON taxonomy(name, site_name);

CREATE TABLE taxonomy_item(
  id SERIAL PRIMARY KEY, 
  taxonomy_item_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  taxonomy_id UUID NOT NULL,
  item_name VARCHAR(20) NOT NULL,
  item_type CHAR(1) NOT NULL,
  ordinal INTEGER NOT NULL,
  CONSTRAINT taxonomy_id_fk FOREIGN KEY (taxonomy_id) REFERENCES taxonomy(taxonomy_id) ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE UNIQUE INDEX name_taxonomy_id_name_uniq_idx ON taxonomy_item(item_name, taxonomy_id);