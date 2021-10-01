-- Add up migration script here
CREATE TABLE asset (
    id SERIAL PRIMARY KEY,
    asset_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
    name VARCHAR(50) NOT NULL,
    file_id UUID NOT NULL,
    coll_id UUID NOT NULL,
    coll_name VARCHAR(50) NOT NULL,
    site_id UUID NOT NULL,
    site_name VARCHAR(50) NOT NULL,
    CONSTRAINT file_id_fk FOREIGN KEY (file_id) REFERENCES file(file_id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT coll_id_fk FOREIGN KEY (coll_id) REFERENCES collection(collection_id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT site_id_fk FOREIGN KEY (site_id) REFERENCES site(site_id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE UNIQUE INDEX asset_uniq_idx ON asset(file_id, coll_id, name);
