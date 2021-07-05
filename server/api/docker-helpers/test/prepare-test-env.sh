#!/bin/sh
# wait-for-postgres.sh

export DATABASE_URL=postgres://postgres:postgres@postgres:5432/codemountain_test

until psql ${DATABASE_URL} -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
  sleep 1
done
  
>&2 echo "Postgres is up - executing command"

smtp-sink -u postfix 2525 1000&