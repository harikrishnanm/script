-- Add up migration script here
CREATE TABLE coll (
  coll_id SERIAL PRIMARY KEY,
  site_id INTEGER NOT NULL,
  coll_name VARCHAR(50) NOT NULL,
  slug VARCHAR(25),
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT fk_site
    FOREIGN KEY (site_id)
      REFERENCES site(site_id)
)