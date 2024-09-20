#!/bin/bash
set -e

echo "Mongo DB: initialising mongo"
mongosh "mongodb://${MONGO_INITDB_ROOT_USERNAME}:${MONGO_INITDB_ROOT_PASSWORD}@localhost:27017" <<EOF
use ${MONGO_SHORT_URLS_DB}
db.createUser(
  {
     user: "${MONGO_SHORT_URLS_DB_USERNAME}",
     pwd: "${MONGO_SHORT_URLS_DB_PASSWORD}",
     roles: [ "readWrite", "dbAdmin" ]
  }
)

db.createCollection('short_urls')
db.short_urls.createIndex({key: 1},{unique: true})
db.short_urls.createIndex({long_url: 1},{unique: true})

EOF

echo "Mongo DB user created!!!"