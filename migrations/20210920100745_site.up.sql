CREATE TABLE site (
  id SERIAL PRIMARY KEY,
  site_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  name VARCHAR(50) UNIQUE NOT NULL,
  path VARCHAR(50) NOT NULL,
  slug VARCHAR(20),
  url VARCHAR(100),
  cors_enabled BOOLEAN DEFAULT false,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT if_cors_then_url_is_not_null 
    CHECK ( (NOT cors_enabled) OR (url IS NOT NULL) ) 
);


CREATE UNIQUE INDEX site_path_uniq_idx ON site(path, url);
CREATE UNIQUE INDEX site_slug_uniq_idx ON site(slug, url);