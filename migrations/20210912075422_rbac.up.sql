-- Add up migration script here
CREATE TABLE rbac (
  rbac_id SERIAL PRIMARY KEY,
  path VARCHAR(100) NOT NULL,
  path_match VARCHAR(20) NOT NULL DEFAULT 'EXACT',
  method VARCHAR(20) NOT NULL,
  rbac_role VARCHAR(20) NOT NULL,
  rbac_user VARCHAR(50) NOT NULL,
  description VARCHAR(120),
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp
);
CREATE INDEX user_role_idx ON rbac (rbac_user, rbac_role);
CREATE INDEX path_method_idx ON rbac (path, method);

CREATE TABLE rbac_audit_log (
  rbac_id SERIAL PRIMARY KEY,
  path VARCHAR(100) NOT NULL,
  method VARCHAR(20) NOT NULL,
  rbac_role VARCHAR(20) NOT NULL,
  rbac_user VARCHAR(50) NOT NULL,
  description VARCHAR(120),
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  last_modified_by VARCHAR(50) NOT NULL,
  version INTEGER NOT NULL
);

INSERT INTO rbac (path, path_match, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('(^\/admin\/site$){1}', 'STARTSWITH', 'GET', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin routes', 'Yoda');
INSERT INTO rbac (path, path_match, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('(^\/admin\/site$){1}', 'STARTSWITH', 'POST', 'CMS ADMIN', 'cmsadmin', 'Allow admin to POST all /admin routes', 'Yoda');
INSERT INTO rbac (path, path_match, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('(^\/admin\/site$){1}', 'STARTSWITH', 'PUT', 'CMS ADMIN', 'cmsadmin', 'Allow admin to PUT all /admin routes', 'Yoda');
INSERT INTO rbac (path, path_match, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('(^\/admin\/site$){1}', 'STARTSWITH', 'DELETE', 'CMS ADMIN', 'cmsadmin', 'Allow admin to DELETE all /admin routes', 'Yoda');
INSERT INTO rbac (path, path_match, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('/public', 'STARTSWITH', 'GET', 'ANON', 'anonymous', 'Allow all to GET all /public routes', 'Yoda');