#!/bin/bash

export SERVICE_DB_URL="postgres://postgres:postgres@localhost:5432/db";
export SERVICE_WEB_FOLDER="web";
cargo watch -q -c -x "test $1 -- --nocapture"
