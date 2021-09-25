CREATE TABLE collection (
  id SERIAL PRIMARY KEY,
  collection_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  name VARCHAR(50) NOT NULL,
  parent_id UUID,
  site_id Uuid NOT NULL,
  site_name VARCHAR(50) NOT NULL,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT parent_fk FOREIGN KEY (parent_id) REFERENCES collection(collection_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT site_id_fk FOREIGN KEY (site_id) REFERENCES site(site_id) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT site_name_fk FOREIGN KEY (site_name) REFERENCES site(name) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX site_collection_uniq_idx ON collection(name, site_id);
