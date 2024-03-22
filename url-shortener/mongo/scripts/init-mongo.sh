#!/bin/bash
set -e

mongo --host "mongodb://${MONGO_INITDB_ROOT_USERNAME}:${MONGO_INITDB_ROOT_PASSWORD}@localhost:27017" <<EOF
use ${MONGO_SHORT_URLS_DB}
db.createUser(
  {
     user: "${MONGO_SHORT_URLS_DB_USERNAME}",
     pwd: "${MONGO_SHORT_URLS_DB_PASSWORD}",
     roles: [ "readWrite", "dbAdmin" ]
  }
)

db.createCollection('shortUrls')
db.shortUrls.createIndex({key: 1},{unique: true})
db.shortUrls.createIndex({full: 1},{unique: true})

EOF