
CREATE TABLE rbac (
  rbac_id SERIAL PRIMARY KEY,
  path VARCHAR(100) NOT NULL,
  path_match VARCHAR(15) NOT NULL,
  method VARCHAR(20) NOT NULL,
  rbac_role VARCHAR(20) NOT NULL,
  rbac_user VARCHAR(50) NOT NULL,
  description VARCHAR(120),
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp
);
CREATE UNIQUE INDEX rbac_uniq_idx ON rbac(path, path_match, method, rbac_role, rbac_user);


INSERT INTO rbac (path, path_match, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('/admin', 'STARTSWITH','*', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');