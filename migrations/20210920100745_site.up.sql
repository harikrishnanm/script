CREATE TABLE site (
  id SERIAL PRIMARY KEY,
  site_id UUID NOT NULL UNIQUE DEFAULT uuid_generate_v4 (),
  name VARCHAR(50) UNIQUE NOT NULL,
  url VARCHAR(100),
  cors_enabled BOOLEAN DEFAULT false,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT if_cors_then_url_is_not_null 
    CHECK ( (NOT cors_enabled) OR (url IS NOT NULL) ) 
);

CREATE TYPE authn AS ENUM ('R', 'E', 'O');

CREATE TABLE site_user (
  id SERIAL PRIMARY KEY,
  site_user_id UUID NOT NULL DEFAULT uuid_generate_v4 (),
  site_id UUID NOT NULL,
  site_user VARCHAR(50) NOT NULL,
  authn authn,
  CONSTRAINT fk_site_owner
    FOREIGN KEY (site_id)
      REFERENCES site(site_id)
);
CREATE UNIQUE INDEX site_user_uniq_index ON site_user(site_id, site_user);

