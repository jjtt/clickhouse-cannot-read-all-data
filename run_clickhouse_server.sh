#!/bin/sh

docker run -it \
  --rm \
  --publish 8124:8123 \
  --publish 9001:9000 \
  --volume $(pwd)/create_table.sql:/docker-entrypoint-initdb.d/01_create_table.sql \
  clickhouse/clickhouse-server
