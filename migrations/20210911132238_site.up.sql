-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE site (
  site_id SERIAL PRIMARY KEY,
  id UUID NOT NULL,
  name VARCHAR(50) UNIQUE NOT NULL,
  url VARCHAR(100),
  cors_enabled BOOLEAN DEFAULT false,
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp,
  CONSTRAINT if_cors_then_url_is_not_null 
    CHECK ( (NOT cors_enabled) OR (url IS NOT NULL) ) 
)