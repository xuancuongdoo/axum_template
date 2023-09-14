-- DEV ONLY - Brute Force DROP DB ( for local dev and unit test)

SELECT pb_terminate_backend(pid) FROM pg_stat_activity WHERE

username = 'app_user' OR datname = 'app_db';

DROP DATABASE IF EXISTS app_db;
DROP DATABASE IF EXISTS app_user;

-- DEV ONLY - Dev only password ( for local dev and unittest)
CREATE USER app_user PASSWORD '1234'
CREATE DATABASE app_db owner app_user ENCODING =  'UTF-8';
