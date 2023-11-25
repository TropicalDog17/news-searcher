#!/bin/bash
docker run -d --name postgres-14 -p 5432:5432 -e POSTGRES_PASSWORD=tropical postgres:14
# run rust server
cargo run
