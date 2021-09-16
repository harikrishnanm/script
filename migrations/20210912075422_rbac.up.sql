-- Add up migration script here
CREATE TABLE rbac (
  rbac_id SERIAL PRIMARY KEY,
  path_regex VARCHAR(100) NOT NULL,
  method VARCHAR(20) NOT NULL,
  rbac_role VARCHAR(20) NOT NULL,
  rbac_user VARCHAR(50) NOT NULL,
  description VARCHAR(120),
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  modified TIMESTAMP NOT NULL DEFAULT current_timestamp
);
CREATE INDEX user_role_idx ON rbac (rbac_user, rbac_role);
CREATE INDEX path_method_idx ON rbac (path_regex, method);

CREATE TABLE rbac_audit_log (
  rbac_id SERIAL PRIMARY KEY,
  path_regex VARCHAR(100) NOT NULL,
  method VARCHAR(20) NOT NULL,
  rbac_role VARCHAR(20) NOT NULL,
  rbac_user VARCHAR(50) NOT NULL,
  description VARCHAR(120),
  created_by VARCHAR(50) NOT NULL,
  created TIMESTAMP NOT NULL DEFAULT current_timestamp,
  last_modified_by VARCHAR(50) NOT NULL,
  version INTEGER NOT NULL
);

INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', '*', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');
INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/site/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', 'GET', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');
INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/site/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', 'POST', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');
INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/site/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', 'PUT', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');
INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/site/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', 'PATCH', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');
INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/site/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', 'OPTIONS', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');
INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/site/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', 'DELETE', 'CMS ADMIN', 'cmsadmin', 'Allow admin to GET all /admin/* routes', 'Yoda');
INSERT INTO rbac (path_regex, method, rbac_role, rbac_user, description, created_by) 
  VALUES ('^/site/admin{1}\b[[/]{1}[-a-zA-Z0-9@:%_+.~#?&/=]{1,}[/]{0,1}]{0,}\b$', '*', 'CMS ADMIN', 'cmsuser1', 'Allow admin to GET all /admin/* routes', 'Yoda');