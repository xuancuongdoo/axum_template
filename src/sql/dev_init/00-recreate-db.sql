-- DEV ONLY - Brute Force DROP DB ( for local dev and unit test)

SELECT pb_terminate_backend(pid) FROM pg_stat_activity WHERE

username = 'postgres' OR databasename = 'app_db';

DROP DATABASE IF EXISTS app_db;
DROP DATABASE IF EXISTS postgres;

-- DEV ONLY - Dev only password ( for local dev and unittest)
CREATE USER app_user PASSWORD 'postgres'
CREATE DATABASE app_db owner app_user ENCODING =  'UTF-8';
